pub mod start;
pub mod stop;

pub use start::*;
pub use stop::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailExecuteConfiguration {
    start: Option<ZoneJailExecuteStartConfiguration>,
    stop: Option<ZoneJailExecuteStopConfiguration>,
}

impl ZoneJailExecuteConfiguration {
    pub fn new(
        start: Option<ZoneJailExecuteStartConfiguration>,
        stop: Option<ZoneJailExecuteStopConfiguration>,
    ) -> Self {
        Self { start, stop }
    }

    pub fn start(&self) -> &Option<ZoneJailExecuteStartConfiguration> {
        &self.start
    }

    pub fn start_mut(&mut self) -> &mut Option<ZoneJailExecuteStartConfiguration> {
        &mut self.start
    }

    pub fn set_start(&mut self, start: Option<ZoneJailExecuteStartConfiguration>) {
        self.start = start
    }

    pub fn stop(&self) -> &Option<ZoneJailExecuteStopConfiguration> {
        &self.stop
    }

    pub fn stop_mut(&mut self) -> &mut Option<ZoneJailExecuteStopConfiguration> {
        &mut self.stop
    }

    pub fn set_stop(&mut self, stop: Option<ZoneJailExecuteStopConfiguration>) {
        self.stop = stop
    }
}
