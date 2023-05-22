use crate::{
    CleanupZoneVolumeError, CreateZoneVolumeError, DestroyZoneVolumeError, FromHandlerError,
    OpenZoneVolumeError, ReadZoneConfigurationError, ReceiveZoneVolumeError, SendZoneVolumeError,
    Zone, ZoneTransmissionReader, ZoneTransmissionWriter, ZoneVolume,
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
    OpenZoneVolumeError(OpenZoneVolumeError),
    #[Display("Volume does not exist")]
    VolumeNotExisting,
    DestroyZoneVolumeError(DestroyZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendChrootZoneError {
    OpenZoneVolumeError(OpenZoneVolumeError),
    #[Display("Volume does not exist")]
    VolumeNotExisting,
    SendZoneVolumeError(SendZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveChrootZoneError {
    ReceiveZoneVolumeError(ReceiveZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupChrootZoneError {
    CleanupZoneVolumeError(CleanupZoneVolumeError),
    OpenZoneVolumeError(OpenZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ChrootZone<T> {
    zone: T,
}

impl<'a> ChrootZone<&'a Zone> {
    pub(super) fn volume(&self) -> Result<Option<ZoneVolume<&Zone>>, OpenZoneVolumeError> {
        ZoneVolume::open(self.zone)
    }

    pub(crate) fn create(&self) -> Result<(), CreateChrootZoneError> {
        Ok(())
    }

    pub(crate) fn start(&self) -> Result<(), StartChrootZoneError> {
        Ok(())
    }

    pub(crate) fn stop(&self) -> Result<(), StopChrootZoneError> {
        Ok(())
    }

    pub(crate) fn destroy(&self) -> Result<(), DestroyChrootZoneError> {
        let volume = match self.volume()? {
            None => return Err(DestroyChrootZoneError::VolumeNotExisting),
            Some(volume) => volume,
        };

        volume.destroy()?;

        Ok(())
    }

    pub(crate) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendChrootZoneError> {
        let volume = match self.volume()? {
            None => return Err(SendChrootZoneError::VolumeNotExisting),
            Some(volume) => volume,
        };

        volume.send(writer)?;

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
        if let Some(volume) = self.volume()? {
            volume.cleanup()?;
        }

        Ok(())
    }
}
