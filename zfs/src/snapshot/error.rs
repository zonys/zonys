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

pub enum ReadSnapshotNameError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for ReadSnapshotNameError {}

impl Debug for ReadSnapshotNameError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReadSnapshotNameError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for ReadSnapshotNameError {
    fn from(error: zfs_sys::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum SendSnapshotError {
    ZfsError(zfs_sys::Error),
}

impl error::Error for SendSnapshotError {}

impl Debug for SendSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for SendSnapshotError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs_sys::Error> for SendSnapshotError {
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
