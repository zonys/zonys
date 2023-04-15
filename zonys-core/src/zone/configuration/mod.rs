mod directive;
mod error;

pub use directive::*;
pub use error::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_yaml::{from_reader, from_value, to_value, Value};
use std::fs::File;
use std::io::BufReader;
use std::iter::once;
use std::mem::{replace, take};
use std::path::{Path, PathBuf};
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Method)]
pub struct ZoneConfiguration {
    #[Method(all)]
    directive: ZoneConfigurationDirective,
    directives: Vec<ZoneConfigurationDirective>,
    #[Method(all)]
    path: PathBuf,
}

impl ZoneConfiguration {
    pub fn directives(&self) -> ZoneConfigurationDirectives<&Self> {
        ZoneConfigurationDirectives::new(self)
    }

    pub fn directives_mut(&mut self) -> ZoneConfigurationDirectives<&mut Self> {
        ZoneConfigurationDirectives::new(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Constructor)]
pub struct ZoneConfigurationDirectives<T> {
    configuration: T,
}

impl<'a> ZoneConfigurationDirectives<&'a mut ZoneConfiguration> {
    pub fn prepend(
        &mut self,
        base_path: Option<&Path>,
        directive: ZoneConfigurationDirective,
    ) -> Result<(), ResolveZoneConfigurationDirectiveError> {
        let directives = directive.resolve(base_path)?;
        self.configuration.directives = once(directive)
            .chain(directives.into_iter())
            .chain(take(&mut self.configuration.directives).into_iter())
            .collect();

        Ok(())
    }
}

impl<'a> ZoneConfigurationDirectives<&'a ZoneConfiguration> {
    pub fn iter(&self) -> impl Iterator<Item = &ZoneConfigurationDirective> {
        self.configuration.directives.iter()
    }

