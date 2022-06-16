pub mod execute;
pub mod operate;

pub use execute::*;
pub use operate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailConfiguration {
    operate: Option<ZoneJailOperateConfiguration>,
    execute: Option<ZoneJailExecuteConfiguration>,
}

impl ZoneJailConfiguration {
    pub fn new(
        operate: Option<ZoneJailOperateConfiguration>,
        execute: Option<ZoneJailExecuteConfiguration>,
    ) -> Self {
        Self { operate, execute }
    }

    pub fn operate(&self) -> &Option<ZoneJailOperateConfiguration> {
        &self.operate
    }

    pub fn operate_mut(&mut self) -> &mut Option<ZoneJailOperateConfiguration> {
        &mut self.operate
    }

    pub fn set_operate(&mut self, operate: Option<ZoneJailOperateConfiguration>) {
        self.operate = operate
    }

    pub fn execute(&self) -> &Option<ZoneJailExecuteConfiguration> {
        &self.execute
    }

    pub fn execute_mut(&mut self) -> &mut Option<ZoneJailExecuteConfiguration> {
        &mut self.execute
    }

    pub fn set_execute(&mut self, execute: Option<ZoneJailExecuteConfiguration>) {
        self.execute = execute
    }
}
