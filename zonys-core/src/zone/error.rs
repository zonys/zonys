use crate::namespace::ConvertNamespaceIdentifierFromStrError;
use crate::template::RenderTemplateError;
use crate::zone::configuration::error::ProcessZoneConfigurationError;
use bincode::error::{DecodeError, EncodeError};
use jail::{CreateJailError, DestroyJailError, ExecuteJailError, TryIntoJailIdError};
use nix::errno::Errno;
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::process::ExitStatusError;
use zfs::file_system::error::{
    CreateFileSystemError, DestroyFileSystemError, MountFileSystemError, OpenFileSystemError,
    ReadFileSystemMountStatusError, ReceiveFileSystemError, SendFileSystemError,
    UnmountAllFileSystemError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ExecuteParentZoneError {
    RenderTemplateError(RenderTemplateError),
    IoError(io::Error),
    ExitStatusError(ExitStatusError),
}

impl error::Error for ExecuteParentZoneError {}

impl Debug for ExecuteParentZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::RenderTemplateError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::ExitStatusError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ExecuteParentZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::RenderTemplateError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::ExitStatusError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<RenderTemplateError> for ExecuteParentZoneError {
    fn from(error: RenderTemplateError) -> Self {
        Self::RenderTemplateError(error)
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
    RenderTemplateError(RenderTemplateError),
}

impl error::Error for ExecuteChildZoneError {}

impl Debug for ExecuteChildZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExecuteJailError(error) => Debug::fmt(error, formatter),
            Self::RenderTemplateError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ExecuteChildZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExecuteJailError(error) => Display::fmt(error, formatter),
            Self::RenderTemplateError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<ExecuteJailError> for ExecuteChildZoneError {
    fn from(error: ExecuteJailError) -> Self {
        Self::ExecuteJailError(error)
    }
}

impl From<RenderTemplateError> for ExecuteChildZoneError {
    fn from(error: RenderTemplateError) -> Self {
        Self::RenderTemplateError(error)
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
    IoError(io::Error),
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
    ReadFileSystemMountStatusError(ReadFileSystemMountStatusError),
    UnmountAllFileSystemError(UnmountAllFileSystemError),
    TryIntoJailIdError(TryIntoJailIdError),
    ProcessZoneConfigurationError(ProcessZoneConfigurationError),
}

impl error::Error for CreateZoneError {}

impl Debug for CreateZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
            Self::CreateFileSystemError(error) => Debug::fmt(error, formatter),
            Self::MountFileSystemError(error) => Debug::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Debug::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::CreateJailError(error) => Debug::fmt(error, formatter),
            Self::DestroyJailError(error) => Debug::fmt(error, formatter),
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::StartZoneError(error) => Debug::fmt(error, formatter),
            Self::DestroyFileSystemError(error) => Debug::fmt(error, formatter),
            Self::ReadFileSystemMountStatusError(error) => Debug::fmt(error, formatter),
            Self::UnmountAllFileSystemError(error) => Debug::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
            Self::ProcessZoneConfigurationError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for CreateZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system not existing"),
            Self::CreateFileSystemError(error) => Display::fmt(error, formatter),
            Self::MountFileSystemError(error) => Display::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Display::fmt(error, formatter),
            Self::YamlError(error) => Display::fmt(error, formatter),
            Self::CreateJailError(error) => Display::fmt(error, formatter),
            Self::DestroyJailError(error) => Display::fmt(error, formatter),
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::StartZoneError(error) => Display::fmt(error, formatter),
            Self::DestroyFileSystemError(error) => Display::fmt(error, formatter),
            Self::ReadFileSystemMountStatusError(error) => Display::fmt(error, formatter),
            Self::UnmountAllFileSystemError(error) => Display::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
            Self::ProcessZoneConfigurationError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<io::Error> for CreateZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<CreateFileSystemError> for CreateZoneError {
    fn from(error: CreateFileSystemError) -> Self {
        Self::CreateFileSystemError(error)
    }
}

impl From<MountFileSystemError> for CreateZoneError {
    fn from(error: MountFileSystemError) -> Self {
        Self::MountFileSystemError(error)
    }
}

impl From<OpenFileSystemError> for CreateZoneError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
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

impl From<DestroyFileSystemError> for CreateZoneError {
    fn from(error: DestroyFileSystemError) -> Self {
        Self::DestroyFileSystemError(error)
    }
}

impl From<ReadFileSystemMountStatusError> for CreateZoneError {
    fn from(error: ReadFileSystemMountStatusError) -> Self {
        Self::ReadFileSystemMountStatusError(error)
    }
}

impl From<UnmountAllFileSystemError> for CreateZoneError {
    fn from(error: UnmountAllFileSystemError) -> Self {
        Self::UnmountAllFileSystemError(error)
    }
}

impl From<TryIntoJailIdError> for CreateZoneError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

impl From<ProcessZoneConfigurationError> for CreateZoneError {
    fn from(error: ProcessZoneConfigurationError) -> Self {
        Self::ProcessZoneConfigurationError(error)
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
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    ExecuteZoneError(ExecuteZoneError),
    TryIntoJailIdError(TryIntoJailIdError),
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
}

impl error::Error for DestroyZoneError {}

impl Debug for DestroyZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenZoneConfigurationError(error) => Debug::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Debug::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
            Self::IsRunning => write!(formatter, "Zone is running"),
            Self::CreateJailError(error) => Debug::fmt(error, formatter),
            Self::DestroyJailError(error) => Debug::fmt(error, formatter),
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::DestroyFileSystemError(error) => Debug::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system is not existing"),
            Self::ReadFileSystemMountStatusError(error) => Debug::fmt(error, formatter),
            Self::UnmountAllFileSystemError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for DestroyZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenZoneConfigurationError(error) => Display::fmt(error, formatter),
            Self::ExecuteZoneError(error) => Display::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
            Self::IsRunning => write!(formatter, "Zone is running"),
            Self::CreateJailError(error) => Display::fmt(error, formatter),
            Self::DestroyJailError(error) => Display::fmt(error, formatter),
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::DestroyFileSystemError(error) => Display::fmt(error, formatter),
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
            Self::FileSystemNotExisting => write!(formatter, "File system is not existing"),
            Self::ReadFileSystemMountStatusError(error) => Display::fmt(error, formatter),
            Self::UnmountAllFileSystemError(error) => Display::fmt(error, formatter),
        }
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

impl From<DestroyFileSystemError> for DestroyZoneError {
    fn from(error: DestroyFileSystemError) -> Self {
        Self::DestroyFileSystemError(error)
    }
}

impl From<OpenFileSystemError> for DestroyZoneError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

impl From<ReadFileSystemMountStatusError> for DestroyZoneError {
    fn from(error: ReadFileSystemMountStatusError) -> Self {
        Self::ReadFileSystemMountStatusError(error)
    }
}

impl From<UnmountAllFileSystemError> for DestroyZoneError {
    fn from(error: UnmountAllFileSystemError) -> Self {
        Self::UnmountAllFileSystemError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenZoneError {
    OpenFileSystemError(OpenFileSystemError),
}

impl error::Error for OpenZoneError {}

impl Display for OpenZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for OpenZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<OpenFileSystemError> for OpenZoneError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum NextAllZoneIteratorError {
    ParseZoneIdentifierError(ParseZoneIdentifierError),
    OpenZoneError(OpenZoneError),
}

impl error::Error for NextAllZoneIteratorError {}

impl Display for NextAllZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ParseZoneIdentifierError(error) => Display::fmt(error, formatter),
            Self::OpenZoneError(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for NextAllZoneIteratorError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ParseZoneIdentifierError(error) => Debug::fmt(error, formatter),
            Self::OpenZoneError(error) => Debug::fmt(error, formatter),
        }
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
    ConvertNamespaceIdentifierFromStrError(ConvertNamespaceIdentifierFromStrError),
}

impl error::Error for ParseZoneIdentifierError {}

impl Debug for ParseZoneIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::UuidError(error) => Debug::fmt(error, formatter),
            Self::EmptyInput => write!(formatter, "Input is empty"),
            Self::ConvertNamespaceIdentifierFromStrError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ParseZoneIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::UuidError(error) => Display::fmt(error, formatter),
            Self::EmptyInput => write!(formatter, "Input is empty"),
            Self::ConvertNamespaceIdentifierFromStrError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<uuid::Error> for ParseZoneIdentifierError {
    fn from(error: uuid::Error) -> ParseZoneIdentifierError {
        Self::UuidError(error)
    }
}

impl From<ConvertNamespaceIdentifierFromStrError> for ParseZoneIdentifierError {
    fn from(error: ConvertNamespaceIdentifierFromStrError) -> Self {
        Self::ConvertNamespaceIdentifierFromStrError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum OpenZoneConfigurationError {
    IoError(io::Error),
    YamlError(serde_yaml::Error),
    ProcessZoneConfigurationError(ProcessZoneConfigurationError),
}

impl error::Error for OpenZoneConfigurationError {}

impl Debug for OpenZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::ProcessZoneConfigurationError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for OpenZoneConfigurationError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::YamlError(error) => Display::fmt(error, formatter),
            Self::ProcessZoneConfigurationError(error) => Display::fmt(error, formatter),
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

impl From<ProcessZoneConfigurationError> for OpenZoneConfigurationError {
    fn from(error: ProcessZoneConfigurationError) -> Self {
        Self::ProcessZoneConfigurationError(error)
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
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    SendFileSystemError(SendFileSystemError),
    TryIntoJailIdError(TryIntoJailIdError),
    ZoneIsRunning,
    OpenFileSystemError(OpenFileSystemError),
    MissingFileSystem,
    OpenZoneConfigurationError(OpenZoneConfigurationError),
    YamlError(serde_yaml::Error),
    BincodeEncodeError(EncodeError),
    IoError(io::Error),
}

impl error::Error for SendZoneError {}

impl Debug for SendZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::SendFileSystemError(error) => Display::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Debug::fmt(error, formatter),
            Self::ZoneIsRunning => write!(formatter, "Zone is running"),
            Self::OpenFileSystemError(error) => Debug::fmt(error, formatter),
            Self::MissingFileSystem => write!(formatter, "File system is not existing"),
            Self::OpenZoneConfigurationError(error) => Debug::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::BincodeEncodeError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for SendZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::SendFileSystemError(error) => Display::fmt(error, formatter),
            Self::TryIntoJailIdError(error) => Display::fmt(error, formatter),
            Self::ZoneIsRunning => write!(formatter, "Zone is running"),
            Self::OpenFileSystemError(error) => Display::fmt(error, formatter),
            Self::MissingFileSystem => write!(formatter, "File system is not existing"),
            Self::OpenZoneConfigurationError(error) => Display::fmt(error, formatter),
            Self::YamlError(error) => Display::fmt(error, formatter),
            Self::BincodeEncodeError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
        }
    }
}

impl From<LockZoneError> for SendZoneError {
    fn from(error: LockZoneError) -> Self {
        Self::LockZoneError(error)
    }
}

impl From<UnlockZoneError> for SendZoneError {
    fn from(error: UnlockZoneError) -> Self {
        Self::UnlockZoneError(error)
    }
}

impl From<SendFileSystemError> for SendZoneError {
    fn from(error: SendFileSystemError) -> Self {
        Self::SendFileSystemError(error)
    }
}

impl From<TryIntoJailIdError> for SendZoneError {
    fn from(error: TryIntoJailIdError) -> Self {
        Self::TryIntoJailIdError(error)
    }
}

impl From<OpenFileSystemError> for SendZoneError {
    fn from(error: OpenFileSystemError) -> Self {
        Self::OpenFileSystemError(error)
    }
}

impl From<OpenZoneConfigurationError> for SendZoneError {
    fn from(error: OpenZoneConfigurationError) -> Self {
        Self::OpenZoneConfigurationError(error)
    }
}

impl From<serde_yaml::Error> for SendZoneError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlError(error)
    }
}

impl From<EncodeError> for SendZoneError {
    fn from(error: EncodeError) -> Self {
        Self::BincodeEncodeError(error)
    }
}

impl From<io::Error> for SendZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ReceiveZoneError {
    LockZoneError(LockZoneError),
    UnlockZoneError(UnlockZoneError),
    ReceiveFileSystemError(ReceiveFileSystemError),
    IoError(io::Error),
    BincodeDecodeError(DecodeError),
    YamlError(serde_yaml::Error),
    MissingMagicNumber,
}

impl error::Error for ReceiveZoneError {}

impl Debug for ReceiveZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::LockZoneError(error) => Debug::fmt(error, formatter),
            Self::UnlockZoneError(error) => Debug::fmt(error, formatter),
            Self::ReceiveFileSystemError(error) => Debug::fmt(error, formatter),
            Self::IoError(error) => Debug::fmt(error, formatter),
            Self::BincodeDecodeError(error) => Debug::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::MissingMagicNumber => write!(formatter, "Magic number not existing"),
        }
    }
}

