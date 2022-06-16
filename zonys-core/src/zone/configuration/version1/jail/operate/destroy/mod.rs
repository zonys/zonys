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
pub struct ZoneJailOperateDestroyConfiguration {
    before: Option<ZoneJailOperateDestroyBeforeConfiguration>,
    on: Option<ZoneJailOperateDestroyOnConfiguration>,
    after: Option<ZoneJailOperateDestroyAfterConfiguration>,
}

impl ZoneJailOperateDestroyConfiguration {
    pub fn new(
        before: Option<ZoneJailOperateDestroyBeforeConfiguration>,
        on: Option<ZoneJailOperateDestroyOnConfiguration>,
        after: Option<ZoneJailOperateDestroyAfterConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailOperateDestroyBeforeConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyBeforeConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<ZoneJailOperateDestroyBeforeConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailOperateDestroyOnConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyOnConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailOperateDestroyOnConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailOperateDestroyAfterConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyAfterConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailOperateDestroyAfterConfiguration>) {
        self.after = after
    }
}
