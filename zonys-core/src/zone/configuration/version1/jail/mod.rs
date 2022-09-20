pub mod execute;
pub mod operate;

pub use execute::*;
pub use operate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JailZoneConfigurationDirective {
    operate: Option<JailZoneOperateConfigurationDirective>,
    execute: Option<JailZoneExecuteConfigurationDirective>,
}

impl JailZoneConfigurationDirective {
    pub fn new(
        operate: Option<JailZoneOperateConfigurationDirective>,
        execute: Option<JailZoneExecuteConfigurationDirective>,
    ) -> Self {
        Self { operate, execute }
    }

    pub fn operate(&self) -> &Option<JailZoneOperateConfigurationDirective> {
        &self.operate
    }

    pub fn operate_mut(&mut self) -> &mut Option<JailZoneOperateConfigurationDirective> {
        &mut self.operate
    }

    pub fn set_operate(&mut self, operate: Option<JailZoneOperateConfigurationDirective>) {
        self.operate = operate
    }

    pub fn execute(&self) -> &Option<JailZoneExecuteConfigurationDirective> {
        &self.execute
    }

    pub fn execute_mut(&mut self) -> &mut Option<JailZoneExecuteConfigurationDirective> {
        &mut self.execute
    }

    pub fn set_execute(&mut self, execute: Option<JailZoneExecuteConfigurationDirective>) {
        self.execute = execute
    }
}
