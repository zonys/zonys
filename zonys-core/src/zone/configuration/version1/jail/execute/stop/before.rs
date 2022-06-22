use serde::{Deserialize, Serialize};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneJailExecuteStopBeforeParentEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl ZoneJailExecuteStopBeforeParentEntryConfigurationDirective {
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
pub struct ZoneJailExecuteStopBeforeChildEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl ZoneJailExecuteStopBeforeChildEntryConfigurationDirective {
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
pub enum ZoneJailExecuteStopBeforeEntryConfigurationDirective {
    #[serde(rename = "parent")]
    Parent(ZoneJailExecuteStopBeforeParentEntryConfigurationDirective),
    #[serde(rename = "child")]
    Child(ZoneJailExecuteStopBeforeChildEntryConfigurationDirective),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneJailExecuteStopBeforeConfigurationDirective(
    Vec<ZoneJailExecuteStopBeforeEntryConfigurationDirective>,
);

impl ZoneJailExecuteStopBeforeConfigurationDirective {
    pub fn new(inner: Vec<ZoneJailExecuteStopBeforeEntryConfigurationDirective>) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &Vec<ZoneJailExecuteStopBeforeEntryConfigurationDirective> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<ZoneJailExecuteStopBeforeEntryConfigurationDirective> {
        &mut self.0
    }

    pub fn set_inner(&mut self, inner: Vec<ZoneJailExecuteStopBeforeEntryConfigurationDirective>) {
        self.0 = inner
    }
}
