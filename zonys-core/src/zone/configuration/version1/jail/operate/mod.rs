pub mod create;
pub mod destroy;

pub use create::*;
pub use destroy::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailOperateConfigurationDirective {
    create: Option<ZoneJailOperateCreateConfigurationDirective>,
    destroy: Option<ZoneJailOperateDestroyConfigurationDirective>,
}

impl ZoneJailOperateConfigurationDirective {
    pub fn new(
        create: Option<ZoneJailOperateCreateConfigurationDirective>,
        destroy: Option<ZoneJailOperateDestroyConfigurationDirective>,
    ) -> Self {
        Self { create, destroy }
    }

    pub fn create(&self) -> &Option<ZoneJailOperateCreateConfigurationDirective> {
        &self.create
    }

    pub fn create_mut(&mut self) -> &mut Option<ZoneJailOperateCreateConfigurationDirective> {
        &mut self.create
    }

    pub fn set_create(&mut self, create: Option<ZoneJailOperateCreateConfigurationDirective>) {
        self.create = create
    }

    pub fn destroy(&self) -> &Option<ZoneJailOperateDestroyConfigurationDirective> {
        &self.destroy
    }

    pub fn destroy_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyConfigurationDirective> {
        &mut self.destroy
    }

    pub fn set_destroy(&mut self, destroy: Option<ZoneJailOperateDestroyConfigurationDirective>) {
        self.destroy = destroy
    }
}
