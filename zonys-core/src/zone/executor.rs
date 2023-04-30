use crate::{ReadZoneConfigurationError, Zone};
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum TriggerZoneExecutorCreateError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum TriggerZoneExecutorStartError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum TriggerZoneExecutorStopError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum TriggerZoneExecutorDestroyError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Constructor)]
#[Constructor(visibility = pub(super))]
pub(super) struct ZoneExecutor<T> {
    zone: T,
}

impl ZoneExecutor<&Zone> {
    pub(super) fn trigger_create(&self) -> Result<(), TriggerZoneExecutorCreateError> {
        let _configuration = self.zone.configuration().unit()?;

        Ok(())
    }

    pub(super) fn trigger_start(&self) -> Result<(), TriggerZoneExecutorStartError> {
        let _configuration = self.zone.configuration().unit()?;

        Ok(())
    }

    pub(super) fn trigger_stop(&self) -> Result<(), TriggerZoneExecutorStopError> {
        let _configuration = self.zone.configuration().unit()?;

        Ok(())
    }

    pub(super) fn trigger_destroy(&self) -> Result<(), TriggerZoneExecutorDestroyError> {
        let _configuration = self.zone.configuration().unit()?;

        Ok(())
    }
}
