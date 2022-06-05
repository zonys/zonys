use crate::file_system::error::{FromStrFileSystemIdentifierError, OpenFileSystemError};
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateSnapshotError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for CreateSnapshotError {}

impl Debug for CreateSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for CreateSnapshotError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum DestroySnapshotError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for DestroySnapshotError {}

impl Debug for DestroySnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for DestroySnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for DestroySnapshotError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum SendSnapshotError {
    ZfsError(zfs_sys::Error),
    OpenSnapshotFileSystemError(OpenSnapshotFileSystemError),
    ReadSnapshotIdentifierError(ReadSnapshotIdentifierError),
}

impl error::Error for SendSnapshotError {}

impl Debug for SendSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::OpenSnapshotFileSystemError(error) => Debug::fmt(error, formatter),
            Self::ReadSnapshotIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for SendSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::OpenSnapshotFileSystemError(error) => Display::fmt(error, formatter),
            Self::ReadSnapshotIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for SendSnapshotError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<OpenSnapshotFileSystemError> for SendSnapshotError {
    fn from(error: OpenSnapshotFileSystemError) -> Self {
        Self::OpenSnapshotFileSystemError(error)
    }
}

impl From<ReadSnapshotIdentifierError> for SendSnapshotError {
    fn from(error: ReadSnapshotIdentifierError) -> Self {
        Self::ReadSnapshotIdentifierError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReceiveSnapshotError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for ReceiveSnapshotError {}

impl Debug for ReceiveSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReceiveSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for ReceiveSnapshotError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenSnapshotError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for OpenSnapshotError {}

impl Debug for OpenSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for OpenSnapshotError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenSnapshotFileSystemError {
    ZfsError(zfs_sys::Error),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemNotExisting,
    ReadSnapshotIdentifierError(ReadSnapshotIdentifierError),
}

impl error::Error for OpenSnapshotFileSystemError {}

impl Debug for OpenSnapshotFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
            Self::FileSystemNotExisting => {
                write!(formatter, "File system does not exist")
            }
            Self::ReadSnapshotIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenSnapshotFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
            Self::FileSystemNotExisting => {
                write!(formatter, "File system does not exist")
            }
            Self::ReadSnapshotIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for OpenSnapshotFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<OpenFileSystemError> for OpenSnapshotFileSystemError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

impl From<ReadSnapshotIdentifierError> for OpenSnapshotFileSystemError {
    fn from(error: ReadSnapshotIdentifierError) -> Self {
        Self::ReadSnapshotIdentifierError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReadSnapshotIdentifierError {
    ZfsError(zfs_sys::Error),
    FromStrSnapshotIdentifierError(FromStrSnapshotIdentifierError),
}

impl error::Error for ReadSnapshotIdentifierError {}

impl Debug for ReadSnapshotIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::FromStrSnapshotIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReadSnapshotIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::FromStrSnapshotIdentifierError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for ReadSnapshotIdentifierError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<FromStrSnapshotIdentifierError> for ReadSnapshotIdentifierError {
    fn from(error: FromStrSnapshotIdentifierError) -> Self {
        Self::FromStrSnapshotIdentifierError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum FromStrSnapshotIdentifierError {
    EmptyFileSystemIdentifier,
    EmptySnapshotName,
    FromStrFileSystemIdentifierError(FromStrFileSystemIdentifierError),
}

impl error::Error for FromStrSnapshotIdentifierError {}

impl Debug for FromStrSnapshotIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::EmptyFileSystemIdentifier => write!(formatter, "File system identifier is empty"),
            Self::EmptySnapshotName => write!(formatter, "Snapshot name is empty"),
            Self::FromStrFileSystemIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for FromStrSnapshotIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::EmptyFileSystemIdentifier => write!(formatter, "File system identifier is empty"),
            Self::EmptySnapshotName => write!(formatter, "Snapshot name is empty"),
            Self::FromStrFileSystemIdentifierError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<FromStrFileSystemIdentifierError> for FromStrSnapshotIdentifierError {
    fn from(error: FromStrFileSystemIdentifierError) -> Self {
        Self::FromStrFileSystemIdentifierError(error)
    }
}
