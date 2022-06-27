use serde_yaml::Value;
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ProcessZoneConfigurationError {
    YamlError(serde_yaml::Error),
    IoError(io::Error),
    MergeZoneConfigurationError(MergeZoneConfigurationError),
}

impl Debug for ProcessZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::MergeZoneConfigurationError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ProcessZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::YamlError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::MergeZoneConfigurationError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for ProcessZoneConfigurationError {}

impl From<serde_yaml::Error> for ProcessZoneConfigurationError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlError(error)
    }
}

impl From<io::Error> for ProcessZoneConfigurationError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<MergeZoneConfigurationError> for ProcessZoneConfigurationError {
    fn from(error: MergeZoneConfigurationError) -> Self {
        Self::MergeZoneConfigurationError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum MergeZoneConfigurationError {
    IncompatibleValues(Value, Value),
    YamlError(serde_yaml::Error),
}

impl Debug for MergeZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IncompatibleValues(v1, v2) => {
                write!(formatter, "Values are incompatible ({:?}, {:?})", v1, v2)
            }
            Self::YamlError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for MergeZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IncompatibleValues(v1, v2) => {
                write!(formatter, "Values are incompatible ({:?}, {:?})", v1, v2)
            }
            Self::YamlError(error) => Display::fmt(error, formatter),
        }
    }
}

impl error::Error for MergeZoneConfigurationError {}

impl From<serde_yaml::Error> for MergeZoneConfigurationError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlError(error)
    }
}
