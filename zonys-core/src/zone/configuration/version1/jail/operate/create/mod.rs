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
pub struct ZoneJailOperateCreateConfiguration {
    before: Option<ZoneJailOperateCreateBeforeConfiguration>,
    on: Option<ZoneJailOperateCreateOnConfiguration>,
    after: Option<ZoneJailOperateCreateAfterConfiguration>,
}

impl ZoneJailOperateCreateConfiguration {
    pub fn new(
        before: Option<ZoneJailOperateCreateBeforeConfiguration>,
        on: Option<ZoneJailOperateCreateOnConfiguration>,
        after: Option<ZoneJailOperateCreateAfterConfiguration>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailOperateCreateBeforeConfiguration> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailOperateCreateBeforeConfiguration> {
        &mut self.before
    }

    pub fn set_before(&mut self, before: Option<ZoneJailOperateCreateBeforeConfiguration>) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailOperateCreateOnConfiguration> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailOperateCreateOnConfiguration> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailOperateCreateOnConfiguration>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailOperateCreateAfterConfiguration> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailOperateCreateAfterConfiguration> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailOperateCreateAfterConfiguration>) {
        self.after = after
    }
}
