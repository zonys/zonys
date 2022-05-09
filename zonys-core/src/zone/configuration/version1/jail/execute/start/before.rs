use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Version1BeforeStartExecuteJailZoneConfigurationParentEntry {
    program: String,
    arguments: Option<Vec<String>>,
}

impl Version1BeforeStartExecuteJailZoneConfigurationParentEntry {
    pub fn new(program: String, arguments: Option<Vec<String>>) -> Self {
        Self { program, arguments }
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "target")]
pub enum Version1BeforeStartExecuteJailZoneConfigurationEntry {
    #[serde(rename = "parent")]
    Parent(Version1BeforeStartExecuteJailZoneConfigurationParentEntry),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Version1BeforeStartExecuteJailZoneConfiguration(
    Vec<Version1BeforeStartExecuteJailZoneConfigurationEntry>,
);

impl Version1BeforeStartExecuteJailZoneConfiguration {
    pub fn new(inner: Vec<Version1BeforeStartExecuteJailZoneConfigurationEntry>) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &Vec<Version1BeforeStartExecuteJailZoneConfigurationEntry> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Version1BeforeStartExecuteJailZoneConfigurationEntry> {
        &mut self.0
    }

    pub fn set_inner(&mut self, inner: Vec<Version1BeforeStartExecuteJailZoneConfigurationEntry>) {
        self.0 = inner
    }
}
