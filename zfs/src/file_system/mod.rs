pub mod error;
pub mod identifier;
pub mod iterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::snapshot::identifier::{SnapshotIdentifier, SnapshotIdentifierName};
use crate::snapshot::Snapshot;
use crate::{ZfsError, ZFS};

use error::{
    CreateFileSystemError, CreateFileSystemSnapshotError, DestroyFileSystemError,
    MountFileSystemError, OpenFileSystemChildError, OpenFileSystemChildIteratorError,
    OpenFileSystemError, OpenFileSystemSnapshotError, OpenFileSystemSnapshotIteratorError,
    ReadFileSystemIdentifierError, ReceiveFileSystemError, SendFileSystemError,
    UnmountAllFileSystemError, UnmountFileSystemError,
};
use identifier::{FileSystemIdentifier, FileSystemIdentifierComponent};
use iterator::{ChildFileSystemIterator, FileSystemSnapshotIterator};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use std::ffi::{c_int, c_void, CStr, CString};
use std::os::unix::prelude::RawFd;
use std::ptr::null_mut;
use std::str::FromStr;
use zfs_sys::{
    zfs_close, zfs_create, zfs_destroy, zfs_get_name, zfs_handle_t, zfs_is_mounted,
    zfs_iter_children, zfs_iter_snapshots, zfs_mount, zfs_open, zfs_type_t_ZFS_TYPE_FILESYSTEM,
    zfs_unmount, zfs_unmountall,
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
    handle: *mut zfs_handle_t,
}

impl Drop for FileSystem {
    fn drop(&mut self) {
        unsafe { zfs_close(self.handle) }
    }
}

impl FileSystem {
    pub(crate) fn new(handle: *mut zfs_handle_t) -> Self {
        Self { handle }
    }
}

impl FileSystem {
    pub fn children(&self) -> FileSystemChildren<&'_ Self> {
        FileSystemChildren::new(self)
    }

    pub fn children_mut(&mut self) -> FileSystemChildren<&'_ mut Self> {
        FileSystemChildren::new(self)
    }

    pub fn snapshots(&self) -> FileSystemSnapshots<&'_ Self> {
        FileSystemSnapshots::new(self)
    }

    pub fn snapshots_mut(&mut self) -> FileSystemSnapshots<&'_ mut Self> {
        FileSystemSnapshots::new(self)
    }
}

impl FileSystem {
    pub fn identifier(&self) -> Result<FileSystemIdentifier, ReadFileSystemIdentifierError> {
        let result = unsafe { CStr::from_ptr(zfs_get_name(self.handle)).to_str()? };

        Ok(FileSystemIdentifier::from_str(result)?)
    }

    pub fn mount_status(&self) -> FileSystemMountStatus {
        let result = unsafe { zfs_is_mounted(self.handle, null_mut()) };

        if result == 0 {
            return FileSystemMountStatus::Unmounted;
        }

        FileSystemMountStatus::Mounted
    }
}

impl FileSystem {
    pub fn mount(&mut self) -> Result<(), MountFileSystemError> {
        let result = unsafe { zfs_mount(self.handle, null_mut(), 0) };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(())
    }

    pub fn unmount(&mut self) -> Result<(), UnmountFileSystemError> {
        let result = unsafe { zfs_unmount(self.handle, null_mut(), 0) };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(())
    }

    pub fn unmount_all(&mut self) -> Result<(), UnmountAllFileSystemError> {
        let result = unsafe { zfs_unmountall(self.handle, 0) };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(())
    }

    pub fn destroy(self) -> Result<(), DestroyFileSystemError> {
        let result = unsafe { zfs_destroy(self.handle, 0) };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(())
    }

    pub fn create(identifier: &FileSystemIdentifier) -> Result<(), CreateFileSystemError> {
        ZFS.with::<_, Result<_, CreateFileSystemError>>(|zfs| {
            let string = CString::new(identifier.to_string())?;

            let result = unsafe {
                zfs_create(
                    **zfs,
                    string.as_ptr(),
                    zfs_type_t_ZFS_TYPE_FILESYSTEM,
                    null_mut(),
                )
            };

            if result != 0 {
                return Err(ZfsError::try_from(())?.into());
            }

            Ok(())
        })
    }

