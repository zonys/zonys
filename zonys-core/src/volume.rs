use crate::{
    DeserializeZoneTransmissionError, FileSystemIdentifierTryFromZoneIdentifierError, RawFdReader,
    RawFdWriter, SerializeZoneTransmissionError, Zone, ZoneConfigurationUnit,
    ZoneConfigurationVersion1VolumeUnit, ZoneTransmissionReader, ZoneTransmissionWriter,
};
use nix::errno::Errno;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_dir_all};
use std::io::{self, BufReader, BufWriter};
use std::num::TryFromIntError;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use tar::{Archive, Builder};
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
    OpenFileSystemSnapshotIteratorError(OpenFileSystemSnapshotIteratorError),
    DestroySnapshotError(DestroySnapshotError),
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

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneVolumeError {
    IOError(io::Error),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    NotExisting,
    SendFileSystemError(SendFileSystemError),
    PostcardError(postcard::Error),
    Errno(Errno),
    TryFromIntError(TryFromIntError),
    SerializeZoneTransmissionError(SerializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneVolumeError {
    IOError(io::Error),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    ReceiveFileSystemError(ReceiveFileSystemError),
    DeserializeZoneTransmissionError(DeserializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub enum ZoneVolumeType {
    Zfs,
    Directory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
enum ZoneVolumeTransmissionHeader {
    Version1 { r#type: ZoneVolumeType },
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
                for snapshot in file_system.snapshots().iter()? {
                    snapshot.destroy()?;
                }

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

    pub(super) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendZoneVolumeError> {
        match FileSystem::open(&FileSystemIdentifier::try_from(
            self.zone.identifier().clone(),
        )?)? {
            Some(mut file_system) => {
                writer.serialize(&ZoneVolumeTransmissionHeader::Version1 {
                    r#type: ZoneVolumeType::Zfs,
                })?;
                file_system.send(writer.as_raw_fd())?;
            }
            None => {
                writer.serialize(&ZoneVolumeTransmissionHeader::Version1 {
                    r#type: ZoneVolumeType::Directory,
                })?;

                let mut builder =
                    Builder::new(BufWriter::new(RawFdWriter::new(writer.as_raw_fd())));

                builder.follow_symlinks(false);
                builder.append_dir_all(".", &self.directory_path())?;
                builder.into_inner()?;
            }
        };

        Ok(())
    }

    pub(super) fn receive(
        &self,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<(), ReceiveZoneVolumeError> {
        let header = reader.deserialize::<ZoneVolumeTransmissionHeader>()?;

        let r#type = match header {
            ZoneVolumeTransmissionHeader::Version1 { r#type } => r#type,
        };

        match r#type {
            ZoneVolumeType::Zfs => {
                FileSystem::receive(
                    self.zone.identifier().clone().try_into()?,
                    reader.as_raw_fd(),
                )?;
            }
            ZoneVolumeType::Directory => {
                let mut archive =
                    Archive::new(BufReader::new(RawFdReader::new(reader.as_raw_fd())));

                archive.unpack(self.directory_path())?;
            }
        }

        Ok(())
    }
}
