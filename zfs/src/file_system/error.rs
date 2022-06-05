use crate::snapshot::error::{
    CreateSnapshotError, DestroySnapshotError, OpenSnapshotError, ReceiveSnapshotError,
    SendSnapshotError,
};
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenFileSystemError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for OpenFileSystemError {}

impl Debug for OpenFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for OpenFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateFileSystemError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for CreateFileSystemError {}

impl Debug for CreateFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for CreateFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum DestroyFileSystemError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for DestroyFileSystemError {}

impl Debug for DestroyFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for DestroyFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for DestroyFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum SendFileSystemError {
    CreateFileSystemSnapshotError(CreateFileSystemSnapshotError),
    OpenFileSystemSnapshotError(OpenFileSystemSnapshotError),
    SnapshotMissing,
    DestroySnapshotError(DestroySnapshotError),
    SendSnapshotError(SendSnapshotError),
}

impl error::Error for SendFileSystemError {}

impl Debug for SendFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::CreateFileSystemSnapshotError(error) => Debug::fmt(error, formatter),
            Self::OpenFileSystemSnapshotError(error) => Debug::fmt(error, formatter),
            Self::SnapshotMissing => write!(formatter, "Snapshot is not existing"),
            Self::DestroySnapshotError(error) => Debug::fmt(error, formatter),
            Self::SendSnapshotError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for SendFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::CreateFileSystemSnapshotError(error) => Display::fmt(error, formatter),
            Self::OpenFileSystemSnapshotError(error) => Display::fmt(error, formatter),
            Self::SnapshotMissing => write!(formatter, "Snapshot is not existing"),
            Self::DestroySnapshotError(error) => Display::fmt(error, formatter),
            Self::SendSnapshotError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<CreateFileSystemSnapshotError> for SendFileSystemError {
    fn from(error: CreateFileSystemSnapshotError) -> Self {
        Self::CreateFileSystemSnapshotError(error)
    }
}

impl From<OpenFileSystemSnapshotError> for SendFileSystemError {
    fn from(error: OpenFileSystemSnapshotError) -> Self {
        Self::OpenFileSystemSnapshotError(error)
    }
}

impl From<DestroySnapshotError> for SendFileSystemError {
    fn from(error: DestroySnapshotError) -> Self {
        Self::DestroySnapshotError(error)
    }
}

impl From<SendSnapshotError> for SendFileSystemError {
    fn from(error: SendSnapshotError) -> Self {
        Self::SendSnapshotError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReceiveFileSystemError {
    ReceiveSnapshotError(ReceiveSnapshotError),
    OpenSnapshotError(OpenSnapshotError),
    DestroySnapshotError(DestroySnapshotError),
    MissingSnapshot,
}

impl error::Error for ReceiveFileSystemError {}

impl Debug for ReceiveFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReceiveSnapshotError(error) => Debug::fmt(error, formatter),
            Self::OpenSnapshotError(error) => Debug::fmt(error, formatter),
            Self::DestroySnapshotError(error) => Debug::fmt(error, formatter),
            Self::MissingSnapshot => write!(formatter, "Snapshot is not existing"),
        }
    }
}

impl Display for ReceiveFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReceiveSnapshotError(error) => Display::fmt(error, formatter),
            Self::OpenSnapshotError(error) => Display::fmt(error, formatter),
            Self::DestroySnapshotError(error) => Display::fmt(error, formatter),
            Self::MissingSnapshot => write!(formatter, "Snapshot is not existing"),
        }
    }
}

impl From<ReceiveSnapshotError> for ReceiveFileSystemError {
    fn from(error: ReceiveSnapshotError) -> Self {
        Self::ReceiveSnapshotError(error)
    }
}

impl From<OpenSnapshotError> for ReceiveFileSystemError {
    fn from(error: OpenSnapshotError) -> Self {
        Self::OpenSnapshotError(error)
    }
}

impl From<DestroySnapshotError> for ReceiveFileSystemError {
    fn from(error: DestroySnapshotError) -> Self {
        Self::DestroySnapshotError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum FromStrFileSystemIdentifierError {
    EmptyPoolName,
    EmptyFileSystemName,
}

impl error::Error for FromStrFileSystemIdentifierError {}

impl Debug for FromStrFileSystemIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::EmptyPoolName => write!(formatter, "Pool name is empty"),
            Self::EmptyFileSystemName => write!(formatter, "File system name is empty"),
        }
    }
}

