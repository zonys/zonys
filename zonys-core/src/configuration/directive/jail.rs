use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailDirective {
    from: Option<String>,
    from_work_path: Option<String>,
    volume: Option<ZoneConfigurationVersion1VolumeDirective>,
    execute: Option<ZoneConfigurationVersion1JailExecuteDirective>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1VolumeDirective {
    #[serde(alias = "auto", rename = "automatic")]
    Automatic,
    #[serde(rename = "zfs")]
    Zfs,
    #[serde(rename = "directory")]
    Directory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailExecuteDirective {
    create: Option<ZoneConfigurationVersion1JailCreateDirective>,
    start: Option<ZoneConfigurationVersion1JailStartDirective>,
    stop: Option<ZoneConfigurationVersion1JailStopDirective>,
    destroy: Option<ZoneConfigurationVersion1JailDestroyDirective>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailCreateDirective {
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailStartDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailStopDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailDestroyDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailProgramDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}
