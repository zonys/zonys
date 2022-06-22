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
pub struct ZoneJailExecuteStopConfigurationDirective {
    before: Option<ZoneJailExecuteStopBeforeConfigurationDirective>,
    on: Option<ZoneJailExecuteStopOnConfigurationDirective>,
    after: Option<ZoneJailExecuteStopAfterConfigurationDirective>,
}

impl ZoneJailExecuteStopConfigurationDirective {
    pub fn new(
        before: Option<ZoneJailExecuteStopBeforeConfigurationDirective>,
        on: Option<ZoneJailExecuteStopOnConfigurationDirective>,
        after: Option<ZoneJailExecuteStopAfterConfigurationDirective>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailExecuteStopBeforeConfigurationDirective> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailExecuteStopBeforeConfigurationDirective> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<ZoneJailExecuteStopBeforeConfigurationDirective>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailExecuteStopOnConfigurationDirective> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailExecuteStopOnConfigurationDirective> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailExecuteStopOnConfigurationDirective>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailExecuteStopAfterConfigurationDirective> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailExecuteStopAfterConfigurationDirective> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailExecuteStopAfterConfigurationDirective>) {
        self.after = after
    }
}
