pub mod error;
pub mod identifier;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::file_system::FileSystem;
use crate::{ZfsError, ZFS};
use error::{
    CreateSnapshotError, DestroySnapshotError, OpenSnapshotError, OpenSnapshotFileSystemError,
    ReadSnapshotIdentifierError, ReceiveSnapshotError, SendSnapshotError,
};
use identifier::SnapshotIdentifier;
use std::ffi::{CStr, CString};
use std::os::unix::prelude::RawFd;
use std::ptr::{null, null_mut};
use std::str::FromStr;
use zfs_sys::{
    recvflags_t, sendflags_t, zfs_close, zfs_destroy, zfs_get_name, zfs_handle_t, zfs_open,
    zfs_receive, zfs_send_one, zfs_snapshot, zfs_type_t_ZFS_TYPE_SNAPSHOT,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Snapshot {
    handle: *mut zfs_handle_t,
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        unsafe { zfs_close(self.handle) }
    }
}

impl Snapshot {
    pub(crate) fn new(handle: *mut zfs_handle_t) -> Self {
        Self { handle }
    }
}

impl Snapshot {
    pub fn create(identifier: &SnapshotIdentifier) -> Result<(), CreateSnapshotError> {
        ZFS.with::<_, Result<_, CreateSnapshotError>>(|zfs| {
            let string = CString::new(identifier.to_string())?;

            let result = unsafe { zfs_snapshot(**zfs, string.as_ptr(), 0, null_mut()) };

            if result != 0 {
                return Err(ZfsError::try_from(())?.into());
            }

            Ok(())
        })
    }

    pub fn open(identifier: &SnapshotIdentifier) -> Result<Option<Self>, OpenSnapshotError> {
        ZFS.with::<_, Result<_, OpenSnapshotError>>(|zfs| {
            let string = CString::new(identifier.to_string())?;

            let result = unsafe {
                zfs_open(
                    **zfs,
                    string.as_ptr(),
                    zfs_type_t_ZFS_TYPE_SNAPSHOT.try_into()?,
                )
            };

            if result.is_null() {
                return Ok(None);
            }

            Ok(Some(Self::new(result)))
        })
    }
}

impl Snapshot {
    pub fn identifier(&self) -> Result<SnapshotIdentifier, ReadSnapshotIdentifierError> {
        let result = unsafe { CStr::from_ptr(zfs_get_name(self.handle)).to_str()? };

        Ok(SnapshotIdentifier::from_str(result)?)
    }

    pub fn file_system(&self) -> Result<FileSystem, OpenSnapshotFileSystemError> {
        let identifier = self.identifier()?;

        match FileSystem::open(identifier.file_system_identifier())? {
            None => Err(OpenSnapshotFileSystemError::FileSystemNotExisting),
            Some(f) => Ok(f),
        }
    }

    pub fn destroy(self) -> Result<(), DestroySnapshotError> {
        let result = unsafe { zfs_destroy(self.handle, 0) };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(())
    }

    pub fn send(&mut self, file_descriptor: RawFd) -> Result<(), SendSnapshotError> {
        let result = unsafe {
            zfs_send_one(
                self.handle,
                null(),
                file_descriptor,
                &mut sendflags_t {
                    verbosity: 0,
                    replicate: 0,
                    skipmissing: 0,
                    doall: 0,
                    fromorigin: 0,
                    pad: 0,
                    props: 0,
                    dryrun: 0,
                    progressastitle: 0,
                    parsable: 0,
                    progress: 0,
                    largeblock: 0,
                    embed_data: 0,
                    compress: 0,
                    raw: 0,
                    backup: 0,
                    holds: 0,
                    saved: 0,
                },
                null_mut(),
            )
        };

        if result != 0 {
            return Err(ZfsError::try_from(())?.into());
        }

        Ok(())
    }

    pub fn receive(
        identifier: &SnapshotIdentifier,
        file_descriptor: RawFd,
    ) -> Result<(), ReceiveSnapshotError> {
        ZFS.with::<_, Result<_, ReceiveSnapshotError>>(|zfs| {
            let string = CString::new(identifier.to_string())?;

            let result = unsafe {
                zfs_receive(
                    **zfs,
                    string.as_ptr(),
                    null_mut(),
                    &mut recvflags_t {
                        verbose: 0,
                        isprefix: 0,
                        istail: 0,
                        dryrun: 0,
                        force: 0,
                        canmountoff: 0,
                        resumable: 0,
                        byteswap: 0,
                        heal: 0,
                        nomount: 0,
                        holds: 0,
                        skipholds: 0,
                        domount: 0,
                        forceunmount: 0,
                    },
                    file_descriptor,
                    null_mut(),
                )
            };

            if result != 0 {
                return Err(ZfsError::try_from(())?.into());
            }

            Ok(())
        })
    }
}
