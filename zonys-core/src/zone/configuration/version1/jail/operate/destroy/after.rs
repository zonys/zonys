use serde::{Deserialize, Serialize};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneJailOperateDestroyAfterParentEntryConfiguration {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl ZoneJailOperateDestroyAfterParentEntryConfiguration {
    pub fn new(
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            program,
            arguments,
            environment_variables,
        }
    }

    pub fn program(&self) -> &String {
        &self.program
    }

    pub fn program_mut(&mut self) -> &mut String {
        &mut self.program
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program
    }

    pub fn arguments(&self) -> &Option<Vec<String>> {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.arguments
    }

    pub fn set_arguments(&mut self, arguments: Option<Vec<String>>) {
        self.arguments = arguments
    }

    pub fn environment_variables(&self) -> &Option<HashMap<String, String>> {
        &self.environment_variables
    }

    pub fn environment_variables_mut(&mut self) -> &mut Option<HashMap<String, String>> {
        &mut self.environment_variables
    }

    pub fn set_environment_variables(
        &mut self,
        environment_variables: Option<HashMap<String, String>>,
    ) {
        self.environment_variables = environment_variables
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "target")]
pub enum ZoneJailOperateDestroyAfterEntryConfiguration {
    #[serde(rename = "parent")]
    Parent(ZoneJailOperateDestroyAfterParentEntryConfiguration),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneJailOperateDestroyAfterConfiguration(
    Vec<ZoneJailOperateDestroyAfterEntryConfiguration>,
);

impl ZoneJailOperateDestroyAfterConfiguration {
    pub fn new(inner: Vec<ZoneJailOperateDestroyAfterEntryConfiguration>) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &Vec<ZoneJailOperateDestroyAfterEntryConfiguration> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<ZoneJailOperateDestroyAfterEntryConfiguration> {
        &mut self.0
    }

    pub fn set_inner(&mut self, inner: Vec<ZoneJailOperateDestroyAfterEntryConfiguration>) {
        self.0 = inner
    }
}
