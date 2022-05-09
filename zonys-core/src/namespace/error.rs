use crate::zone::{OpenZoneError, ParseZoneIdentifierError};
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ParseNamespaceIdentifierError {}

impl Debug for ParseNamespaceIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}

impl error::Error for ParseNamespaceIdentifierError {}

impl Display for ParseNamespaceIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenNamespaceError {
    ZfsError(zfs::Error),
}

impl Debug for OpenNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for OpenNamespaceError {}

impl From<zfs::Error> for OpenNamespaceError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenNamespaceZoneIteratorError {
    ZfsError(zfs::Error),
}

impl Debug for OpenNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for OpenNamespaceZoneIteratorError {}

impl From<zfs::Error> for OpenNamespaceZoneIteratorError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NextNamespaceZoneIteratorError {
    ZfsError(zfs::Error),
    ParseZoneIdentifierError(ParseZoneIdentifierError),
    OpenZoneError(OpenZoneError),
}

impl Debug for NextNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::ParseZoneIdentifierError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for NextNamespaceZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::ParseZoneIdentifierError(error) => Display::fmt(error, formatter),
            Self::OpenZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for NextNamespaceZoneIteratorError {}

impl From<zfs::Error> for NextNamespaceZoneIteratorError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<ParseZoneIdentifierError> for NextNamespaceZoneIteratorError {
    fn from(error: ParseZoneIdentifierError) -> Self {
        Self::ParseZoneIdentifierError(error)
    }
}

impl From<OpenZoneError> for NextNamespaceZoneIteratorError {
    fn from(error: OpenZoneError) -> Self {
        Self::OpenZoneError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateNamespaceError {
    ZfsError(zfs::Error),
    FileSystemNotExisting,
}

impl Debug for CreateNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
        }
    }
}

impl Display for CreateNamespaceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
        }
    }
}

impl error::Error for CreateNamespaceError {}

impl From<zfs::Error> for CreateNamespaceError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}
