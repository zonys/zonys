use crate::zone::{ConvertZoneIdentifierFromFileSystemIdentifierError, OpenZoneError};
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use zfs::file_system::error::{
    CreateFileSystemError, MountFileSystemError, OpenFileSystemChildIteratorError,
    OpenFileSystemError, ReadFileSystemIdentifierError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ConvertNamespaceIdentifierFromStrError {
    MissingRootComponent,
}

impl Debug for ConvertNamespaceIdentifierFromStrError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingRootComponent => write!(formatter, "Root component is missing"),
        }
    }
}

impl error::Error for ConvertNamespaceIdentifierFromStrError {}

impl Display for ConvertNamespaceIdentifierFromStrError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingRootComponent => write!(formatter, "Root component is missing"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenNamespaceError {
    OpenFileSystemError(OpenFileSystemError),
}

impl Debug for OpenNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for OpenNamespaceError {}

impl From<OpenFileSystemError> for OpenNamespaceError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenNamespaceZoneIteratorError {
    OpenFileSystemChildIteratorError(OpenFileSystemChildIteratorError),
}

impl error::Error for OpenNamespaceZoneIteratorError {}

impl Debug for OpenNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenFileSystemChildIteratorError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenFileSystemChildIteratorError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<OpenFileSystemChildIteratorError> for OpenNamespaceZoneIteratorError {
    fn from(error: OpenFileSystemChildIteratorError) -> Self {
        Self::OpenFileSystemChildIteratorError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NextNamespaceZoneIteratorError {
    ReadFileSystemIdentifierError(ReadFileSystemIdentifierError),
    ConvertZoneIdentifierFromFileSystemIdentifierError(
        ConvertZoneIdentifierFromFileSystemIdentifierError,
    ),
    OpenZoneError(OpenZoneError),
}

impl Debug for NextNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Debug::fmt(error, formatter),
            Self::ConvertZoneIdentifierFromFileSystemIdentifierError(error) => {
                Debug::fmt(error, formatter)
            }
            Self::OpenZoneError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for NextNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFileSystemIdentifierError(error) => Display::fmt(error, formatter),
            Self::ConvertZoneIdentifierFromFileSystemIdentifierError(error) => {
                Debug::fmt(error, formatter)
            }
            Self::OpenZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for NextNamespaceZoneIteratorError {}

impl From<ReadFileSystemIdentifierError> for NextNamespaceZoneIteratorError {
    fn from(error: ReadFileSystemIdentifierError) -> Self {
        Self::ReadFileSystemIdentifierError(error)
    }
}

impl From<ConvertZoneIdentifierFromFileSystemIdentifierError> for NextNamespaceZoneIteratorError {
    fn from(error: ConvertZoneIdentifierFromFileSystemIdentifierError) -> Self {
        Self::ConvertZoneIdentifierFromFileSystemIdentifierError(error)
    }
}

impl From<OpenZoneError> for NextNamespaceZoneIteratorError {
    fn from(error: OpenZoneError) -> Self {
        Self::OpenZoneError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateNamespaceError {
    CreateFileSystemError(CreateFileSystemError),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemNotExisting,
    MountFileSystemError(MountFileSystemError),
}

impl Debug for CreateNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::CreateFileSystemError(error) => Debug::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
            Self::MountFileSystemError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::CreateFileSystemError(error) => Display::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
            Self::MountFileSystemError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for CreateNamespaceError {}

impl From<CreateFileSystemError> for CreateNamespaceError {
    fn from(error: CreateFileSystemError) -> Self {
        Self::CreateFileSystemError(error)
    }
}

impl From<OpenFileSystemError> for CreateNamespaceError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

impl From<MountFileSystemError> for CreateNamespaceError {
    fn from(error: MountFileSystemError) -> Self {
        Self::MountFileSystemError(error)
    }
}
