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
pub struct ZoneJailExecuteStopConfiguration {
    before: Option<ZoneJailExecuteStopBeforeConfiguration>,
    on: Option<ZoneJailExecuteStopOnConfiguration>,
    after: Option<ZoneJailExecuteStopAfterConfiguration>,
}

impl ZoneJailExecuteStopConfiguration {
    pub fn new(
        before: Option<ZoneJailExecuteStopBeforeConfiguration>,
        on: Option<ZoneJailExecuteStopOnConfiguration>,
        after: Option<ZoneJailExecuteStopAfterConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailExecuteStopBeforeConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailExecuteStopBeforeConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<ZoneJailExecuteStopBeforeConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailExecuteStopOnConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailExecuteStopOnConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailExecuteStopOnConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailExecuteStopAfterConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailExecuteStopAfterConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailExecuteStopAfterConfiguration>) {
        self.after = after
    }
}
