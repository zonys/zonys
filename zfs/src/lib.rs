#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod file_system;

////////////////////////////////////////////////////////////////////////////////////////////////////

use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use zfs_sys::wrapper::{libzfs_init, WrapperError};

use crate::file_system::FileSystems;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const SEPARATOR: &str = "/";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Error {
    WrapperError(WrapperError),
    ZfsError(i32, String),
}

impl error::Error for Error {}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::WrapperError(error) => Debug::fmt(error, formatter),
            Self::ZfsError(code, description) => {
                write!(formatter, "{} ({})", description, code)
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::WrapperError(error) => Display::fmt(error, formatter),
            Self::ZfsError(code, description) => {
                write!(formatter, "{} ({})", description, code)
            }
        }
    }
}

impl From<WrapperError> for Error {
    fn from(error: WrapperError) -> Self {
        Self::WrapperError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Zfs {
    file_systems: FileSystems,
}

impl Zfs {
    pub fn new() -> Result<Self, Error> {
        let handle = libzfs_init()?;

        Ok(Self {
            file_systems: FileSystems::new(handle),
        })
    }

    pub fn file_systems(&self) -> &FileSystems {
        &self.file_systems
    }

    pub fn file_systems_mut(&mut self) -> &mut FileSystems {
        &mut self.file_systems
    }
}
