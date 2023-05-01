use crate::{
    FileSystemIdentifierTryFromZoneIdentifierError, Zone, ZoneConfigurationUnit,
    ZoneConfigurationVersion1VolumeUnit,
};
use std::fs::{create_dir_all, remove_dir_all};
use std::io;
use std::path::PathBuf;
use zfs::file_system::error::{
    CreateFileSystemError, DestroyFileSystemError, MountFileSystemError, OpenFileSystemError,
    UnmountAllFileSystemError,
};
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::file_system::FileSystem;
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadZoneVolumeTypeError {
    OpenFileSystemError(OpenFileSystemError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneVolumeError {
    IOError(io::Error),
    #[Display("Created file system does not exist")]
    FileSystemNotExisting,
    CreateFileSystemError(CreateFileSystemError),
    MountFileSystemError(MountFileSystemError),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneVolumeError {
    IOError(io::Error),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    DestroyFileSystemError(DestroyFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneVolumeError {
    IOError(io::Error),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    DestroyFileSystemError(DestroyFileSystemError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ZoneVolumeType {
    Zfs,
    Directory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ZoneVolume<T> {
    zone: T,
}

impl ZoneVolume<&Zone> {
    pub fn directory_path(&self) -> PathBuf {
        self.zone.paths().root_directory()
    }

    pub fn r#type(&self) -> Result<Option<ZoneVolumeType>, ReadZoneVolumeTypeError> {
        match FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            Some(_file_system) => Ok(Some(ZoneVolumeType::Zfs)),
            None if self.directory_path().exists() => Ok(Some(ZoneVolumeType::Directory)),
            None => Ok(None),
        }
    }

    fn create_zfs(&self) -> Result<(), CreateZoneVolumeError> {
        let file_system_identifier =
            FileSystemIdentifier::try_from(self.zone.identifier().clone())?;
        FileSystem::create(&file_system_identifier)?;
        let mut file_system = FileSystem::open(&file_system_identifier)?
            .ok_or(CreateZoneVolumeError::FileSystemNotExisting)?;
        file_system.mount()?;

        Ok(())
    }

    fn create_directory(&self) -> Result<(), CreateZoneVolumeError> {
        create_dir_all(self.directory_path())?;

        Ok(())
    }

    pub(super) fn create(
        &self,
        configuration: &ZoneConfigurationUnit,
    ) -> Result<(), CreateZoneVolumeError> {
        let file_system = configuration
            .traverser()
            .inorder()
            .flat_map(|unit| unit.file_system())
            .next();

        match file_system {
            Some(ZoneConfigurationVersion1VolumeUnit::Automatic) | None => {
                let mut file_system_identifier =
                    FileSystemIdentifier::try_from(self.zone.identifier().clone())?;

                file_system_identifier.set_components(Vec::default());

                if FileSystem::open(&file_system_identifier)?.is_some() {
                    self.create_zfs()?;
                } else {
                    self.create_directory()?;
                }
            }
            Some(ZoneConfigurationVersion1VolumeUnit::Zfs) => {
                self.create_zfs()?;
            }
            Some(ZoneConfigurationVersion1VolumeUnit::Directory) => {
                self.create_directory()?;
            }
        }

        Ok(())
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyZoneVolumeError> {
        match FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            Some(mut file_system) => {
                file_system.unmount_all()?;
                file_system.destroy()?;
            }
            None => {
                remove_dir_all(self.directory_path())?;
            }
        };

        Ok(())
    }
    pub(super) fn cleanup(&self) -> Result<(), CleanupZoneVolumeError> {
        match FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            Some(mut file_system) => {
                file_system.unmount_all()?;
                file_system.destroy()?;
            }
            None => {
                let path = self.directory_path();
                if path.exists() {
                    remove_dir_all(path)?;
                }
            }
        };

        Ok(())
    }
}
