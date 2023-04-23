use crate::zone::{OpenZoneConfigurationError, OpenZoneError, ParseZoneIdentifierError};
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::path::StripPrefixError;
use zfs::file_system::error::{CreateFileSystemError, MountFileSystemError, OpenFileSystemError};
use ztd::{Display, Error, From};

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

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenNamespaceZoneIteratorError {
    IoError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum NextNamespaceZoneIteratorError {
    OpenZoneError(OpenZoneError),
    IoError(io::Error),
    ParseZoneIdentifierError(ParseZoneIdentifierError),
    StripPrefixError(StripPrefixError),
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

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenNamespaceMatchZoneIteratorError {
    OpenNamespaceZoneIteratorError(OpenNamespaceZoneIteratorError),
    RegexError(regex::Error),
}

impl error::Error for OpenNamespaceMatchZoneIteratorError {}

impl Debug for OpenNamespaceMatchZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenNamespaceZoneIteratorError(error) => Debug::fmt(error, formatter),
            Self::RegexError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenNamespaceMatchZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenNamespaceZoneIteratorError(error) => Display::fmt(error, formatter),
            Self::RegexError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<OpenNamespaceZoneIteratorError> for OpenNamespaceMatchZoneIteratorError {
    fn from(error: OpenNamespaceZoneIteratorError) -> Self {
        Self::OpenNamespaceZoneIteratorError(error)
    }
}

impl From<regex::Error> for OpenNamespaceMatchZoneIteratorError {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NextNamespaceMatchZoneIteratorError {
    NextNamespaceZoneIteratorError(NextNamespaceZoneIteratorError),
    OpenZoneConfigurationError(OpenZoneConfigurationError),
}

impl error::Error for NextNamespaceMatchZoneIteratorError {}

impl Debug for NextNamespaceMatchZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NextNamespaceZoneIteratorError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for NextNamespaceMatchZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NextNamespaceZoneIteratorError(error) => Display::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<NextNamespaceZoneIteratorError> for NextNamespaceMatchZoneIteratorError {
    fn from(error: NextNamespaceZoneIteratorError) -> Self {
        Self::NextNamespaceZoneIteratorError(error)
    }
}

impl From<OpenZoneConfigurationError> for NextNamespaceMatchZoneIteratorError {
    fn from(error: OpenZoneConfigurationError) -> Self {
        Self::OpenZoneConfigurationError(error)
    }
}
