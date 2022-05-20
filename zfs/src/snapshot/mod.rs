pub mod error;

////////////////////////////////////////////////////////////////////////////////////////////////////

use error::{
    CreateSnapshotError, DestroySnapshotError, OpenSnapshotError, ReadSnapshotNameError,
    SendSnapshotError,
};
use std::os::unix::prelude::RawFd;
use zfs_sys::r#extern::sendflags_t;
use zfs_sys::{
    libzfs_init, zfs_destroy, zfs_get_name, zfs_open, zfs_send_one, zfs_snapshot, ZfsHandle,
    ZfsType,
};

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

    pub fn open(name: &str) -> Result<Option<Self>, OpenSnapshotError> {
        Ok(zfs_open(&libzfs_init()?, name, ZfsType::Snapshot)?.map(Self::new))
    }
}

impl Snapshot {
    pub fn name(&self) -> Result<String, ReadSnapshotNameError> {
        zfs_get_name(&self.handle).map_err(|e| e.into())
    }

    pub fn destroy(self) -> Result<(), DestroySnapshotError> {
        Ok(zfs_destroy(self.handle, false)?)
    }

    pub fn send(&mut self, file_descriptor: RawFd) -> Result<(), SendSnapshotError> {
        zfs_send_one(
            &mut self.handle,
            "",
            file_descriptor,
            &mut sendflags_t {
                verbosity: 0,
                replicate: false,
                skipmissing: false,
                doall: false,
                fromorigin: false,
                pad: false,
                props: false,
                dryrun: false,
                parsable: false,
                progress: false,
                largeblock: false,
                embed_data: false,
                compress: false,
                raw: false,
                backup: false,
                holds: false,
                saved: false,
            },
            None,
        )?;

        Ok(())
    }
}
