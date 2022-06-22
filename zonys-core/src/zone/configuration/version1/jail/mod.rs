pub mod execute;
pub mod operate;

pub use execute::*;
pub use operate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailConfigurationDirective {
    operate: Option<ZoneJailOperateConfigurationDirective>,
    execute: Option<ZoneJailExecuteConfigurationDirective>,
}

impl ZoneJailConfigurationDirective {
    pub fn new(
        operate: Option<ZoneJailOperateConfigurationDirective>,
        execute: Option<ZoneJailExecuteConfigurationDirective>,
    ) -> Self {
        Self { operate, execute }
    }

    pub fn operate(&self) -> &Option<ZoneJailOperateConfigurationDirective> {
        &self.operate
    }

    pub fn operate_mut(&mut self) -> &mut Option<ZoneJailOperateConfigurationDirective> {
        &mut self.operate
    }

    pub fn set_operate(&mut self, operate: Option<ZoneJailOperateConfigurationDirective>) {
        self.operate = operate
    }

    pub fn execute(&self) -> &Option<ZoneJailExecuteConfigurationDirective> {
        &self.execute
    }

    pub fn execute_mut(&mut self) -> &mut Option<ZoneJailExecuteConfigurationDirective> {
        &mut self.execute
    }

    pub fn set_execute(&mut self, execute: Option<ZoneJailExecuteConfigurationDirective>) {
        self.execute = execute
    }
}
