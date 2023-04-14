use serde_yaml::Value;
use std::io;
use std::path::PathBuf;
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ProcessZoneConfigurationError {
    YamlError(serde_yaml::Error),
    IoError(io::Error),
    MergeZoneConfigurationError(MergeZoneConfigurationError),
    #[Display("Parent path is missing")]
    MissingParent(PathBuf),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum MergeZoneConfigurationError {
    #[Display("Values are incompatible {value0:?} {value1:?}")]
    IncompatibleValues(Value, Value),
    YamlError(serde_yaml::Error),
}
