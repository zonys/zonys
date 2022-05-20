pub mod error;

////////////////////////////////////////////////////////////////////////////////////////////////////

use error::{CreateSnapshotError, DestroySnapshotError, ReadSnapshotNameError};
use zfs_sys::{libzfs_init, zfs_destroy, zfs_get_name, zfs_snapshot, ZfsHandle};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Snapshot {
    handle: ZfsHandle,
}

impl Snapshot {
    pub(crate) fn new(handle: ZfsHandle) -> Self {
        Self { handle }
    }
}

impl Snapshot {
    pub fn create(name: &str) -> Result<(), CreateSnapshotError> {
        zfs_snapshot(&mut libzfs_init()?, name, false, None)?;

        Ok(())
    }
}

impl Snapshot {
    pub fn name(&self) -> Result<String, ReadSnapshotNameError> {
        zfs_get_name(&self.handle).map_err(|e| e.into())
    }

    pub fn destroy(self) -> Result<(), DestroySnapshotError> {
        Ok(zfs_destroy(self.handle, false)?)
    }
}
