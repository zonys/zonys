pub mod start;
pub mod stop;

pub use start::*;
pub use stop::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ZoneJailExecuteConfigurationDirective {
    start: Option<ZoneJailExecuteStartConfigurationDirective>,
    stop: Option<ZoneJailExecuteStopConfigurationDirective>,
}

impl ZoneJailExecuteConfigurationDirective {
    pub fn new(
        start: Option<ZoneJailExecuteStartConfigurationDirective>,
        stop: Option<ZoneJailExecuteStopConfigurationDirective>,
    ) -> Self {
        Self { start, stop }
    }

    pub fn start(&self) -> &Option<ZoneJailExecuteStartConfigurationDirective> {
        &self.start
    }

    pub fn start_mut(&mut self) -> &mut Option<ZoneJailExecuteStartConfigurationDirective> {
        &mut self.start
    }

    pub fn set_start(&mut self, start: Option<ZoneJailExecuteStartConfigurationDirective>) {
        self.start = start
    }

    pub fn stop(&self) -> &Option<ZoneJailExecuteStopConfigurationDirective> {
        &self.stop
    }

    pub fn stop_mut(&mut self) -> &mut Option<ZoneJailExecuteStopConfigurationDirective> {
        &mut self.stop
    }

    pub fn set_stop(&mut self, stop: Option<ZoneJailExecuteStopConfigurationDirective>) {
        self.stop = stop
    }
}
