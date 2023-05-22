mod directory;
#[cfg(target_os = "freebsd")]
mod zfs;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(target_os = "freebsd")]
pub use crate::volume::zfs::*;
pub use directory::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    DeserializeZoneTransmissionError, SerializeZoneTransmissionError, Zone, ZoneTransmissionReader,
    ZoneTransmissionWriter,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneVolumeError {
    CheckZoneZfsVolumeSupportError(CheckZoneZfsVolumeSupportError),
    CreateZoneDirectoryVolumeError(CreateZoneDirectoryVolumeError),
    #[cfg(target_os = "freebsd")]
    CreateZoneZfsVolumeError(CreateZoneZfsVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenZoneVolumeError {
    OpenZoneZfsVolumeError(OpenZoneZfsVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneVolumeError {
    DestroyZoneDirectoryVolumeError(DestroyZoneDirectoryVolumeError),
    #[cfg(target_os = "freebsd")]
    DestroyZoneZfsVolumeError(DestroyZoneZfsVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneVolumeError {
    SendZoneDirectoryVolumeError(SendZoneDirectoryVolumeError),
    #[cfg(target_os = "freebsd")]
    SendZoneZfsVolumeError(SendZoneZfsVolumeError),
    SerializeZoneTransmissionError(SerializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneVolumeError {
    ReceiveZoneDirectoryVolumeError(ReceiveZoneDirectoryVolumeError),
    #[cfg(target_os = "freebsd")]
    ReceiveZoneZfsVolumeError(ReceiveZoneZfsVolumeError),
    DeserializeZoneTransmissionError(DeserializeZoneTransmissionError),
    #[Display("Unsupported transmission type ({value})")]
    UnsupportedTransmissionType(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneVolumeError {
    CleanupZoneDirectoryVolumeError(CleanupZoneDirectoryVolumeError),
    #[cfg(target_os = "freebsd")]
    CleanupZoneZfsVolumeError(CleanupZoneZfsVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ZoneVolumeType {
    Automatic,
    Directory,
    Zfs,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Deserialize, Serialize)]
pub enum ZoneVolumeTransmissionVersion1Type {
    #[Display("ZFS")]
    Zfs,
    #[Display("Directory")]
    Directory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
enum ZoneVolumeTransmissionHeader {
    Version1 {
        r#type: ZoneVolumeTransmissionVersion1Type,
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ZoneVolume<T> {
    Directory(ZoneDirectoryVolume<T>),
    #[cfg(target_os = "freebsd")]
    Zfs(ZoneZfsVolume<T>),
}

impl<'a> ZoneVolume<&'a Zone> {
    pub fn root_directory_path(&self) -> PathBuf {
        match self {
            Self::Directory(directory) => directory.root_directory_path(),
            #[cfg(target_os = "freebsd")]
            Self::Zfs(zfs) => zfs.root_directory_path(),
        }
    }

    pub(crate) fn open(zone: &'a Zone) -> Result<Option<Self>, OpenZoneVolumeError> {
        match ZoneZfsVolume::open(zone)? {
            None => {}
            Some(volume) => return Ok(Some(Self::Zfs(volume))),
        };

        match ZoneDirectoryVolume::open(zone) {
            None => {}
            Some(volume) => return Ok(Some(Self::Directory(volume))),
        }

        Ok(None)
    }

    pub(crate) fn create(
        zone: &'a Zone,
        r#type: ZoneVolumeType,
    ) -> Result<(), CreateZoneVolumeError> {
        match r#type {
            ZoneVolumeType::Automatic => {
                if ZoneZfsVolume::is_supported(zone)? {
                    ZoneZfsVolume::create(zone)?;
                } else {
                    ZoneDirectoryVolume::create(zone)?;
                }
            }
            ZoneVolumeType::Directory => {
                ZoneDirectoryVolume::create(zone)?;
            }
            ZoneVolumeType::Zfs => {
                ZoneZfsVolume::create(zone)?;
            }
        }

        Ok(())
    }

    pub(crate) fn destroy(&self) -> Result<(), DestroyZoneVolumeError> {
        match self {
            Self::Directory(directory) => Ok(directory.destroy()?),
            #[cfg(target_os = "freebsd")]
            Self::Zfs(zfs) => Ok(zfs.destroy()?),
        }
    }

    pub(crate) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendZoneVolumeError> {
        match self {
            Self::Directory(directory) => {
                writer.serialize(&ZoneVolumeTransmissionHeader::Version1 {
                    r#type: ZoneVolumeTransmissionVersion1Type::Directory,
                })?;

                Ok(directory.send(writer)?)
            }
            #[cfg(target_os = "freebsd")]
            Self::Zfs(zfs) => {
                writer.serialize(&ZoneVolumeTransmissionHeader::Version1 {
                    r#type: ZoneVolumeTransmissionVersion1Type::Zfs,
                })?;

                Ok(zfs.send(writer)?)
            }
        }
    }

    pub(crate) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveZoneVolumeError> {
        let header = reader.deserialize::<ZoneVolumeTransmissionHeader>()?;

        let r#type = match header {
            ZoneVolumeTransmissionHeader::Version1 { r#type } => r#type,
        };

        match r#type {
            #[cfg(target_os = "freebsd")]
            ZoneVolumeTransmissionVersion1Type::Zfs => {
                Ok(Self::Zfs(ZoneZfsVolume::receive(zone, reader)?))
            }
            #[cfg(not(target_os = "freebsd"))]
            ZoneVolumeTransmissionVersion1Type::Zfs => Err(
                ReceiveZoneVolumeError::UnsupportedTransmissionType(String::from("ZFS")),
            ),
            ZoneVolumeTransmissionVersion1Type::Directory => {
                Ok(Self::Directory(ZoneDirectoryVolume::receive(zone, reader)?))
            }
        }
    }

    pub(crate) fn cleanup(&self) -> Result<(), CleanupZoneVolumeError> {
        match self {
            Self::Directory(directory) => Ok(directory.cleanup()?),
            #[cfg(target_os = "freebsd")]
            Self::Zfs(zfs) => Ok(zfs.cleanup()?),
        }
    }
}
