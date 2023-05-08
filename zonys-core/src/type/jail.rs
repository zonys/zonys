use crate::{
    CleanupZoneVolumeError, CreateZoneVolumeError, DestroyZoneVolumeError, FromHandler,
    FromHandlerError, JailZoneConfigurationVolumeType, ReadZoneConfigurationError,
    ReceiveZoneVolumeError, SendZoneVolumeError, Zone, ZoneConfigurationTypeReader,
    ZoneDirectoryVolume, ZoneTransmissionReader, ZoneTransmissionWriter, ZoneVolume, ZoneZfsVolume,
};
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CreateZoneVolumeError(CreateZoneVolumeError),
    FromHandlerError(FromHandlerError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartJailZoneError {
    Unknown,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopJailZoneError {
    Unknown,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    DestroyZoneVolumeError(DestroyZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    SendZoneVolumeError(SendZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    ReceiveZoneVolumeError(ReceiveZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CleanupZoneVolumeError(CleanupZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct JailZone<T> {
    zone: T,
}

impl<'a> JailZone<&'a Zone> {
    pub(crate) fn volume(&self) -> Result<ZoneVolume<&Zone>, ReadZoneConfigurationError> {
        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        match jail.volume() {
            JailZoneConfigurationVolumeType::Automatic => {
                todo!()
            }
            JailZoneConfigurationVolumeType::Zfs => {
                Ok(ZoneVolume::Zfs(ZoneZfsVolume::new(self.zone)))
            }
            JailZoneConfigurationVolumeType::Directory => {
                Ok(ZoneVolume::Directory(ZoneDirectoryVolume::new(self.zone)))
            }
        }
    }

    pub(crate) fn create(&self) -> Result<(), CreateJailZoneError> {
        let volume = self.volume()?;
        volume.create()?;

        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        if let Some(from) = jail.from() {
            FromHandler::handle(from, &volume.root_directory_path())?;
        }

        Ok(())
    }

    pub(crate) fn start(&self) -> Result<(), StartJailZoneError> {
        Ok(())
    }

    pub(crate) fn stop(&self) -> Result<(), StopJailZoneError> {
        Ok(())
    }

    pub(crate) fn destroy(&self) -> Result<(), DestroyJailZoneError> {
        self.volume()?.destroy()?;

        Ok(())
    }

    pub(crate) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendJailZoneError> {
        self.volume()?.send(writer)?;

        Ok(())
    }

    pub(crate) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveJailZoneError> {
        ZoneVolume::receive(zone, reader)?;

        Ok(Self::new(zone))
    }

    pub(crate) fn cleanup(&self) -> Result<(), CleanupJailZoneError> {
        self.volume()?.cleanup()?;

        Ok(())
    }
}