impl Display for FromStrFileSystemIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::EmptyPoolName => write!(formatter, "Pool name is empty"),
            Self::EmptyFileSystemName => write!(formatter, "File system name is empty"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReadFileSystemIdentifierError {
    ZfsError(zfs_sys::Error),
    FromStrFileSystemIdentifierError(FromStrFileSystemIdentifierError),
}

impl error::Error for ReadFileSystemIdentifierError {}

impl Debug for ReadFileSystemIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::FromStrFileSystemIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReadFileSystemIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::FromStrFileSystemIdentifierError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for ReadFileSystemIdentifierError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<FromStrFileSystemIdentifierError> for ReadFileSystemIdentifierError {
    fn from(error: FromStrFileSystemIdentifierError) -> Self {
        Self::FromStrFileSystemIdentifierError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenFileSystemChildError {
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    OpenFileSystemError(OpenFileSystemError),
}

impl error::Error for OpenFileSystemChildError {}

impl Debug for OpenFileSystemChildError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Debug::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenFileSystemChildError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Display::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<ReadFileSystemIdentifierError> for OpenFileSystemChildError {
    fn from(error: ReadFileSystemIdentifierError) -> Self {
        Self::ReadFileSystemIdentifierError(error)
    }
}

impl From<OpenFileSystemError> for OpenFileSystemChildError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReadFileSystemMountStatusError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for ReadFileSystemMountStatusError {}

impl Debug for ReadFileSystemMountStatusError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReadFileSystemMountStatusError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for ReadFileSystemMountStatusError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenFileSystemSnapshotIteratorError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for OpenFileSystemSnapshotIteratorError {}

impl Debug for OpenFileSystemSnapshotIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenFileSystemSnapshotIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for OpenFileSystemSnapshotIteratorError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenFileSystemChildIteratorError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for OpenFileSystemChildIteratorError {}

impl Debug for OpenFileSystemChildIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenFileSystemChildIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for OpenFileSystemChildIteratorError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum MountFileSystemError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for MountFileSystemError {}

impl Debug for MountFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for MountFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for MountFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum UnmountFileSystemError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for UnmountFileSystemError {}

impl Debug for UnmountFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for UnmountFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for UnmountFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum UnmountAllFileSystemError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for UnmountAllFileSystemError {}

impl Debug for UnmountAllFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for UnmountAllFileSystemError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for UnmountAllFileSystemError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateFileSystemSnapshotError {
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    CreateSnapshotError(CreateSnapshotError),
}

impl error::Error for CreateFileSystemSnapshotError {}

impl Debug for CreateFileSystemSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Debug::fmt(error, formatter),
            Self::CreateSnapshotError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateFileSystemSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Display::fmt(error, formatter),
            Self::CreateSnapshotError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<ReadFileSystemIdentifierError> for CreateFileSystemSnapshotError {
    fn from(error: ReadFileSystemIdentifierError) -> Self {
        Self::ReadFileSystemIdentifierError(error)
    }
}

impl From<CreateSnapshotError> for CreateFileSystemSnapshotError {
    fn from(error: CreateSnapshotError) -> Self {
        Self::CreateSnapshotError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenFileSystemSnapshotError {
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    OpenSnapshotError(OpenSnapshotError),
}

impl error::Error for OpenFileSystemSnapshotError {}

impl Debug for OpenFileSystemSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Debug::fmt(error, formatter),
            Self::OpenSnapshotError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenFileSystemSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Display::fmt(error, formatter),
            Self::OpenSnapshotError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<ReadFileSystemIdentifierError> for OpenFileSystemSnapshotError {
    fn from(error: ReadFileSystemIdentifierError) -> Self {
        Self::ReadFileSystemIdentifierError(error)
    }
}

impl From<OpenSnapshotError> for OpenFileSystemSnapshotError {
    fn from(error: OpenSnapshotError) -> Self {
        Self::OpenSnapshotError(error)
    }
}
