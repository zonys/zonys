pub mod jail;

pub use self::jail::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::template::TemplateObject;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Version1ZoneConfigurationType {
    #[serde(rename = "jail")]
    Jail(Version1JailZoneConfiguration),
}

impl Default for Version1ZoneConfigurationType {
    fn default() -> Self {
        Self::Jail(Version1JailZoneConfiguration::default())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Version1ZoneConfiguration {
    variables: Option<TemplateObject>,
    #[serde(flatten)]
    r#type: Version1ZoneConfigurationType,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

impl Version1ZoneConfiguration {
    pub fn new(
        variables: Option<TemplateObject>,
        r#type: Version1ZoneConfigurationType,
        start_after_create: Option<bool>,
        destroy_after_stop: Option<bool>,
    ) -> Self {
        Self {
            variables,
            r#type,
            start_after_create,
            destroy_after_stop,
        }
    }

    pub fn variables(&self) -> &Option<TemplateObject> {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> &mut Option<TemplateObject> {
        &mut self.variables
    }

    pub fn set_variables(&mut self, variables: Option<TemplateObject>) {
        self.variables = variables
    }

    pub fn r#type(&self) -> &Version1ZoneConfigurationType {
        &self.r#type
    }

    pub fn type_mut(&mut self) -> &mut Version1ZoneConfigurationType {
        &mut self.r#type
    }

    pub fn set_type(&mut self, r#type: Version1ZoneConfigurationType) {
        self.r#type = r#type
    }

    pub fn start_after_create(&self) -> &Option<bool> {
        &self.start_after_create
    }

    pub fn start_after_create_mut(&mut self) -> &mut Option<bool> {
        &mut self.start_after_create
    }

    pub fn set_start_after_create(&mut self, start_after_create: Option<bool>) {
        self.start_after_create = start_after_create
    }

    pub fn destroy_after_stop(&self) -> &Option<bool> {
        &self.destroy_after_stop
    }

    pub fn destroy_after_stop_mut(&mut self) -> &mut Option<bool> {
        &mut self.destroy_after_stop
    }

    pub fn set_destroy_after_stop(&mut self, destroy_after_stop: Option<bool>) {
        self.destroy_after_stop = destroy_after_stop
    }
}
