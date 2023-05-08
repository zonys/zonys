use crate::{
    CleanupZoneVolumeError, CreateZoneVolumeError, DestroyZoneVolumeError, FromHandlerError,
    ReadZoneConfigurationError, ReceiveZoneVolumeError, SendZoneVolumeError, Zone,
    ZoneTransmissionReader, ZoneTransmissionWriter, ZoneVolume,
};
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateChrootZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CreateZoneVolumeError(CreateZoneVolumeError),
    FromHandlerError(FromHandlerError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartChrootZoneError {
    Unknown,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopChrootZoneError {
    Unknown,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyChrootZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    DestroyZoneVolumeError(DestroyZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendChrootZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    SendZoneVolumeError(SendZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveChrootZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    ReceiveZoneVolumeError(ReceiveZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupChrootZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CleanupZoneVolumeError(CleanupZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ChrootZone<T> {
    _zone: T,
}

impl<'a> ChrootZone<&'a Zone> {
    pub(crate) fn volume(&self) -> Result<ZoneVolume<&Zone>, ReadZoneConfigurationError> {
        unimplemented!()
    }

    pub(crate) fn create(&self) -> Result<(), CreateChrootZoneError> {
        self.volume()?.create()?;

        Ok(())
    }

    pub(crate) fn start(&self) -> Result<(), StartChrootZoneError> {
        Ok(())
    }

    pub(crate) fn stop(&self) -> Result<(), StopChrootZoneError> {
        Ok(())
    }

    pub(crate) fn destroy(&self) -> Result<(), DestroyChrootZoneError> {
        self.volume()?.destroy()?;

        Ok(())
    }

    pub(crate) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendChrootZoneError> {
        self.volume()?.send(writer)?;

        Ok(())
    }

    pub(crate) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveChrootZoneError> {
        ZoneVolume::receive(zone, reader)?;

        Ok(Self::new(zone))
    }

    pub(crate) fn cleanup(&self) -> Result<(), CleanupChrootZoneError> {
        self.volume()?.cleanup()?;

        Ok(())
    }
}
