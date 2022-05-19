use crate::namespace::ParseNamespaceIdentifierError;
use crate::template;
use jail::{CreateJailError, DestroyJailError, ExecuteJailError, TryIntoJailIdError};
use nix::errno::Errno;
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::process::ExitStatusError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ExecuteParentZoneError {
    TemplateError(template::Error),
    IoError(io::Error),
    ExitStatusError(ExitStatusError),
}

impl error::Error for ExecuteParentZoneError {}

impl Debug for ExecuteParentZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::TemplateError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::ExitStatusError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ExecuteParentZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::TemplateError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::ExitStatusError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<template::Error> for ExecuteParentZoneError {
    fn from(error: template::Error) -> Self {
        Self::TemplateError(error)
    }
}

impl From<io::Error> for ExecuteParentZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<ExitStatusError> for ExecuteParentZoneError {
    fn from(error: ExitStatusError) -> Self {
        Self::ExitStatusError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ExecuteChildZoneError {
    ExecuteJailError(ExecuteJailError),
    TemplateError(template::Error),
}

impl error::Error for ExecuteChildZoneError {}

impl Debug for ExecuteChildZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExecuteJailError(error) => Debug::fmt(error, formatter),
            Self::TemplateError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ExecuteChildZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExecuteJailError(error) => Display::fmt(error, formatter),
            Self::TemplateError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<ExecuteJailError> for ExecuteChildZoneError {
    fn from(error: ExecuteJailError) -> Self {
        Self::ExecuteJailError(error)
    }
}

impl From<template::Error> for ExecuteChildZoneError {
    fn from(error: template::Error) -> Self {
        Self::TemplateError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ExecuteZoneError {
    Parent(ExecuteParentZoneError),
    Child(ExecuteChildZoneError),
}

impl error::Error for ExecuteZoneError {}

impl Debug for ExecuteZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Parent(error) => Debug::fmt(error, formatter),
            Self::Child(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ExecuteZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Parent(error) => Display::fmt(error, formatter),
            Self::Child(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<ExecuteParentZoneError> for ExecuteZoneError {
    fn from(error: ExecuteParentZoneError) -> Self {
        Self::Parent(error)
    }
}

impl From<ExecuteChildZoneError> for ExecuteZoneError {
    fn from(error: ExecuteChildZoneError) -> Self {
        Self::Child(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateZoneError {
    ZfsError(zfs::Error),
    IoError(io::Error),
    FileSystemNotExisting,
    ExecuteZoneError(ExecuteZoneError),
    YamlError(serde_yaml::Error),
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    StartZoneError(StartZoneError),
}

impl error::Error for CreateZoneError {}

impl Debug for CreateZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
            Self::ExecuteZoneError(error) => Debug::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::CreateJailError(error) => Debug::fmt(error, formatter),
            Self::DestroyJailError(error) => Debug::fmt(error, formatter),
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::StartZoneError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
            Self::ExecuteZoneError(error) => Display::fmt(error, formatter),
            Self::YamlError(error) => Display::fmt(error, formatter),
            Self::CreateJailError(error) => Display::fmt(error, formatter),
            Self::DestroyJailError(error) => Display::fmt(error, formatter),
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::StartZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for CreateZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<io::Error> for CreateZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<ExecuteZoneError> for CreateZoneError {
    fn from(error: ExecuteZoneError) -> Self {
        Self::ExecuteZoneError(error)
    }
}

impl From<serde_yaml::Error> for CreateZoneError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlError(error)
    }
}

impl From<CreateJailError> for CreateZoneError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJailError(error)
    }
}

impl From<DestroyJailError> for CreateZoneError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJailError(error)
    }
}

impl From<LockZoneError> for CreateZoneError {
    fn from(error: LockZoneError) -> Self {
        Self::LockZoneError(error)
    }
}

impl From<UnlockZoneError> for CreateZoneError {
    fn from(error: UnlockZoneError) -> Self {
        Self::UnlockZoneError(error)
    }
}

impl From<StartZoneError> for CreateZoneError {
    fn from(error: StartZoneError) -> Self {
        Self::StartZoneError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StartZoneError {
    AlreadyRunning,
    TryIntoJailIdError(TryIntoJailIdError),
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    CreateJailError(CreateJailError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
}

impl error::Error for StartZoneError {}

impl Debug for StartZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::AlreadyRunning => write!(formatter, "Zone is already running"),
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Debug::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Debug::fmt(error, formatter),
            Self::CreateJailError(error) => Debug::fmt(error, formatter),
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for StartZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::AlreadyRunning => write!(formatter, "Zone is already running"),
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Display::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Display::fmt(error, formatter),
            Self::CreateJailError(error) => Display::fmt(error, formatter),
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<TryIntoJailIdError> for StartZoneError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

impl From<OpenZoneConfigurationError> for StartZoneError {
    fn from(error: OpenZoneConfigurationError) -> Self {
        Self::OpenZoneConfigurationError(error)
    }
}

impl From<ExecuteZoneError> for StartZoneError {
    fn from(error: ExecuteZoneError) -> Self {
        Self::ExecuteZoneError(error)
    }
}

impl From<CreateJailError> for StartZoneError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJailError(error)
    }
}

impl From<LockZoneError> for StartZoneError {
    fn from(error: LockZoneError) -> Self {
        Self::LockZoneError(error)
    }
}

impl From<UnlockZoneError> for StartZoneError {
    fn from(error: UnlockZoneError) -> Self {
        Self::UnlockZoneError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StopZoneError {
    NotRunning,
    TryIntoJailIdError(TryIntoJailIdError),
    DestroyJailError(DestroyJailError),
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    DestroyZoneError(DestroyZoneError),
}

impl error::Error for StopZoneError {}

impl Debug for StopZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NotRunning => write!(formatter, "Zone is not running"),
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
            Self::DestroyJailError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Debug::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Debug::fmt(error, formatter),
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::DestroyZoneError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for StopZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::NotRunning => write!(formatter, "Zone is not running"),
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
            Self::DestroyJailError(error) => Display::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Display::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Display::fmt(error, formatter),
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::DestroyZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<TryIntoJailIdError> for StopZoneError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

impl From<DestroyJailError> for StopZoneError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJailError(error)
    }
}

impl From<OpenZoneConfigurationError> for StopZoneError {
    fn from(error: OpenZoneConfigurationError) -> Self {
        Self::OpenZoneConfigurationError(error)
    }
}

impl From<ExecuteZoneError> for StopZoneError {
    fn from(error: ExecuteZoneError) -> Self {
        Self::ExecuteZoneError(error)
    }
}

impl From<LockZoneError> for StopZoneError {
    fn from(error: LockZoneError) -> Self {
        Self::LockZoneError(error)
    }
}

impl From<UnlockZoneError> for StopZoneError {
    fn from(error: UnlockZoneError) -> Self {
        Self::UnlockZoneError(error)
    }
}

impl From<DestroyZoneError> for StopZoneError {
    fn from(error: DestroyZoneError) -> Self {
        Self::DestroyZoneError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum DestroyZoneError {
    ZfsError(zfs::Error),
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    TryIntoJailIdError(TryIntoJailIdError),
    IsRunning,
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    IoError(io::Error),
}

impl error::Error for DestroyZoneError {}

impl Debug for DestroyZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Debug::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Debug::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
            Self::IsRunning => write!(formatter, "Zone is running"),
            Self::CreateJailError(error) => Debug::fmt(error, formatter),
            Self::DestroyJailError(error) => Debug::fmt(error, formatter),
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for DestroyZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::OpenZoneConfigurationError(error) => Display::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Display::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
            Self::IsRunning => write!(formatter, "Zone is running"),
            Self::CreateJailError(error) => Display::fmt(error, formatter),
            Self::DestroyJailError(error) => Display::fmt(error, formatter),
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for DestroyZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<OpenZoneConfigurationError> for DestroyZoneError {
    fn from(error: OpenZoneConfigurationError) -> Self {
        Self::OpenZoneConfigurationError(error)
    }
}

impl From<ExecuteZoneError> for DestroyZoneError {
    fn from(error: ExecuteZoneError) -> Self {
        Self::ExecuteZoneError(error)
    }
}

impl From<TryIntoJailIdError> for DestroyZoneError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

impl From<CreateJailError> for DestroyZoneError {
    fn from(error: CreateJailError) -> Self {
        Self::CreateJailError(error)
    }
}

impl From<DestroyJailError> for DestroyZoneError {
    fn from(error: DestroyJailError) -> Self {
        Self::DestroyJailError(error)
    }
}

impl From<LockZoneError> for DestroyZoneError {
    fn from(error: LockZoneError) -> Self {
        Self::LockZoneError(error)
    }
}

impl From<UnlockZoneError> for DestroyZoneError {
    fn from(error: UnlockZoneError) -> Self {
        Self::UnlockZoneError(error)
    }
}

impl From<io::Error> for DestroyZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ExistsZoneError {
    ZfsError(zfs::Error),
}

impl error::Error for ExistsZoneError {}

impl Display for ExistsZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for ExistsZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for ExistsZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenZoneError {
    ZfsError(zfs::Error),
}

impl error::Error for OpenZoneError {}

impl Display for OpenZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for OpenZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for OpenZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum AllZoneError {
    ZfsError(zfs::Error),
}

impl error::Error for AllZoneError {}

impl Display for AllZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for AllZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for AllZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NextAllZoneIteratorError {
    ZfsError(zfs::Error),
    ParseZoneIdentifierError(ParseZoneIdentifierError),
    OpenZoneError(OpenZoneError),
}

impl error::Error for NextAllZoneIteratorError {}

impl Display for NextAllZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
            Self::ParseZoneIdentifierError(error) => Display::fmt(error, formatter),
            Self::OpenZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for NextAllZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
            Self::ParseZoneIdentifierError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for NextAllZoneIteratorError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

impl From<ParseZoneIdentifierError> for NextAllZoneIteratorError {
    fn from(error: ParseZoneIdentifierError) -> Self {
        Self::ParseZoneIdentifierError(error)
    }
}

impl From<OpenZoneError> for NextAllZoneIteratorError {
    fn from(error: OpenZoneError) -> Self {
        Self::OpenZoneError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ParseZoneIdentifierBaseError {
    EmptyInput,
}

impl error::Error for ParseZoneIdentifierBaseError {}

impl Debug for ParseZoneIdentifierBaseError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "Input is empty"),
        }
    }
}

impl Display for ParseZoneIdentifierBaseError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "Input is empty"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ParseZoneIdentifierError {
    UuidError(uuid::Error),
    EmptyInput,
    ParseNamespaceIdentifierError(ParseNamespaceIdentifierError),
}

impl error::Error for ParseZoneIdentifierError {}

impl Debug for ParseZoneIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::UuidError(error) => Debug::fmt(error, formatter),
            Self::EmptyInput => write!(formatter, "Input is empty"),
            Self::ParseNamespaceIdentifierError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ParseZoneIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::UuidError(error) => Display::fmt(error, formatter),
            Self::EmptyInput => write!(formatter, "Input is empty"),
            Self::ParseNamespaceIdentifierError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<uuid::Error> for ParseZoneIdentifierError {
    fn from(error: uuid::Error) -> ParseZoneIdentifierError {
        Self::UuidError(error)
    }
}

impl From<ParseNamespaceIdentifierError> for ParseZoneIdentifierError {
    fn from(error: ParseNamespaceIdentifierError) -> Self {
        Self::ParseNamespaceIdentifierError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenZoneConfigurationError {
    IoError(io::Error),
    YamlError(serde_yaml::Error),
}

impl error::Error for OpenZoneConfigurationError {}

impl Debug for OpenZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::YamlError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<io::Error> for OpenZoneConfigurationError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_yaml::Error> for OpenZoneConfigurationError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum RetrieveZoneRunningStatusError {
    TryIntoJailIdError(TryIntoJailIdError),
}

impl error::Error for RetrieveZoneRunningStatusError {}

impl Debug for RetrieveZoneRunningStatusError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for RetrieveZoneRunningStatusError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<TryIntoJailIdError> for RetrieveZoneRunningStatusError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum LockZoneError {
    Errno(Errno),
    IoError(io::Error),
    AlreadyLocked,
}

impl error::Error for LockZoneError {}

impl Debug for LockZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::AlreadyLocked => write!(formatter, "Zone is already locked"),
        }
    }
}

impl Display for LockZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::AlreadyLocked => write!(formatter, "Zone is already locked"),
        }
    }
}