    pub fn open(identifier: &FileSystemIdentifier) -> Result<Option<Self>, OpenFileSystemError> {
        ZFS.with::<_, Result<_, OpenFileSystemError>>(|zfs| {
            let string = CString::new(identifier.to_string())?;

            let result = unsafe {
                zfs_open(
                    **zfs,
                    string.as_ptr(),
                    zfs_type_t_ZFS_TYPE_FILESYSTEM.try_into()?,
                )
            };

            if result.is_null() {
                return Ok(None);
            }

            Ok(Some(Self::new(result)))
        })
    }

    pub fn send(&mut self, file_descriptor: RawFd) -> Result<(), SendFileSystemError> {
        let name = Alphanumeric.sample_string(&mut thread_rng(), DEFAULT_RANDOM_NAME_LENGTH);

        self.snapshots_mut().create(name.clone())?;
        let mut snapshot = self
            .snapshots()
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
            Snapshot::open(&snapshot_identifier)?.ok_or(ReceiveFileSystemError::SnapshotMissing)?;

        snapshot.destroy()?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystemChildren<T> {
    file_system: T,
}

impl<T> FileSystemChildren<T> {
    fn new(file_system: T) -> Self {
        Self { file_system }
    }
}

impl<'a> FileSystemChildren<&'a FileSystem> {
    pub fn iter(&self) -> Result<ChildFileSystemIterator, OpenFileSystemChildIteratorError> {
        struct Data {
            file_systems: Vec<FileSystem>,
        }

        extern "C" fn handler(handle: *mut zfs_handle_t, data: *mut c_void) -> c_int {
            let data: &mut Data = unsafe { &mut *(data as *mut Data) };

            data.file_systems.push(FileSystem::new(handle));

            0
        }

        let mut data = Data {
            file_systems: Vec::default(),
        };

        let result = unsafe {
            zfs_iter_children(
                self.file_system.handle,
                0,
                Some(handler),
                &mut data as *mut _ as *mut c_void,
            )
        };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(ChildFileSystemIterator::new(data.file_systems.into_iter()))
    }

    pub fn open(
        &self,
        name: FileSystemIdentifierComponent,
    ) -> Result<Option<FileSystem>, OpenFileSystemChildError> {
        let mut identifier = self.file_system.identifier()?;
        identifier.components_mut().push(name);
        FileSystem::open(&identifier).map_err(OpenFileSystemChildError::OpenFileSystemError)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystemSnapshots<T> {
    file_system: T,
}

impl<T> FileSystemSnapshots<T> {
    fn new(file_system: T) -> Self {
        Self { file_system }
    }
}

impl<'a> FileSystemSnapshots<&'a FileSystem> {
    pub fn iter(&self) -> Result<FileSystemSnapshotIterator, OpenFileSystemSnapshotIteratorError> {
        struct Data {
            snapshots: Vec<Snapshot>,
        }

        extern "C" fn handler(handle: *mut zfs_handle_t, data: *mut c_void) -> c_int {
            let data: &mut Data = unsafe { &mut *(data as *mut Data) };

            data.snapshots.push(Snapshot::new(handle));

            0
        }

        let mut data = Data {
            snapshots: Vec::default(),
        };

        let result = unsafe {
            zfs_iter_snapshots(
                self.file_system.handle,
                0,
                Some(handler),
                &mut data as *mut _ as *mut c_void,
                0,
                0,
            )
        };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(FileSystemSnapshotIterator::new(data.snapshots.into_iter()))
    }

    pub fn open(
        &self,
        name: SnapshotIdentifierName,
    ) -> Result<Option<Snapshot>, OpenFileSystemSnapshotError> {
        let identifier = SnapshotIdentifier::new(self.file_system.identifier()?, name);

        Ok(Snapshot::open(&identifier)?)
    }
}

impl<'a> FileSystemSnapshots<&'a mut FileSystem> {
    pub fn create(
        &mut self,
        name: SnapshotIdentifierName,
    ) -> Result<(), CreateFileSystemSnapshotError> {
        let identifier = SnapshotIdentifier::new(self.file_system.identifier()?, name);

        Ok(Snapshot::create(&identifier)?)
    }
}
