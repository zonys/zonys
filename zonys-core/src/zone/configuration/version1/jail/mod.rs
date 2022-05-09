pub mod execute;

pub use execute::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Version1NetworkJailZoneConfiguration {
    #[serde(rename = "parent")]
    Parent,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Version1JailZoneConfiguration {
    execute: Option<Version1ExecuteJailZoneConfiguration>,
    network: Option<Version1NetworkJailZoneConfiguration>,
}

impl Version1JailZoneConfiguration {
    pub fn new(execute: Option<Version1ExecuteJailZoneConfiguration>, network: Option<Version1NetworkJailZoneConfiguration>) -> Self {
        Self { execute, network }
    }

    pub fn execute(&self) -> &Option<Version1ExecuteJailZoneConfiguration> {
        &self.execute
    }

    pub fn execute_mut(&mut self) -> &mut Option<Version1ExecuteJailZoneConfiguration> {
        &mut self.execute
    }

    pub fn set_execute(&mut self, execute: Option<Version1ExecuteJailZoneConfiguration>) {
        self.execute = execute
    }

    pub fn network(&self) -> &Option<Version1NetworkJailZoneConfiguration> {
        &self.network
    }

    pub fn network_mut(&mut self) -> &mut Option<Version1NetworkJailZoneConfiguration> {
        &mut self.network
    }

    pub fn set_network(&mut self, network: Option<Version1NetworkJailZoneConfiguration>) {
        self.network = network
    }
}
