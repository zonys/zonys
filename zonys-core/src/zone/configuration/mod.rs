pub mod version1;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum ZoneConfiguration {
    #[serde(rename = "1")]
    Version1(version1::ZoneConfiguration),
}

impl Default for ZoneConfiguration {
    fn default() -> Self {
        Self::Version1(version1::ZoneConfiguration::default())
    }
}
