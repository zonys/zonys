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
pub struct ZoneJailOperateCreateConfigurationDirective {
    before: Option<ZoneJailOperateCreateBeforeConfigurationDirective>,
    on: Option<ZoneJailOperateCreateOnConfigurationDirective>,
    after: Option<ZoneJailOperateCreateAfterConfigurationDirective>,
}

impl ZoneJailOperateCreateConfigurationDirective {
    pub fn new(
        before: Option<ZoneJailOperateCreateBeforeConfigurationDirective>,
        on: Option<ZoneJailOperateCreateOnConfigurationDirective>,
        after: Option<ZoneJailOperateCreateAfterConfigurationDirective>,
    ) -> Self {
        Self { before, on, after }
    }

    pub fn before(&self) -> &Option<ZoneJailOperateCreateBeforeConfigurationDirective> {
        &self.before
    }

    pub fn before_mut(&mut self) -> &mut Option<ZoneJailOperateCreateBeforeConfigurationDirective> {
        &mut self.before
    }

    pub fn set_before(
        &mut self,
        before: Option<ZoneJailOperateCreateBeforeConfigurationDirective>,
    ) {
        self.before = before
    }

    pub fn on(&self) -> &Option<ZoneJailOperateCreateOnConfigurationDirective> {
        &self.on
    }

    pub fn on_mut(&mut self) -> &mut Option<ZoneJailOperateCreateOnConfigurationDirective> {
        &mut self.on
    }

    pub fn set_on(&mut self, on: Option<ZoneJailOperateCreateOnConfigurationDirective>) {
        self.on = on
    }

    pub fn after(&self) -> &Option<ZoneJailOperateCreateAfterConfigurationDirective> {
        &self.after
    }

    pub fn after_mut(&mut self) -> &mut Option<ZoneJailOperateCreateAfterConfigurationDirective> {
        &mut self.after
    }

    pub fn set_after(&mut self, after: Option<ZoneJailOperateCreateAfterConfigurationDirective>) {
        self.after = after
    }
}
