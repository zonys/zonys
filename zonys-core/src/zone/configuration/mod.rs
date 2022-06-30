pub mod error;
pub mod version1;

////////////////////////////////////////////////////////////////////////////////////////////////////

use error::{MergeZoneConfigurationError, ProcessZoneConfigurationError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, from_value, to_value, Value};
use std::fs::File;
use std::io::BufReader;
use std::mem::replace;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum ZoneConfigurationVersionDirective {
    #[serde(rename = "1")]
    Version1(version1::ZoneConfigurationDirective),
}

impl Default for ZoneConfigurationVersionDirective {
    fn default() -> Self {
        Self::Version1(version1::ZoneConfigurationDirective::default())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ZoneConfigurationDirective {
    #[serde(flatten)]
    version: ZoneConfigurationVersionDirective,
}

impl ZoneConfigurationDirective {
    pub fn new(version: ZoneConfigurationVersionDirective) -> Self {
        Self { version }
    }

    pub fn version(&self) -> &ZoneConfigurationVersionDirective {
        &self.version
    }

    pub fn version_mut(&mut self) -> &mut ZoneConfigurationVersionDirective {
        &mut self.version
    }

    pub fn set_version(&mut self, version: ZoneConfigurationVersionDirective) {
        self.version = version
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug)]
pub struct ZoneConfiguration {
    directive: ZoneConfigurationDirective,
    path: PathBuf,
}

impl ZoneConfiguration {
    pub fn new(directive: ZoneConfigurationDirective, path: PathBuf) -> Self {
        Self { directive, path }
    }

    pub fn directive(&self) -> &ZoneConfigurationDirective {
        &self.directive
    }

    pub fn directive_mut(&mut self) -> &mut ZoneConfigurationDirective {
        &mut self.directive
    }

    pub fn set_directive(&mut self, directive: ZoneConfigurationDirective) {
        self.directive = directive
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path
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
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
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
                    included_configuration_path,
                ))?;

                configuration = self.merge(configuration, included_configuration)?;
            }
        }

        Ok(configuration)
    }
}
