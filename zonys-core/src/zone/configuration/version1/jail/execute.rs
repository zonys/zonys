use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneExecuteStartEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneExecuteStopEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneExecuteConfigurationDirective {
    start: Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>>,
    stop: Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>>,
}
