use crate::{
    AcquireZoneLockError, CleanupZoneLockError, CreateZoneVolumeError,
    DestroyZoneConfigurationError, ReleaseZoneLockError, TriggerZoneExecutorCreateError,
    TriggerZoneExecutorDestroyError, TriggerZoneExecutorStartError, TriggerZoneExecutorStopError,
};
use crate::{
    CleanupZoneConfigurationError, CleanupZoneVolumeError, DestroyZoneVolumeError,
    HoldZoneLockError, ReadZoneConfigurationError, TransformZoneConfigurationError,
    WriteZoneConfigurationError,
};
use crate::{
    FileSystemIdentifierTryFromZoneIdentifierError, RenderTemplateError,
    ZoneIdentifierTryFromPathError,
};
use jail::{CreateJailError, DestroyJailError, TryIntoJailIdError};
use nix::errno::Errno;
use std::io;
use std::path::StripPrefixError;
use url::ParseError;
use zfs::file_system::error::{
    DestroyFileSystemError, OpenFileSystemError, ReceiveFileSystemError, SendFileSystemError,
    UnmountAllFileSystemError,
};
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
    CleanupZoneVolumeError(CleanupZoneVolumeError),
    CleanupZoneConfigurationError(CleanupZoneConfigurationError),
    CleanupZoneLockError(CleanupZoneLockError),
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
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    StartZoneError(StartZoneError),
    DestroyFileSystemError(DestroyFileSystemError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    TryIntoJailIdError(TryIntoJailIdError),
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
    CreateZoneVolumeError(CreateZoneVolumeError),
    TransformZoneConfigurationError(TransformZoneConfigurationError),
    HoldZoneLockError(HoldZoneLockError),
    WriteZoneConfigurationError(WriteZoneConfigurationError),
    CleanupZoneError(CleanupZoneError),
    TriggerZoneExecutorCreateError(TriggerZoneExecutorCreateError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartZoneError {
    #[Display("Zone is already running")]
    AlreadyRunning,
    HoldZoneLockError(HoldZoneLockError),
    TriggerZoneExecutorStartError(TriggerZoneExecutorStartError),
    ReadZoneStatusError(ReadZoneStatusError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopZoneError {
    #[Display("Zone is not running")]
    NotRunning,
    HoldZoneLockError(HoldZoneLockError),
    TriggerZoneExecutorStopError(TriggerZoneExecutorStopError),
    ReadZoneStatusError(ReadZoneStatusError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    DestroyZoneError(DestroyZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneError {
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    TryIntoJailIdError(TryIntoJailIdError),
    #[Display("Zone is running")]
    IsRunning,
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    AcquireZoneLockError(AcquireZoneLockError),
    ReleaseZoneLockError(ReleaseZoneLockError),
    IoError(io::Error),
    DestroyFileSystemError(DestroyFileSystemError),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemNotExisting,
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    HoldZoneLockError(HoldZoneLockError),
    DestroyZoneConfigurationError(DestroyZoneConfigurationError),
    DestroyZoneVolumeError(DestroyZoneVolumeError),
    TriggerZoneExecutorDestroyError(TriggerZoneExecutorDestroyError),
    ReadZoneStatusError(ReadZoneStatusError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum OpenZoneError {
    OpenFileSystemError(OpenFileSystemError),
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
    SendFileSystemError(SendFileSystemError),
    TryIntoJailIdError(TryIntoJailIdError),
    ZoneIsRunning,
    OpenFileSystemError(OpenFileSystemError),
    MissingFileSystem,
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    YamlError(serde_yaml::Error),
    PostcardError(postcard::Error),
    IoError(io::Error),
    Errno(Errno),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
    ReadZoneConfigurationError(ReadZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneError {
    HoldZoneLockError(HoldZoneLockError),
    ReceiveFileSystemError(ReceiveFileSystemError),
    IoError(io::Error),
    PostcardError(postcard::Error),
    YamlError(serde_yaml::Error),
    #[Display("Magic number is missing")]
    MissingMagicNumber,
    Errno(Errno),
    #[Display("Input is empty")]
    EmptyInput,
    ZoneIdentifierTryFromPathError(ZoneIdentifierTryFromPathError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
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
