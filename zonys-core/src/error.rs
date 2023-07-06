use crate::{
    AcquireZoneLockError, CleanupZoneConfigurationError, CleanupZoneLockError,
    CleanupZoneTypeError, CreateZoneTypeError, DestroyZoneConfigurationError, DestroyZoneTypeError,
    FileSystemIdentifierTryFromZoneIdentifierError, HoldZoneLockError, ReadZoneConfigurationError,
    ReceiveZoneConfigurationError, ReceiveZoneTypeError, ReleaseZoneLockError, RenderTemplateError,
    SendZoneConfigurationError, SendZoneTypeError, StartZoneTypeError, StopZoneTypeError,
    WriteZoneConfigurationError, ZoneIdentifierTryFromPathError,
};
use nix::errno::Errno;
use std::io;
use std::path::StripPrefixError;
use url::ParseError;
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadZoneStatusError {
    __Placeholder,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CleanupZoneConfigurationError(CleanupZoneConfigurationError),
    CleanupZoneLockError(CleanupZoneLockError),
    CleanupZoneTypeError(CleanupZoneTypeError),
    CleanupZoneErrors(Vec<CleanupZoneError>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneError {
    IoError(io::Error),
    #[Display("File system is not existing")]
    FileSystemNotExisting,
    YamlError(serde_yaml::Error),
    StartZoneError(StartZoneError),
    RenderTemplateError(RenderTemplateError),
    ReqwestError(reqwest::Error),
    ParseUrlError(ParseError),
    #[Display("Scheme {value} is unsupported")]
    #[From(skip)]
    UnsupportedScheme(String),
    #[Display("Extension {value} is unsupported")]
    #[From(skip)]
    UnsupportedExtension(String),
    ZoneIdentifierTryFromPathError(ZoneIdentifierTryFromPathError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    HoldZoneLockError(HoldZoneLockError),
    WriteZoneConfigurationError(WriteZoneConfigurationError),
    CleanupZoneError(CleanupZoneError),
    CreateZoneTypeError(CreateZoneTypeError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartZoneError {
    #[Display("Zone is already running")]
    AlreadyRunning,
    HoldZoneLockError(HoldZoneLockError),
    ReadZoneStatusError(ReadZoneStatusError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    StartZoneTypeError(StartZoneTypeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopZoneError {
    #[Display("Zone is not running")]
    NotRunning,
    HoldZoneLockError(HoldZoneLockError),
    ReadZoneStatusError(ReadZoneStatusError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    DestroyZoneError(DestroyZoneError),
    StopZoneTypeError(StopZoneTypeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneError {
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    #[Display("Zone is running")]
    IsRunning,
    AcquireZoneLockError(AcquireZoneLockError),
    ReleaseZoneLockError(ReleaseZoneLockError),
    IoError(io::Error),
    HoldZoneLockError(HoldZoneLockError),
    DestroyZoneConfigurationError(DestroyZoneConfigurationError),
    ReadZoneStatusError(ReadZoneStatusError),
    DestroyZoneTypeError(DestroyZoneTypeError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenZoneError {
    __Placeholder,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ParseZoneIdentifierBaseError {
    #[Display("Input is empty")]
    EmptyInput,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ParseZoneIdentifierError {
    UuidError(uuid::Error),
    #[Display("Input is empty")]
    EmptyInput,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenZoneConfigurationError {
    IoError(io::Error),
    YamlError(serde_yaml::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneError {
    HoldZoneLockError(HoldZoneLockError),
    ZoneIsRunning,
    MissingFileSystem,
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    YamlError(serde_yaml::Error),
    PostcardError(postcard::Error),
    IoError(io::Error),
    Errno(Errno),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    SendZoneConfigurationError(SendZoneConfigurationError),
    SendZoneTypeError(SendZoneTypeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneError {
    HoldZoneLockError(HoldZoneLockError),
    IoError(io::Error),
    PostcardError(postcard::Error),
    YamlError(serde_yaml::Error),
    #[Display("Magic number is missing")]
    MissingMagicNumber,
    Errno(Errno),
    #[Display("Input is empty")]
    EmptyInput,
    ZoneIdentifierTryFromPathError(ZoneIdentifierTryFromPathError),
    ReceiveZoneConfigurationError(ReceiveZoneConfigurationError),
    ReceiveZoneTypeError(ReceiveZoneTypeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ConvertZoneIdentifierFromFileSystemIdentifierError {
    #[Display("Zone identifier is missing")]
    MissingZoneIdentifier,
    UuidError(uuid::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum AllZoneIteratorError {
    IoError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum NextAllZoneIteratorError {
    IoError(io::Error),
    OpenZoneError(OpenZoneError),
    ParseZoneIdentifierError(ParseZoneIdentifierError),
    StripPrefixError(StripPrefixError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum MatchZoneIteratorError {
    IoError(io::Error),
    RegexError(regex::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum NextMatchZoneIteratorError {
    IoError(io::Error),
    NextAllZoneIteratorError(NextAllZoneIteratorError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}
