pub mod create;
pub mod destroy;
pub mod start;
pub mod stop;

pub use create::*;
pub use destroy::*;
pub use start::*;
pub use stop::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Version1ExecuteJailZoneConfiguration {
    create: Option<Version1CreateExecuteJailZoneConfiguration>,
    start: Option<Version1StartExecuteJailZoneConfiguration>,
    stop: Option<Version1StopExecuteJailZoneConfiguration>,
    destroy: Option<Version1DestroyExecuteJailZoneConfiguration>,
}

impl Version1ExecuteJailZoneConfiguration {
    pub fn new(
        create: Option<Version1CreateExecuteJailZoneConfiguration>,
        start: Option<Version1StartExecuteJailZoneConfiguration>,
        stop: Option<Version1StopExecuteJailZoneConfiguration>,
        destroy: Option<Version1DestroyExecuteJailZoneConfiguration>,
    ) -> Self {
        Self {
            create,
            start,
            stop,
            destroy,
        }
    }

    pub fn create(&self) -> &Option<Version1CreateExecuteJailZoneConfiguration> {
        &self.create
    }

    pub fn create_mut(&mut self) -> &mut Option<Version1CreateExecuteJailZoneConfiguration> {
        &mut self.create
    }

    pub fn set_create(&mut self, create: Option<Version1CreateExecuteJailZoneConfiguration>) {
        self.create = create
    }

    pub fn start(&self) -> &Option<Version1StartExecuteJailZoneConfiguration> {
        &self.start
    }

    pub fn start_mut(&mut self) -> &mut Option<Version1StartExecuteJailZoneConfiguration> {
        &mut self.start
    }

    pub fn set_start(&mut self, start: Option<Version1StartExecuteJailZoneConfiguration>) {
        self.start = start
    }

    pub fn stop(&self) -> &Option<Version1StopExecuteJailZoneConfiguration> {
        &self.stop
    }

    pub fn stop_mut(&mut self) -> &mut Option<Version1StopExecuteJailZoneConfiguration> {
        &mut self.stop
    }

    pub fn set_stop(&mut self, stop: Option<Version1StopExecuteJailZoneConfiguration>) {
        self.stop = stop
    }

    pub fn destroy(&self) -> &Option<Version1DestroyExecuteJailZoneConfiguration> {
        &self.destroy
    }

    pub fn destroy_mut(&mut self) -> &mut Option<Version1DestroyExecuteJailZoneConfiguration> {
        &mut self.destroy
    }

    pub fn set_destroy(&mut self, destroy: Option<Version1DestroyExecuteJailZoneConfiguration>) {
        self.destroy = destroy
    }
}
