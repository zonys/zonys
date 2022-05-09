pub mod after;
pub mod before;
pub mod on;

pub use after::*;
pub use before::*;
pub use on::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Version1StopExecuteJailZoneConfiguration {
    before: Option<Version1BeforeStopExecuteJailZoneConfiguration>,
    on: Option<Version1OnStopExecuteJailZoneConfiguration>,
    after: Option<Version1AfterStopExecuteJailZoneConfiguration>,
}

impl Version1StopExecuteJailZoneConfiguration {
    pub fn new(
        before: Option<Version1BeforeStopExecuteJailZoneConfiguration>,
        on: Option<Version1OnStopExecuteJailZoneConfiguration>,
        after: Option<Version1AfterStopExecuteJailZoneConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<Version1BeforeStopExecuteJailZoneConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<Version1BeforeStopExecuteJailZoneConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<Version1BeforeStopExecuteJailZoneConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<Version1OnStopExecuteJailZoneConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<Version1OnStopExecuteJailZoneConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<Version1OnStopExecuteJailZoneConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<Version1AfterStopExecuteJailZoneConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<Version1AfterStopExecuteJailZoneConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<Version1AfterStopExecuteJailZoneConfiguration>) {
        self.after = after
    }
}
