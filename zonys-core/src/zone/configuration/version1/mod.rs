pub mod jail;

pub use self::jail::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::template::TemplateObject;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
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