impl Display for ReceiveZoneError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::LockZoneError(error) => Display::fmt(error, formatter),
            Self::UnlockZoneError(error) => Display::fmt(error, formatter),
            Self::ReceiveFileSystemError(error) => Display::fmt(error, formatter),
            Self::IoError(error) => Display::fmt(error, formatter),
            Self::BincodeDecodeError(error) => Display::fmt(error, formatter),
            Self::YamlError(error) => Debug::fmt(error, formatter),
            Self::MissingMagicNumber => write!(formatter, "Magic number not existing"),
        }
    }
}

impl From<LockZoneError> for ReceiveZoneError {
    fn from(error: LockZoneError) -> Self {
        Self::LockZoneError(error)
    }
}

impl From<UnlockZoneError> for ReceiveZoneError {
    fn from(error: UnlockZoneError) -> Self {
        Self::UnlockZoneError(error)
    }
}

impl From<ReceiveFileSystemError> for ReceiveZoneError {
    fn from(error: ReceiveFileSystemError) -> Self {
        Self::ReceiveFileSystemError(error)
    }
}

impl From<io::Error> for ReceiveZoneError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<DecodeError> for ReceiveZoneError {
    fn from(error: DecodeError) -> Self {
        Self::BincodeDecodeError(error)
    }
}

impl From<serde_yaml::Error> for ReceiveZoneError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ConvertZoneIdentifierFromFileSystemIdentifierError {
    MissingZoneIdentifier,
    UuidError(uuid::Error),
}

impl error::Error for ConvertZoneIdentifierFromFileSystemIdentifierError {}

impl Debug for ConvertZoneIdentifierFromFileSystemIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingZoneIdentifier => write!(formatter, "Zone identifier is missing"),
            Self::UuidError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl Display for ConvertZoneIdentifierFromFileSystemIdentifierError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingZoneIdentifier => write!(formatter, "Zone identifier is missing"),
            Self::UuidError(error) => Debug::fmt(error, formatter),
        }
    }
}

impl From<uuid::Error> for ConvertZoneIdentifierFromFileSystemIdentifierError {
    fn from(error: uuid::Error) -> Self {
        Self::UuidError(error)
    }
}
