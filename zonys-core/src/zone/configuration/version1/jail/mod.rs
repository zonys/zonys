pub mod execute;
pub mod operate;

pub use execute::*;
pub use operate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct JailZoneConfigurationDirective {
    operate: Option<JailZoneOperateConfigurationDirective>,
    execute: Option<JailZoneExecuteConfigurationDirective>,
}
