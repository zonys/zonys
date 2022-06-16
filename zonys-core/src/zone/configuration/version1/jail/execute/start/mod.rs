pub mod after;
pub mod before;
pub mod on;

pub use after::*;
pub use before::*;
pub use on::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailExecuteStartConfiguration {
    before: Option<ZoneJailExecuteStartBeforeConfiguration>,
    on: Option<ZoneJailExecuteStartOnConfiguration>,
    after: Option<ZoneJailExecuteStartAfterConfiguration>,
}

impl ZoneJailExecuteStartConfiguration {
    pub fn new(
        before: Option<ZoneJailExecuteStartBeforeConfiguration>,
        on: Option<ZoneJailExecuteStartOnConfiguration>,
        after: Option<ZoneJailExecuteStartAfterConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailExecuteStartBeforeConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailExecuteStartBeforeConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<ZoneJailExecuteStartBeforeConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailExecuteStartOnConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailExecuteStartOnConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailExecuteStartOnConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailExecuteStartAfterConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailExecuteStartAfterConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailExecuteStartAfterConfiguration>) {
        self.after = after
    }
}
