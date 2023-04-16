use crate::namespace::ConvertNamespaceIdentifierFromStrError;
use crate::template::RenderTemplateError;
use crate::zone::configuration::ProcessZoneConfigurationError;
use crate::zone::executor::{
    CreateZoneExecutorEventError, DestroyZoneExecutorEventError, RunningZoneExecutorEventError,
    StartZoneExecutorEventError, StopZoneExecutorEventError,
};
use jail::{CreateJailError, DestroyJailError, ExecuteJailError, TryIntoJailIdError};
use nix::errno::Errno;
use std::io;
use std::process::ExitStatusError;
use url::ParseError;
use zfs::file_system::error::{
    CreateFileSystemError, DestroyFileSystemError, MountFileSystemError, OpenFileSystemError,
    ReadFileSystemMountStatusError, ReceiveFileSystemError, SendFileSystemError,
    UnmountAllFileSystemError,
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
pub enum CreateZoneError {
    #[From]
    IoError(io::Error),
    #[Display("File system is not existing")]
    FileSystemNotExisting,
    #[From]
    CreateFileSystemError(CreateFileSystemError),
    #[From]
    MountFileSystemError(MountFileSystemError),
    #[From]
    OpenFileSystemError(OpenFileSystemError),
    #[From]
    ExecuteZoneError(ExecuteZoneError),
    #[From]
    YamlError(serde_yaml::Error),
    #[From]
    CreateJailError(CreateJailError),
    #[From]
    DestroyJailError(DestroyJailError),
    #[From]
    LockZoneError(LockZoneError),
    #[From]
    UnlockZoneError(UnlockZoneError),
    #[From]
    StartZoneError(StartZoneError),
    #[From]
    DestroyFileSystemError(DestroyFileSystemError),
    #[From]
    ReadFileSystemMountStatusError(ReadFileSystemMountStatusError),
    #[From]
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    #[From]
    TryIntoJailIdError(TryIntoJailIdError),
    #[From]
    ProcessZoneConfigurationError(ProcessZoneConfigurationError),
    #[From]
    CreateZoneExecutorEventError(CreateZoneExecutorEventError),
    #[From]
    RenderTemplateError(RenderTemplateError),
    #[From]
    ReqwestError(reqwest::Error),
    #[From]
    ParseUrlError(ParseError),
    #[Display("Scheme {value} is unsupported")]
    UnsupportedScheme(String),
    #[Display("Extension {value} is unsupported")]
    UnsupportedExtension(String),
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
    ReadFileSystemMountStatusError(ReadFileSystemMountStatusError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    DestroyZoneExecutorEventError(DestroyZoneExecutorEventError),
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
pub enum NextAllZoneIteratorError {
    ParseZoneIdentifierError(ParseZoneIdentifierError),
    OpenZoneError(OpenZoneError),
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
    ConvertNamespaceIdentifierFromStrError(ConvertNamespaceIdentifierFromStrError),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ConvertZoneIdentifierFromFileSystemIdentifierError {
    #[Display("Zone identifier is missing")]
    MissingZoneIdentifier,
    UuidError(uuid::Error),
}
