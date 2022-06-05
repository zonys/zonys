use serde::{Deserialize, Serialize};
use serde_yaml::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Serialize)]
pub enum ZoneTransmissionHeader {
    Version1(Version1ZoneTransmissionHeader),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Serialize)]
pub struct Version1ZoneTransmissionHeader {
    configuration: Value,
}

impl Version1ZoneTransmissionHeader {
    pub fn new(configuration: Value) -> Self {
        Self { configuration }
    }

    pub fn configuration(&self) -> &Value {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut Value {
        &mut self.configuration
    }

    pub fn set_configuration(&mut self, configuration: Value) {
        self.configuration = configuration
    }
}
