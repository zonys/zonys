use crate::snapshot::error::{
    CreateSnapshotError, DestroySnapshotError, OpenSnapshotError, ReceiveSnapshotError,
    SendSnapshotError,
};
use crate::{TryIntoZfsError, ZfsError};
use std::ffi::NulError;
use std::num::TryFromIntError;
use std::str::Utf8Error;
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenFileSystemError {
    TryIntoZfsError(TryIntoZfsError),
    TryFromIntError(TryFromIntError),
    ZfsError(ZfsError),
    NulError(NulError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateFileSystemError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
    NulError(NulError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyFileSystemError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendFileSystemError {
    CreateFileSystemSnapshotError(CreateFileSystemSnapshotError),
    OpenFileSystemSnapshotError(OpenFileSystemSnapshotError),
    #[Display("Snapshot must exist")]
    SnapshotMissing,
    DestroySnapshotError(DestroySnapshotError),
    SendSnapshotError(SendSnapshotError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveFileSystemError {
    ReceiveSnapshotError(ReceiveSnapshotError),
    OpenSnapshotError(OpenSnapshotError),
    DestroySnapshotError(DestroySnapshotError),
    #[Display("Snapshot must exist")]
    SnapshotMissing,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
pub enum FromStrFileSystemIdentifierError {
    #[Display("Pool name must not be empty")]
    EmptyPoolName,
    #[Display("File system name must not be empty")]
    EmptyFileSystemName,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadFileSystemIdentifierError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
    Utf8Error(Utf8Error),
    FromStrFileSystemIdentifierError(FromStrFileSystemIdentifierError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenFileSystemChildError {
    OpenFileSystemError(OpenFileSystemError),
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenFileSystemSnapshotIteratorError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenFileSystemChildIteratorError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum MountFileSystemError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum UnmountFileSystemError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum UnmountAllFileSystemError {
    TryIntoZfsError(TryIntoZfsError),
    ZfsError(ZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateFileSystemSnapshotError {
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    CreateSnapshotError(CreateSnapshotError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenFileSystemSnapshotError {
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    OpenSnapshotError(OpenSnapshotError),
}
