use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub enum ZoneTransmissionVersion1Type {
    Zfs,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneTransmissionVersion1Header {
    configuration: Vec<u8>,
    r#type: ZoneTransmissionVersion1Type,
}

impl ZoneTransmissionVersion1Header {
    pub fn new(configuration: Vec<u8>, r#type: ZoneTransmissionVersion1Type) -> Self {
        Self {
            configuration,
            r#type,
        }
    }

    pub fn configuration(&self) -> &Vec<u8> {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut Vec<u8> {
        &mut self.configuration
    }

    pub fn set_configuration(&mut self, configuration: Vec<u8>) {
        self.configuration = configuration
    }

    pub fn r#type(&self) -> &ZoneTransmissionVersion1Type {
        &self.r#type
    }

    pub fn type_mut(&mut self) -> &mut ZoneTransmissionVersion1Type {
        &mut self.r#type
    }

    pub fn set_type(&mut self, r#type: ZoneTransmissionVersion1Type) {
        self.r#type = r#type
    }
}