    pub fn read_first<F, T>(&self, function: F) -> Option<T>
    where
        F: FnMut(&'a ZoneConfigurationDirective) -> Option<T>,
        T: 'a,
    {
        self.configuration
            .directives
            .iter()
            .flat_map(function)
            .next()
    }

    pub fn read_last<F, T>(&self, function: F) -> Option<T>
    where
        F: FnMut(&ZoneConfigurationDirective) -> Option<T>,
    {
        self.configuration
            .directives
            .iter()
            .flat_map(function)
            .last()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ZoneConfigurationProcessor {}

impl ZoneConfigurationProcessor {
    fn merge_value(&self, left: Value, right: Value) -> Result<Value, MergeZoneConfigurationError> {
        match (left, right) {
            (Value::Null, v) => Ok(v),
            (Value::Bool(_), v) => Ok(v),
            (Value::Number(_), v) => Ok(v),
            (Value::String(_), v) => Ok(v),
            (Value::Sequence(s1), Value::Sequence(s2)) => Ok(Value::Sequence(
                s1.into_iter().chain(s2.into_iter()).collect(),
            )),
            (Value::Mapping(mut m1), Value::Mapping(m2)) => {
                for (key, value_right) in m2.into_iter() {
                    match m1.get_mut(&key) {
                        Some(ref mut value_left) => {
                            let value =
                                self.merge_value(replace(value_left, Value::Null), value_right)?;

                            m1.insert(key, value);
                        }
                        None => {
                            m1.insert(key, value_right);
                        }
                    }
                }

                Ok(Value::Mapping(m1))
            }
            (v, Value::Null) => Ok(v),
            (v1, v2) => Err(MergeZoneConfigurationError::IncompatibleValues(v1, v2)),
        }
    }

    fn merge_any<R, S, T>(&self, left: S, right: T) -> Result<R, MergeZoneConfigurationError>
    where
        R: 'static + DeserializeOwned + Serialize,
        S: 'static + DeserializeOwned + Serialize,
        T: 'static + DeserializeOwned + Serialize,
    {
        from_value::<R>(self.merge_value(to_value(left)?, to_value(right)?)?).map_err(|e| e.into())
    }

    fn merge_version1(
        &self,
        mut left: version1::ZoneConfigurationDirective,
        mut right: version1::ZoneConfigurationDirective,
    ) -> Result<version1::ZoneConfigurationDirective, MergeZoneConfigurationError> {
        let r#type = match (
            replace(
                left.type_mut(),
                version1::ZoneConfigurationTypeDirective::default(),
            ),
            replace(
                right.type_mut(),
                version1::ZoneConfigurationTypeDirective::default(),
            ),
        ) {
            (
                version1::ZoneConfigurationTypeDirective::Undefined(left),
                version1::ZoneConfigurationTypeDirective::Undefined(right),
            ) => version1::ZoneConfigurationTypeDirective::Undefined(self.merge_any(left, right)?),
            (
                version1::ZoneConfigurationTypeDirective::Jail(left),
                version1::ZoneConfigurationTypeDirective::Undefined(right),
            ) => version1::ZoneConfigurationTypeDirective::Jail(self.merge_any(left, right)?),
            (
                version1::ZoneConfigurationTypeDirective::Undefined(left),
                version1::ZoneConfigurationTypeDirective::Jail(right),
            ) => version1::ZoneConfigurationTypeDirective::Jail(self.merge_any(left, right)?),
            (
                version1::ZoneConfigurationTypeDirective::Jail(left),
                version1::ZoneConfigurationTypeDirective::Jail(right),
            ) => version1::ZoneConfigurationTypeDirective::Jail(self.merge_any(left, right)?),
        };

        Ok(version1::ZoneConfigurationDirective::new(
            self.merge_any(
                replace(left.from_mut(), None),
                replace(right.from_mut(), None),
            )?,
            self.merge_any(
                replace(left.include_mut(), None),
                replace(right.include_mut(), None),
            )?,
            self.merge_any(
                replace(left.variables_mut(), None),
                replace(right.variables_mut(), None),
            )?,
            self.merge_any(
                replace(left.tags_mut(), None),
                replace(right.tags_mut(), None),
            )?,
            r#type,
            self.merge_any(
                replace(left.start_after_create_mut(), None),
                replace(right.start_after_create_mut(), None),
            )?,
            self.merge_any(
                replace(left.destroy_after_stop_mut(), None),
                replace(right.destroy_after_stop_mut(), None),
            )?,
            self.merge_any(
                replace(left.file_system_mut(), None),
                replace(right.file_system_mut(), None),
            )?,
        ))
    }

    fn merge(
        &self,
        mut left: ZoneConfiguration,
        mut right: ZoneConfiguration,
    ) -> Result<ZoneConfiguration, MergeZoneConfigurationError> {
        let (mut left_directive, left_path) = (
            replace(left.directive_mut(), ZoneConfigurationDirective::default()),
            replace(left.path_mut(), PathBuf::default()),
        );
        let mut right_directive =
            replace(right.directive_mut(), ZoneConfigurationDirective::default());

        let mut left_version = replace(
            left_directive.version_mut(),
            ZoneConfigurationVersionDirective::default(),
        );
        let right_version = replace(
            right_directive.version_mut(),
            ZoneConfigurationVersionDirective::default(),
        );

        match (left_version, right_version) {
            (
                ZoneConfigurationVersionDirective::Version1(left),
                ZoneConfigurationVersionDirective::Version1(right),
            ) => {
                left_version =
                    ZoneConfigurationVersionDirective::Version1(self.merge_version1(left, right)?)
            }
        };

        Ok(ZoneConfiguration::new(
            ZoneConfigurationDirective::new(left_version),
            Vec::default(),
            left_path,
        ))
    }
}

impl ZoneConfigurationProcessor {
    pub fn process(
        &self,
        mut configuration: ZoneConfiguration,
    ) -> Result<ZoneConfiguration, ProcessZoneConfigurationError> {
        loop {
            let include = match configuration.directive_mut().version_mut() {
                directive::ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.include_mut().take().unwrap_or_default()
                }
            };

            if include.len() == 0 {
                break;
            }

            for item in include {
                let included_configuration_path = if configuration.path().is_dir() {
                    configuration.path()
                } else {
                    match configuration.path().parent() {
                        None => {
                            return Err(ProcessZoneConfigurationError::MissingParent(replace(
                                configuration.path_mut(),
                                PathBuf::default(),
                            )))
                        }
                        Some(p) => p,
                    }
                };

                let included_configuration_path = included_configuration_path.join(item);

                let included_configuration = self.process(ZoneConfiguration::new(
                    from_reader(&mut BufReader::new(File::open(
                        &included_configuration_path,
                    )?))?,
                    Vec::default(),
                    included_configuration_path,
                ))?;

                configuration = self.merge(configuration, included_configuration)?;
            }
        }

        Ok(configuration)
    }
}
