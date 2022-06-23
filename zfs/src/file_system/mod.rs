pub mod error;
pub mod identifier;
pub mod iterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::snapshot::identifier::{SnapshotIdentifier, SnapshotIdentifierName};
use crate::snapshot::Snapshot;
use error::{
    CreateFileSystemError, CreateFileSystemSnapshotError, DestroyFileSystemError,
    MountFileSystemError, OpenFileSystemChildError, OpenFileSystemChildIteratorError,
    OpenFileSystemError, OpenFileSystemSnapshotError, OpenFileSystemSnapshotIteratorError,
    ReadFileSystemIdentifierError, ReadFileSystemMountStatusError, ReceiveFileSystemError,
    SendFileSystemError, UnmountAllFileSystemError, UnmountFileSystemError,
};
use identifier::{FileSystemIdentifier, FileSystemIdentifierComponent};
use iterator::{ChildFileSystemIterator, FileSystemSnapshotIterator};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use std::os::unix::prelude::RawFd;
use std::str::FromStr;
use zfs_sys::{
    libzfs_init, zfs_create, zfs_destroy, zfs_get_name, zfs_is_mounted, zfs_iter_children,
    zfs_iter_snapshots, zfs_mount, zfs_open, zfs_unmount, zfs_unmountall, ZfsHandle, ZfsType,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DEFAULT_RANDOM_NAME_LENGTH: usize = 16;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum FileSystemMountStatus {
    Mounted,
    Unmounted,
}

impl FileSystemMountStatus {
    pub fn is_mounted(&self) -> bool {
        match self {
            Self::Mounted => true,
            Self::Unmounted => false,
        }
    }

    pub fn is_unmounted(&self) -> bool {
        !self.is_mounted()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystem {
    handle: ZfsHandle,
    children: FileSystemChildren,
    snapshots: FileSystemSnapshots,
}

impl FileSystem {
    pub(crate) fn new(handle: ZfsHandle) -> Self {
        Self {
            handle: handle.clone(),
            children: FileSystemChildren::new(handle.clone()),
            snapshots: FileSystemSnapshots::new(handle),
        }
    }

    pub(crate) fn handle_mut(&mut self) -> &mut ZfsHandle {
        &mut self.handle
    }
}

impl FileSystem {
    pub fn children(&self) -> &FileSystemChildren {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut FileSystemChildren {
        &mut self.children
    }

    pub fn snapshots(&self) -> &FileSystemSnapshots {
        &self.snapshots
    }

    pub fn snapshots_mut(&mut self) -> &mut FileSystemSnapshots {
        &mut self.snapshots
    }
}

impl FileSystem {
    pub fn identifier(&self) -> Result<FileSystemIdentifier, ReadFileSystemIdentifierError> {
        Ok(FileSystemIdentifier::from_str(
            &zfs_get_name(&self.handle).map_err(ReadFileSystemIdentifierError::from)?,
        )?)
    }

    pub fn mount_status(&self) -> Result<FileSystemMountStatus, ReadFileSystemMountStatusError> {
        match zfs_is_mounted(&self.handle, None)? {
            true => Ok(FileSystemMountStatus::Mounted),
            false => Ok(FileSystemMountStatus::Unmounted),
        }
    }
}

impl FileSystem {
    pub fn mount(&mut self) -> Result<(), MountFileSystemError> {
        zfs_mount(&mut self.handle, None, 0)?;

        Ok(())
    }

    pub fn unmount(&mut self) -> Result<(), UnmountFileSystemError> {
        Ok(zfs_unmount(&mut self.handle, None, 0)?)
    }

    pub fn unmount_all(&mut self) -> Result<(), UnmountAllFileSystemError> {
        Ok(zfs_unmountall(&mut self.handle, 0)?)
    }

    pub fn destroy(self) -> Result<(), DestroyFileSystemError> {
        zfs_destroy(self.handle, false)?;

        Ok(())
    }

    pub fn create(identifier: &FileSystemIdentifier) -> Result<(), CreateFileSystemError> {
        zfs_create(
            &libzfs_init()?,
            &identifier.to_string(),
            ZfsType::FileSystem,
            None,
        )?;

        Ok(())
    }

    pub fn open(identifier: &FileSystemIdentifier) -> Result<Option<Self>, OpenFileSystemError> {
        Ok(zfs_open(
            &libzfs_init()?,
            &identifier.to_string(),
            ZfsType::FileSystem,
        )?
        .map(FileSystem::new))
    }

    pub fn send(&mut self, file_descriptor: RawFd) -> Result<(), SendFileSystemError> {
        let name = Alphanumeric.sample_string(&mut thread_rng(), DEFAULT_RANDOM_NAME_LENGTH);

        self.snapshots_mut().create(name.clone())?;
        let mut snapshot = self
            .snapshots_mut()
            .open(name)?
            .ok_or(SendFileSystemError::SnapshotMissing)?;
        let result = snapshot.send(file_descriptor);

        snapshot.destroy()?;

        Ok(result?)
    }

    pub fn receive(
        file_system_identifier: FileSystemIdentifier,
        file_descriptor: RawFd,
    ) -> Result<(), ReceiveFileSystemError> {
        let snapshot_identifier = SnapshotIdentifier::new(
            file_system_identifier,
            Alphanumeric.sample_string(&mut thread_rng(), DEFAULT_RANDOM_NAME_LENGTH),
        );

        Snapshot::receive(&snapshot_identifier, file_descriptor)?;
        let snapshot =
            Snapshot::open(&snapshot_identifier)?.ok_or(ReceiveFileSystemError::MissingSnapshot)?;

        snapshot.destroy()?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystemChildren {
    handle: ZfsHandle,
}

impl FileSystemChildren {
    fn new(handle: ZfsHandle) -> Self {
        Self { handle }
    }
}

impl FileSystemChildren {
    pub fn iter(&self) -> Result<ChildFileSystemIterator, OpenFileSystemChildIteratorError> {
        let mut result = Vec::new();

        zfs_iter_children(&self.handle, |handle| {
            result.push(FileSystem::new(handle));
            true
        })?;

        Ok(ChildFileSystemIterator::new(result.into_iter()))
    }

    pub fn open(
        &self,
        name: FileSystemIdentifierComponent,
    ) -> Result<Option<FileSystem>, OpenFileSystemChildError> {
        let mut identifier = FileSystem::new(self.handle.clone()).identifier()?;
        identifier.components_mut().push(name);
        FileSystem::open(&identifier).map_err(OpenFileSystemChildError::OpenFileSystemError)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystemSnapshots {
    handle: ZfsHandle,
}

impl FileSystemSnapshots {
    fn new(handle: ZfsHandle) -> Self {
        Self { handle }
    }

    fn file_system(&self) -> FileSystem {
        FileSystem::new(self.handle.clone())
    }
}

impl FileSystemSnapshots {
    pub fn iter(&self) -> Result<FileSystemSnapshotIterator, OpenFileSystemSnapshotIteratorError> {
        let mut result = Vec::new();

        zfs_iter_snapshots(&self.handle, |handle| {
            result.push(Snapshot::new(handle));
            true
        })?;

        Ok(FileSystemSnapshotIterator::new(result.into_iter()))
    }

    pub fn create(
        &mut self,
        name: SnapshotIdentifierName,
    ) -> Result<(), CreateFileSystemSnapshotError> {
        let identifier = SnapshotIdentifier::new(self.file_system().identifier()?, name);

        Ok(Snapshot::create(&identifier)?)
    }

    pub fn open(
        &self,
        name: SnapshotIdentifierName,
    ) -> Result<Option<Snapshot>, OpenFileSystemSnapshotError> {
        let identifier = SnapshotIdentifier::new(self.file_system().identifier()?, name);

        Ok(Snapshot::open(&identifier)?)
    }
}
