use crate::{
    FileSystemIdentifierTryFromZoneIdentifierError, Zone, ZoneTransmissionReader,
    ZoneTransmissionWriter,
};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use zfs::file_system::error::{
    CreateFileSystemError, DestroyFileSystemError, MountFileSystemError, OpenFileSystemError,
    OpenFileSystemSnapshotIteratorError, ReceiveFileSystemError, SendFileSystemError,
    UnmountAllFileSystemError,
};
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::file_system::FileSystem;
use zfs::snapshot::error::DestroySnapshotError;
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CheckZoneZfsVolumeSupportError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    OpenFileSystemError(OpenFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenZoneZfsVolumeError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    OpenFileSystemError(OpenFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneZfsVolumeError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    #[Display("Created file system does not exist")]
    FileSystemNotExisting,
    OpenFileSystemError(OpenFileSystemError),
    CreateFileSystemError(CreateFileSystemError),
    MountFileSystemError(MountFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneZfsVolumeError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    OpenFileSystemError(OpenFileSystemError),
    #[Display("File system does not exist")]
    FileSystemNotExisting,
    DestroyFileSystemError(DestroyFileSystemError),
    DestroySnapshotError(DestroySnapshotError),
    OpenFileSystemSnapshotIteratorError(OpenFileSystemSnapshotIteratorError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneZfsVolumeError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    OpenFileSystemError(OpenFileSystemError),
    SendFileSystemError(SendFileSystemError),
    #[Display("File system does not exist")]
    FileSystemNotExisting,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneZfsVolumeError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    ReceiveFileSystemError(ReceiveFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneZfsVolumeError {
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    OpenFileSystemError(OpenFileSystemError),
    DestroyFileSystemError(DestroyFileSystemError),
    DestroySnapshotError(DestroySnapshotError),
    OpenFileSystemSnapshotIteratorError(OpenFileSystemSnapshotIteratorError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Constructor)]
#[Constructor(visibility = pub(crate))]
pub struct ZoneZfsVolume<T> {
    zone: T,
}

impl<'a> ZoneZfsVolume<&'a Zone> {
    pub fn root_directory_path(&self) -> PathBuf {
        self.zone.paths().root_directory()
    }

    pub(super) fn is_supported(zone: &'a Zone) -> Result<bool, CheckZoneZfsVolumeSupportError> {
        let mut file_system_identifier = FileSystemIdentifier::try_from(zone.identifier().clone())?;

        loop {
            file_system_identifier = file_system_identifier.parent();

            if FileSystem::open(&file_system_identifier)?.is_some() {
                return Ok(true);
            }

            if file_system_identifier.components().is_empty() {
                break;
            }
        }

        Ok(false)
    }

    pub(super) fn open(zone: &'a Zone) -> Result<Option<Self>, OpenZoneZfsVolumeError> {
        let file_system_identifier = FileSystemIdentifier::try_from(zone.identifier().clone())?;

        match FileSystem::open(&file_system_identifier)? {
            None => Ok(None),
            Some(_file_system) => Ok(Some(Self::new(zone))),
        }
    }

    pub(super) fn create(zone: &'a Zone) -> Result<(), CreateZoneZfsVolumeError> {
        let file_system_identifier = FileSystemIdentifier::try_from(zone.identifier().clone())?;
        FileSystem::create(&file_system_identifier)?;
        let mut file_system = FileSystem::open(&file_system_identifier)?
            .ok_or(CreateZoneZfsVolumeError::FileSystemNotExisting)?;
        file_system.mount()?;

        Ok(())
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyZoneZfsVolumeError> {
        let mut file_system = match FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            Some(file_system) => file_system,
            None => return Err(DestroyZoneZfsVolumeError::FileSystemNotExisting),
        };

        for snapshot in file_system.snapshots().iter()? {
            snapshot.destroy()?;
        }

        file_system.unmount_all()?;
        file_system.destroy()?;

        Ok(())
    }

    pub(super) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendZoneZfsVolumeError> {
        let mut file_system = match FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            Some(file_system) => file_system,
            None => return Err(SendZoneZfsVolumeError::FileSystemNotExisting),
        };

        file_system.send(writer.as_raw_fd())?;

        Ok(())
    }

    pub(super) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveZoneZfsVolumeError> {
        FileSystem::receive(zone.identifier().clone().try_into()?, reader.as_raw_fd())?;

        Ok(Self::new(zone))
    }

    pub(super) fn cleanup(&self) -> Result<(), CleanupZoneZfsVolumeError> {
        if let Some(mut file_system) = FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            file_system.unmount_all()?;
            file_system.destroy()?;
        };

        Ok(())
    }
}
