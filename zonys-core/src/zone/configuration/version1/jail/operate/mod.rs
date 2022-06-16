pub mod create;
pub mod destroy;

pub use create::*;
pub use destroy::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailOperateConfiguration {
    create: Option<ZoneJailOperateCreateConfiguration>,
    destroy: Option<ZoneJailOperateDestroyConfiguration>,
}

impl ZoneJailOperateConfiguration {
    pub fn new(
        create: Option<ZoneJailOperateCreateConfiguration>,
        destroy: Option<ZoneJailOperateDestroyConfiguration>,
    ) -> Self {
        Self { create, destroy }
    }

    pub fn create(&self) -> &Option<ZoneJailOperateCreateConfiguration> {
        &self.create
    }

    pub fn create_mut(&mut self) -> &mut Option<ZoneJailOperateCreateConfiguration> {
        &mut self.create
    }

    pub fn set_create(&mut self, create: Option<ZoneJailOperateCreateConfiguration>) {
        self.create = create
    }

    pub fn destroy(&self) -> &Option<ZoneJailOperateDestroyConfiguration> {
        &self.destroy
    }

    pub fn destroy_mut(&mut self) -> &mut Option<ZoneJailOperateDestroyConfiguration> {
        &mut self.destroy
    }

    pub fn set_destroy(&mut self, destroy: Option<ZoneJailOperateDestroyConfiguration>) {
        self.destroy = destroy
    }
}
