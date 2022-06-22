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
pub struct ZoneJailOperateDestroyConfigurationDirective {
    before: Option<ZoneJailOperateDestroyBeforeConfigurationDirective>,
    on: Option<ZoneJailOperateDestroyOnConfigurationDirective>,
    after: Option<ZoneJailOperateDestroyAfterConfigurationDirective>,
}

impl ZoneJailOperateDestroyConfigurationDirective {
    pub fn new(
        before: Option<ZoneJailOperateDestroyBeforeConfigurationDirective>,
        on: Option<ZoneJailOperateDestroyOnConfigurationDirective>,
        after: Option<ZoneJailOperateDestroyAfterConfigurationDirective>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailOperateDestroyBeforeConfigurationDirective> {
        &self.before
    }

    pub fn before_mut(
        &mut self,
    ) -> &mut Option<ZoneJailOperateDestroyBeforeConfigurationDirective> {
        &mut self.before
    }

    pub fn set_before(
        &mut self,
        before: Option<ZoneJailOperateDestroyBeforeConfigurationDirective>,
    ) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailOperateDestroyOnConfigurationDirective> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyOnConfigurationDirective> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailOperateDestroyOnConfigurationDirective>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailOperateDestroyAfterConfigurationDirective> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyAfterConfigurationDirective> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailOperateDestroyAfterConfigurationDirective>) {
        self.after = after
    }
}
