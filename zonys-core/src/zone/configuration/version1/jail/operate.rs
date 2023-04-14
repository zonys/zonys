use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneOperateCreateEntryConfigurationDirective {
    parent: Option<bool>,
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneOperateDestroyEntryConfigurationDirective {
    parent: Option<bool>,
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneOperateConfigurationDirective {
    create: Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>>,
    destroy: Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>>,
}
