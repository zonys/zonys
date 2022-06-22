pub mod error;
pub mod version1;

////////////////////////////////////////////////////////////////////////////////////////////////////

use error::ProcessZoneConfigurationError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum ZoneConfigurationVersionDirective {
    #[serde(rename = "1")]
    Version1(version1::ZoneConfiguration),
}

impl Default for ZoneConfigurationVersionDirective {
    fn default() -> Self {
        Self::Version1(version1::ZoneConfiguration::default())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ZoneConfigurationDirective {
    #[serde(flatten)]
    version: ZoneConfigurationVersionDirective,
}

impl ZoneConfigurationDirective {
    pub fn new(version: ZoneConfigurationVersionDirective) -> Self {
        Self { version }
    }

    pub fn version(&self) -> &ZoneConfigurationVersionDirective {
        &self.version
    }

    pub fn version_mut(&mut self) -> &mut ZoneConfigurationVersionDirective {
        &mut self.version
    }

    pub fn set_version(&mut self, version: ZoneConfigurationVersionDirective) {
        self.version = version
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug)]
pub struct ZoneConfiguration {
    directive: ZoneConfigurationDirective,
    path: PathBuf,
}

impl ZoneConfiguration {
    pub fn new(directive: ZoneConfigurationDirective, path: PathBuf) -> Self {
        Self { directive, path }
    }

    pub fn directive(&self) -> &ZoneConfigurationDirective {
        &self.directive
    }

    pub fn directive_mut(&mut self) -> &mut ZoneConfigurationDirective {
        &mut self.directive
    }

    pub fn set_directive(&mut self, directive: ZoneConfigurationDirective) {
        self.directive = directive
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ZoneConfigurationProcessor {}

impl ZoneConfigurationProcessor {
    pub fn process(
        &self,
        configuration: ZoneConfiguration,
    ) -> Result<ZoneConfiguration, ProcessZoneConfigurationError> {
        Ok(configuration)
    }
}
