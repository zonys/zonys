pub mod jail;

pub use self::jail::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::template::TemplateObject;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ZoneConfigurationTypeDirective {
    #[serde(rename = "jail")]
    Jail(ZoneJailConfigurationDirective),
    #[serde(rename = "undefined")]
    Undefined(Value),
}

impl Default for ZoneConfigurationTypeDirective {
    fn default() -> Self {
        Self::Jail(ZoneJailConfigurationDirective::default())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneConfigurationDirective {
    include: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    tags: Option<Vec<String>>,
    #[serde(flatten)]
    r#type: ZoneConfigurationTypeDirective,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

impl ZoneConfigurationDirective {
    pub fn new(
        include: Option<Vec<String>>,
        variables: Option<TemplateObject>,
        tags: Option<Vec<String>>,
        r#type: ZoneConfigurationTypeDirective,
        start_after_create: Option<bool>,
        destroy_after_stop: Option<bool>,
    ) -> Self {
        Self {
            include,
            variables,
            tags,
            r#type,
            start_after_create,
            destroy_after_stop,
        }
    }

    pub fn include(&self) -> &Option<Vec<String>> {
        &self.include
    }

    pub fn include_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.include
    }

    pub fn set_include(&mut self, include: Option<Vec<String>>) {
        self.include = include
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

    pub fn tags(&self) -> &Option<Vec<String>> {
        &self.tags
    }

    pub fn tags_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.tags
    }

    pub fn set_tags(&mut self, tags: Option<Vec<String>>) {
        self.tags = tags
    }

    pub fn r#type(&self) -> &ZoneConfigurationTypeDirective {
        &self.r#type
    }

    pub fn type_mut(&mut self) -> &mut ZoneConfigurationTypeDirective {
        &mut self.r#type
    }

    pub fn set_type(&mut self, r#type: ZoneConfigurationTypeDirective) {
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
