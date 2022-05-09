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
pub struct Version1DestroyExecuteJailZoneConfiguration {
    before: Option<Version1BeforeDestroyExecuteJailZoneConfiguration>,
    on: Option<Version1OnDestroyExecuteJailZoneConfiguration>,
    after: Option<Version1AfterDestroyExecuteJailZoneConfiguration>,
}

impl Version1DestroyExecuteJailZoneConfiguration {
    pub fn new(
        before: Option<Version1BeforeDestroyExecuteJailZoneConfiguration>,
        on: Option<Version1OnDestroyExecuteJailZoneConfiguration>,
        after: Option<Version1AfterDestroyExecuteJailZoneConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<Version1BeforeDestroyExecuteJailZoneConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<Version1BeforeDestroyExecuteJailZoneConfiguration> {
        &mut self.before
    }

    pub fn set_before(
        &mut self,
        before: Option<Version1BeforeDestroyExecuteJailZoneConfiguration>,
    ) {
        self.before = before
    }

    pub fn on(&self) -> &Option<Version1OnDestroyExecuteJailZoneConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<Version1OnDestroyExecuteJailZoneConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<Version1OnDestroyExecuteJailZoneConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<Version1AfterDestroyExecuteJailZoneConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<Version1AfterDestroyExecuteJailZoneConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<Version1AfterDestroyExecuteJailZoneConfiguration>) {
        self.after = after
    }
}
