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
pub struct Version1StartExecuteJailZoneConfiguration {
    before: Option<Version1BeforeStartExecuteJailZoneConfiguration>,
    on: Option<Version1OnStartExecuteJailZoneConfiguration>,
    after: Option<Version1AfterStartExecuteJailZoneConfiguration>,
}

impl Version1StartExecuteJailZoneConfiguration {
    pub fn new(
        before: Option<Version1BeforeStartExecuteJailZoneConfiguration>,
        on: Option<Version1OnStartExecuteJailZoneConfiguration>,
        after: Option<Version1AfterStartExecuteJailZoneConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<Version1BeforeStartExecuteJailZoneConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<Version1BeforeStartExecuteJailZoneConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<Version1BeforeStartExecuteJailZoneConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<Version1OnStartExecuteJailZoneConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<Version1OnStartExecuteJailZoneConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<Version1OnStartExecuteJailZoneConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<Version1AfterStartExecuteJailZoneConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<Version1AfterStartExecuteJailZoneConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<Version1AfterStartExecuteJailZoneConfiguration>) {
        self.after = after
    }
}
