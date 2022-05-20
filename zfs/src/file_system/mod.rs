pub mod error;
pub mod iterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::snapshot::error::CreateSnapshotError;
use crate::snapshot::Snapshot;
use crate::{Error, Zfs, SEPARATOR};
use iterator::{
    AllFileSystemIterator, ChildFileSystemIterator, FileSystemSnapshotIterator,
    RootFileSystemIterator,
};
use zfs_sys::{
    libzfs_init, zfs_create, zfs_dataset_exists, zfs_destroy, zfs_get_name, zfs_is_mounted,
    zfs_iter_children, zfs_iter_root, zfs_iter_snapshots, zfs_mount, zfs_open, zfs_unmount,
    zfs_unmountall, LibzfsHandle, ZfsHandle, ZfsType,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystems {
    handle: LibzfsHandle,
}

impl FileSystems {
    pub(crate) fn new(handle: LibzfsHandle) -> Self {
        Self { handle }
    }

    pub fn create(&mut self, name: &str) -> Result<(), Error> {
        Ok(zfs_create(&self.handle, name, ZfsType::FileSystem, None)?)
    }

    pub fn open(&self, name: &str) -> Result<Option<FileSystem>, Error> {
        Ok(zfs_open(&self.handle, name, ZfsType::FileSystem)?.map(FileSystem::new))
    }

    pub fn exists(&self, name: &str) -> Result<bool, Error> {
        Ok(zfs_dataset_exists(
            &libzfs_init()?,
            name,
            ZfsType::FileSystem,
        )?)
    }

    pub fn roots(&self) -> Result<RootFileSystemIterator, Error> {
        let mut result = Vec::new();

        zfs_iter_root(&self.handle, |handle| {
            result.push(FileSystem::new(handle));
            true
        })?;

        Ok(RootFileSystemIterator::new(result.into_iter()))
    }

    pub fn all(&self) -> Result<AllFileSystemIterator, Error> {
        Ok(AllFileSystemIterator::default())
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
    pub fn is_mounted(&self) -> Result<bool, Error> {
        Ok(zfs_is_mounted(&self.handle, None)?)
    }

    pub fn unmount(&mut self) -> Result<(), Error> {
        Ok(zfs_unmount(&mut self.handle, None, 0)?)
    }

    pub fn unmount_all(&mut self) -> Result<(), Error> {
        Ok(zfs_unmountall(&mut self.handle, 0)?)
    }

    pub fn mount(&mut self) -> Result<(), Error> {
        Ok(zfs_mount(&mut self.handle, None, 0)?)
    }

    pub fn destroy(self) -> Result<(), Error> {
        Ok(zfs_destroy(self.handle, false)?)
    }

    pub fn name(&self) -> Result<String, Error> {
        zfs_get_name(&self.handle).map_err(|e| e.into())
    }

    pub fn exists(name: &str) -> Result<bool, Error> {
        Zfs::new()?.file_systems().exists(name)
    }

    pub fn create(name: &str) -> Result<(), Error> {
        Zfs::new()?.file_systems_mut().create(name)
    }

    pub fn open(name: &str) -> Result<Option<Self>, Error> {
        Zfs::new()?.file_systems().open(name)
    }

    pub fn roots() -> Result<RootFileSystemIterator, Error> {
        Zfs::new()?.file_systems().roots()
    }

    pub fn all() -> Result<AllFileSystemIterator, Error> {
        Zfs::new()?.file_systems().all()
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
    pub fn iter(&self) -> Result<ChildFileSystemIterator, Error> {
        let mut result = Vec::new();

        zfs_iter_children(&self.handle, |handle| {
            result.push(FileSystem::new(handle));
            true
        })?;

        Ok(ChildFileSystemIterator::new(result.into_iter()))
    }

    pub fn open(&self, name: &str) -> Result<Option<FileSystem>, Error> {
        let mut value = zfs_get_name(&self.handle).map_err(Error::from)?;
        value.push_str(SEPARATOR);
        value.push_str(name);

        FileSystem::open(&value)
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
}

impl FileSystemSnapshots {
    pub fn iter(&self) -> Result<FileSystemSnapshotIterator, Error> {
        let mut result = Vec::new();

        zfs_iter_snapshots(&self.handle, |handle| {
            result.push(Snapshot::new(handle));
            true
        })?;

        Ok(FileSystemSnapshotIterator::new(result.into_iter()))
    }

    pub fn create(&mut self, name: &str) -> Result<(), CreateSnapshotError> {
        Snapshot::create(&format!("{}@{}", zfs_get_name(&self.handle)?, name,))
    }
}
