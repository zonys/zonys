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
pub struct Version1CreateExecuteJailZoneConfiguration {
    before: Option<Version1BeforeCreateExecuteJailZoneConfiguration>,
    on: Option<Version1OnCreateExecuteJailZoneConfiguration>,
    after: Option<Version1AfterCreateExecuteJailZoneConfiguration>,
}

impl Version1CreateExecuteJailZoneConfiguration {
    pub fn new(
        before: Option<Version1BeforeCreateExecuteJailZoneConfiguration>,
        on: Option<Version1OnCreateExecuteJailZoneConfiguration>,
        after: Option<Version1AfterCreateExecuteJailZoneConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<Version1BeforeCreateExecuteJailZoneConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<Version1BeforeCreateExecuteJailZoneConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<Version1BeforeCreateExecuteJailZoneConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<Version1OnCreateExecuteJailZoneConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<Version1OnCreateExecuteJailZoneConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<Version1OnCreateExecuteJailZoneConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<Version1AfterCreateExecuteJailZoneConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<Version1AfterCreateExecuteJailZoneConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<Version1AfterCreateExecuteJailZoneConfiguration>) {
        self.after = after
    }
}
