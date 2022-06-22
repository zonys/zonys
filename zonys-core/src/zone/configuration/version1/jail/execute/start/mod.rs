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
pub struct ZoneJailExecuteStartConfigurationDirective {
    before: Option<ZoneJailExecuteStartBeforeConfigurationDirective>,
    on: Option<ZoneJailExecuteStartOnConfigurationDirective>,
    after: Option<ZoneJailExecuteStartAfterConfigurationDirective>,
}

impl ZoneJailExecuteStartConfigurationDirective {
    pub fn new(
        before: Option<ZoneJailExecuteStartBeforeConfigurationDirective>,
        on: Option<ZoneJailExecuteStartOnConfigurationDirective>,
        after: Option<ZoneJailExecuteStartAfterConfigurationDirective>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailExecuteStartBeforeConfigurationDirective> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailExecuteStartBeforeConfigurationDirective> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<ZoneJailExecuteStartBeforeConfigurationDirective>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailExecuteStartOnConfigurationDirective> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailExecuteStartOnConfigurationDirective> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailExecuteStartOnConfigurationDirective>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailExecuteStartAfterConfigurationDirective> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailExecuteStartAfterConfigurationDirective> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailExecuteStartAfterConfigurationDirective>) {
        self.after = after
    }
}
