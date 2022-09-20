pub mod error;
pub mod event;
pub mod jail;
pub mod null;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use self::jail::*;
pub use error::*;
pub use event::*;
pub use null::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ZoneExecutor {
    fn running(
        &self,
        event: RunningZoneExecutorEvent,
    ) -> Result<RunningZoneExecutorEvent, RunningZoneExecutorEventError> {
        Ok(event)
    }

    fn create(
        &mut self,
        event: CreateZoneExecutorEvent,
    ) -> Result<CreateZoneExecutorEvent, CreateZoneExecutorEventError> {
        Ok(event)
    }

    fn start(
        &mut self,
        event: StartZoneExecutorEvent,
    ) -> Result<StartZoneExecutorEvent, StartZoneExecutorEventError> {
        Ok(event)
    }

    fn stop(
        &mut self,
        event: StopZoneExecutorEvent,
    ) -> Result<StopZoneExecutorEvent, StopZoneExecutorEventError> {
        Ok(event)
    }

    fn destroy(
        &mut self,
        event: DestroyZoneExecutorEvent,
    ) -> Result<DestroyZoneExecutorEvent, DestroyZoneExecutorEventError> {
        Ok(event)
    }
}