impl From<Errno> for LockZoneError {
    fn from(errno: Errno) -> Self {
        Self::Errno(errno)
    }
}

impl From<io::Error> for LockZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum UnlockZoneError {
    Errno(Errno),
    IoError(io::Error),
    NotLocked,
}

impl error::Error for UnlockZoneError {}

impl Debug for UnlockZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Debug::fmt(errno, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::NotLocked => write!(formatter, "Zone is not locked"),
        }
    }
}

impl Display for UnlockZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errno(errno) => Display::fmt(errno, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::NotLocked => write!(formatter, "Zone is not locked"),
        }
    }
}

impl From<Errno> for UnlockZoneError {
    fn from(errno: Errno) -> Self {
        Self::Errno(errno)
    }
}

impl From<io::Error> for UnlockZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum SendZoneError {
    ZfsError(zfs::Error),
}

impl error::Error for SendZoneError {}

impl Debug for SendZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for SendZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for SendZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReceiveZoneError {
    ZfsError(zfs::Error),
}

impl error::Error for ReceiveZoneError {}

impl Debug for ReceiveZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ReceiveZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ZfsError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<zfs::Error> for ReceiveZoneError {
    fn from(error: zfs::Error) -> Self {
        Self::ZfsError(error)
    }
}
