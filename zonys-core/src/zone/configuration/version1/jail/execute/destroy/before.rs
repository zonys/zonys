use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Version1BeforeDestroyExecuteJailZoneConfigurationParentEntry {
    program: String,
    arguments: Option<Vec<String>>,
}

impl Version1BeforeDestroyExecuteJailZoneConfigurationParentEntry {
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
pub struct Version1BeforeDestroyExecuteJailZoneConfigurationChildEntry {
    program: String,
    arguments: Option<Vec<String>>,
}

impl Version1BeforeDestroyExecuteJailZoneConfigurationChildEntry {
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
pub enum Version1BeforeDestroyExecuteJailZoneConfigurationEntry {
    #[serde(rename = "parent")]
    Parent(Version1BeforeDestroyExecuteJailZoneConfigurationParentEntry),
    #[serde(rename = "parent")]
    Child(Version1BeforeDestroyExecuteJailZoneConfigurationChildEntry),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Version1BeforeDestroyExecuteJailZoneConfiguration(
    Vec<Version1BeforeDestroyExecuteJailZoneConfigurationEntry>,
);

impl Version1BeforeDestroyExecuteJailZoneConfiguration {
    pub fn new(inner: Vec<Version1BeforeDestroyExecuteJailZoneConfigurationEntry>) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &Vec<Version1BeforeDestroyExecuteJailZoneConfigurationEntry> {
        &self.0
    }

    pub fn inner_mut(
        &mut self,
    ) -> &mut Vec<Version1BeforeDestroyExecuteJailZoneConfigurationEntry> {
        &mut self.0
    }

    pub fn set_inner(
        &mut self,
        inner: Vec<Version1BeforeDestroyExecuteJailZoneConfigurationEntry>,
    ) {
        self.0 = inner
    }
}
