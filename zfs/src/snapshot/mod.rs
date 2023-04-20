pub mod error;
pub mod identifier;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::file_system::FileSystem;
use error::{
    CreateSnapshotError, DestroySnapshotError, OpenSnapshotError, OpenSnapshotFileSystemError,
    ReadSnapshotIdentifierError, ReceiveSnapshotError, SendSnapshotError,
};
use identifier::SnapshotIdentifier;
use std::os::unix::prelude::RawFd;
use std::str::FromStr;
use zfs_sys::r#extern::recvflags_t;
use zfs_sys::r#extern::sendflags_t;
use zfs_sys::{
    libzfs_init, zfs_destroy, zfs_get_name, zfs_open, zfs_receive, zfs_send, zfs_snapshot,
    ZfsHandle, ZfsType,
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
    pub fn create(identifier: &SnapshotIdentifier) -> Result<(), CreateSnapshotError> {
        zfs_snapshot(&mut libzfs_init()?, &identifier.to_string(), false, None)?;

        Ok(())
    }

    pub fn open(name: &SnapshotIdentifier) -> Result<Option<Self>, OpenSnapshotError> {
        Ok(zfs_open(&libzfs_init()?, &name.to_string(), ZfsType::Snapshot)?.map(Self::new))
    }
}

impl Snapshot {
    pub fn identifier(&self) -> Result<SnapshotIdentifier, ReadSnapshotIdentifierError> {
        Ok(SnapshotIdentifier::from_str(
            &zfs_get_name(&self.handle).map_err(ReadSnapshotIdentifierError::from)?,
        )?)
    }

    pub fn file_system(&self) -> Result<FileSystem, OpenSnapshotFileSystemError> {
        let identifier = self.identifier()?;

        match FileSystem::open(&identifier.file_system_identifier())? {
            None => Err(OpenSnapshotFileSystemError::FileSystemNotExisting),
            Some(f) => Ok(f),
        }
    }

    pub fn destroy(self) -> Result<(), DestroySnapshotError> {
        Ok(zfs_destroy(self.handle, false)?)
    }

    pub fn send(&mut self, file_descriptor: RawFd) -> Result<(), SendSnapshotError> {
        zfs_send(
            self.file_system()?.handle_mut(),
            None,
            &self.identifier()?.name(),
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
            file_descriptor,
            None::<fn(_) -> bool>,
            None,
        )?;

        Ok(())
    }

    pub fn receive(
        identifier: &SnapshotIdentifier,
        file_descriptor: RawFd,
    ) -> Result<(), ReceiveSnapshotError> {
        zfs_receive(
            &mut libzfs_init()?,
            &identifier.to_string(),
            None,
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
            None,
        )?;

        Ok(())
    }
}
