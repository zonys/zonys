mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::configuration::directive::jail::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::TemplateObject;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use ztd::{Constructor, Display, Error, From, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadZoneConfigurationDirectiveError {
    YamlError(serde_yaml::Error),
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum MergeZoneConfigurationDirectiveError {
    _Placeholder,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationDirective {
    #[serde(flatten)]
    version: ZoneConfigurationVersionDirective,
}

impl ZoneConfigurationDirective {
    pub fn read_from_yaml_path(path: &Path) -> Result<Self, ReadZoneConfigurationDirectiveError> {
        Ok(from_reader(BufReader::new(File::open(path)?))?)
    }

    pub fn read_from_path(path: &Path) -> Result<Self, ReadZoneConfigurationDirectiveError> {
        Self::read_from_yaml_path(path)
    }

    pub fn variables(&self) -> &Option<TemplateObject> {
        match &self.version {
            ZoneConfigurationVersionDirective::Version1(version1) => version1.variables(),
        }
    }

    pub fn merge(
        self,
        directive: ZoneConfigurationDirective,
    ) -> Result<Self, MergeZoneConfigurationDirectiveError> {
        Ok(directive)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum ZoneConfigurationVersionDirective {
    #[serde(rename = "experimental")]
    Version1(ZoneConfigurationVersion1Directive),
}

impl Default for ZoneConfigurationVersionDirective {
    fn default() -> Self {
        Self::Version1(ZoneConfigurationVersion1Directive::default())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1ChildDirective {
    source: String,
    directive: ZoneConfigurationDirective,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1Directive {
    tags: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    #[serde(flatten)]
    r#type: ZoneConfigurationVersion1TypeDirective,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ZoneConfigurationVersion1TypeDirective {
    #[serde(rename = "jail")]
    Jail(ZoneConfigurationVersion1JailDirective),
}

impl Default for ZoneConfigurationVersion1TypeDirective {
    fn default() -> Self {
        Self::Jail(ZoneConfigurationVersion1JailDirective::default())
    }
}
