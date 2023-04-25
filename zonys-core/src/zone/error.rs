use crate::template::RenderTemplateError;
use crate::zone::configuration::ProcessZoneConfigurationError;
use crate::zone::executor::{
    CreateZoneExecutorEventError, DestroyZoneExecutorEventError, RunningZoneExecutorEventError,
    StartZoneExecutorEventError, StopZoneExecutorEventError,
};
use crate::zone::identifier::{
    FileSystemIdentifierTryFromZoneIdentifierError, ZoneIdentifierTryFromPathError,
};
use jail::{CreateJailError, DestroyJailError, ExecuteJailError, TryIntoJailIdError};
use nix::errno::Errno;
use std::io;
use std::path::StripPrefixError;
use std::process::ExitStatusError;
use url::ParseError;
use zfs::file_system::error::{
    CreateFileSystemError, DestroyFileSystemError, MountFileSystemError, OpenFileSystemError,
    ReceiveFileSystemError, SendFileSystemError, UnmountAllFileSystemError,
};
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ExecuteParentZoneError {
    RenderTemplateError(RenderTemplateError),
    IoError(io::Error),
    ExitStatusError(ExitStatusError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ExecuteChildZoneError {
    ExecuteJailError(ExecuteJailError),
    RenderTemplateError(RenderTemplateError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ExecuteZoneError {
    Parent(ExecuteParentZoneError),
    Child(ExecuteChildZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneError {
    IoError(io::Error),
    #[Display("File system is not existing")]
    FileSystemNotExisting,
    CreateFileSystemError(CreateFileSystemError),
    MountFileSystemError(MountFileSystemError),
    OpenFileSystemError(OpenFileSystemError),
    ExecuteZoneError(ExecuteZoneError),
    YamlError(serde_yaml::Error),
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    StartZoneError(StartZoneError),
    DestroyFileSystemError(DestroyFileSystemError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    TryIntoJailIdError(TryIntoJailIdError),
    ProcessZoneConfigurationError(ProcessZoneConfigurationError),
    CreateZoneExecutorEventError(CreateZoneExecutorEventError),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartZoneError {
    #[Display("Zone is already running")]
    AlreadyRunning,
    TryIntoJailIdError(TryIntoJailIdError),
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    CreateJailError(CreateJailError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    StartZoneExecutorEventError(StartZoneExecutorEventError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopZoneError {
    #[Display("Zone is not running")]
    NotRunning,
    TryIntoJailIdError(TryIntoJailIdError),
    DestroyJailError(DestroyJailError),
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    DestroyZoneError(DestroyZoneError),
    StopZoneExecutorEventError(StopZoneExecutorEventError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneError {
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    TryIntoJailIdError(TryIntoJailIdError),
    #[Display("Zone is running")]
    IsRunning,
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    IoError(io::Error),
    DestroyFileSystemError(DestroyFileSystemError),
    OpenFileSystemError(OpenFileSystemError),
    FileSystemNotExisting,
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    DestroyZoneExecutorEventError(DestroyZoneExecutorEventError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
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
    ProcessZoneConfigurationError(ProcessZoneConfigurationError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum RetrieveZoneRunningStatusError {
    RunningZoneExecutorEventError(RunningZoneExecutorEventError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum LockZoneError {
    Errno(Errno),
    IoError(io::Error),
    #[Display("Zone is already locked")]
    AlreadyLocked,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum UnlockZoneError {
    Errno(Errno),
    IoError(io::Error),
    #[Display("Zone is not locked")]
    NotLocked,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneError {
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
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
    RetrieveRunningStatusError(RetrieveZoneRunningStatusError),
    FileSystemIdentifierTryFromZoneIdentifierError(FileSystemIdentifierTryFromZoneIdentifierError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneError {
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
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
    OpenZoneConfigurationError(OpenZoneConfigurationError),
}
