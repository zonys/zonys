/*use crate::{Zone, ZoneStatus, CreateZoneError, StartZoneError, StopZoneError, DestroyZoneError};
use ztd::{Constructor, Error, Display, From, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Method)]
#[Constructor(visibility = pub(crate))]
#[Method(accessors)]
pub struct ZoneCreateEvent<'a> {
    zone: &'a Zone,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Method)]
#[Constructor(visibility = pub(crate))]
#[Method(accessors)]
pub struct ZoneStartEvent<'a> {
    zone: &'a Zone,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Method)]
#[Constructor(visibility = pub(crate))]
#[Method(accessors)]
pub struct ZoneStopEvent<'a> {
    zone: &'a Zone,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Method)]
#[Constructor(visibility = pub(crate))]
#[Method(accessors)]
pub struct ZoneDestroyEvent<'a> {
    zone: &'a Zone,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ZoneExecutor {
    fn create(
        &mut self,
        _event: ZoneCreateEvent,
    ) -> Result<(), CreateZoneError> {
        Ok(())
    }

    fn start(
        &mut self,
        _event: ZoneStartEvent,
    ) -> Result<(), StartZoneError> {
        Ok(())
    }

    fn stop(
        &mut self,
        _event: ZoneStopEvent,
    ) -> Result<(), StopZoneError> {
        Ok(())
    }

    fn destroy(
        &mut self,
        _event: ZoneDestroyEvent,
    ) -> Result<(), DestroyZoneError> {
        Ok(())
    }
}*/
