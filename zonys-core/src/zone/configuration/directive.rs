use crate::zone::ResolveZoneConfigurationDirectiveError;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use url::{ParseError, Url};
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationDirective {
    #[serde(flatten)]
    version: ZoneConfigurationVersionDirective,
}

impl ZoneConfigurationDirective {
    pub fn from(&self) -> &Option<String> {
        match &self.version {
            ZoneConfigurationVersionDirective::Version1(version1) => version1.from(),
        }
    }

    pub fn include(&self) -> Box<dyn Iterator<Item = &'_ String> + '_> {
        match &self.version {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                let include = match version1.include() {
                    None => return Box::new(std::iter::empty()),
                    Some(include) => include,
                };

                Box::new(include.iter())
            }
        }
    }

    fn resolve_local(
        include_path: PathBuf,
        base_path: Option<&Path>,
    ) -> Result<(BufReader<File>, Option<PathBuf>), ResolveZoneConfigurationDirectiveError> {
        let path = match (include_path, base_path) {
            (path, _) if path.is_absolute() => path,
            (path, Some(base_path)) if path.is_relative() => {
                base_path.iter().chain(path.iter()).collect()
            }
            (path, _) => path,
        };

        Ok((BufReader::new(File::open(path.clone())?), Some(path)))
    }

    pub fn resolve(
        &self,
        base_path: Option<&Path>,
    ) -> Result<Vec<ZoneConfigurationDirective>, ResolveZoneConfigurationDirectiveError> {
        let mut directives = Vec::default();

        for include in self.include() {
            let (reader, source) = match Url::parse(include) {
                Ok(url) => match url.scheme() {
                    "" | "file" => Self::resolve_local(PathBuf::from(url.path()), base_path)?,
                    _ => {
                        return Err(ResolveZoneConfigurationDirectiveError::UnsupportedScheme(
                            String::from(url.scheme()),
                        ))
                    }
                },
                Err(ParseError::RelativeUrlWithoutBase) => {
                    Self::resolve_local(PathBuf::from(include), base_path)?
                }
                Err(error) => return Err(error.into()),
            };

            let directive = from_reader::<_, ZoneConfigurationDirective>(reader)?;
            let directive_directives = match source {
                Some(source) => directive.resolve(Some(source.as_path()))?,
                None => directive.resolve(None)?,
            };

            directives.push(directive);
            directives.extend(directive_directives);
        }

        Ok(directives)
    }
}

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

pub mod version1 {
    use crate::template::TemplateObject;
    use serde::{Deserialize, Serialize};
    use serde_yaml::Value;
    use std::collections::HashMap;
    use ztd::{Constructor, Method};

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum ZoneConfigurationTypeDirective {
        #[serde(rename = "jail")]
        Jail(JailZoneConfigurationDirective),
        #[serde(rename = "undefined")]
        Undefined(Value),
    }

    impl Default for ZoneConfigurationTypeDirective {
        fn default() -> Self {
            Self::Undefined(Value::Null)
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum ZoneConfigurationFileSystemDirective {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "zfs")]
        Zfs,
        #[serde(rename = "directory")]
        Directory,
    }

    impl Default for ZoneConfigurationFileSystemDirective {
        fn default() -> Self {
            Self::Automatic
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct ZoneConfigurationDirective {
        from: Option<String>,
        include: Option<Vec<String>>,
        variables: Option<TemplateObject>,
        tags: Option<Vec<String>>,
        #[serde(flatten, default = "ZoneConfigurationTypeDirective::Undefined")]
        r#type: ZoneConfigurationTypeDirective,
        start_after_create: Option<bool>,
        destroy_after_stop: Option<bool>,
        #[serde(flatten, default = "ZoneConfigurationTypeDirective::Automatic")]
        file_system: Option<ZoneConfigurationFileSystemDirective>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneConfigurationDirective {
        operate: Option<JailZoneOperateConfigurationDirective>,
        execute: Option<JailZoneExecuteConfigurationDirective>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneExecuteStartEntryConfigurationDirective {
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneExecuteStopEntryConfigurationDirective {
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneExecuteConfigurationDirective {
        start: Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>>,
        stop: Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneOperateCreateEntryConfigurationDirective {
        parent: Option<bool>,
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneOperateDestroyEntryConfigurationDirective {
        parent: Option<bool>,
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
    #[Method(all)]
    pub struct JailZoneOperateConfigurationDirective {
        create: Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>>,
        destroy: Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>>,
    }
}
