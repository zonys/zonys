use crate::file_system::error::{FromStrFileSystemIdentifierError, OpenFileSystemError};
use crate::{TryIntoZfsError, ZfsError};
use std::ffi::NulError;
use std::num::TryFromIntError;
use std::str::Utf8Error;
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateSnapshotError {
    NulError(NulError),
    ZfsError(ZfsError),
    TryIntoZfsError(TryIntoZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroySnapshotError {
    ZfsError(ZfsError),
    TryIntoZfsError(TryIntoZfsError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendSnapshotError {
    ZfsError(ZfsError),
    TryIntoZfsError(TryIntoZfsError),
    NulError(NulError),
    ReadSnapshotIdentifierError(ReadSnapshotIdentifierError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveSnapshotError {
    ZfsError(ZfsError),
    TryIntoZfsError(TryIntoZfsError),
    NulError(NulError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenSnapshotError {
    NulError(NulError),
    TryFromIntError(TryFromIntError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenSnapshotFileSystemError {
    OpenFileSystemError(OpenFileSystemError),
    #[Display("File system must exist")]
    FileSystemNotExisting,
    ReadSnapshotIdentifierError(ReadSnapshotIdentifierError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadSnapshotIdentifierError {
    FromStrSnapshotIdentifierError(FromStrSnapshotIdentifierError),
    Utf8Error(Utf8Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum FromStrSnapshotIdentifierError {
    #[Display("File system identifier must not be empty")]
    EmptyFileSystemIdentifier,
    #[Display("Snapshot name must not be empty")]
    EmptySnapshotName,
    FromStrFileSystemIdentifierError(FromStrFileSystemIdentifierError),
}
