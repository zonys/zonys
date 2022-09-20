use crate::zone::executor::jail::{
    CreateJailZoneExecutorEventError, DestroyJailZoneExecutorEventError,
    RunningJailZoneExecutorEventError, StartJailZoneExecutorEventError,
    StopJailZoneExecutorEventError,
};
use std::error;
use std::fmt::{self, Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum RunningZoneExecutorEventError {
    Jail(RunningJailZoneExecutorEventError),
}

impl Debug for RunningZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for RunningZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for RunningZoneExecutorEventError {}

impl From<RunningJailZoneExecutorEventError> for RunningZoneExecutorEventError {
    fn from(error: RunningJailZoneExecutorEventError) -> Self {
        Self::Jail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CreateZoneExecutorEventError {
    Jail(CreateJailZoneExecutorEventError),
}

impl Debug for CreateZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for CreateZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for CreateZoneExecutorEventError {}

impl From<CreateJailZoneExecutorEventError> for CreateZoneExecutorEventError {
    fn from(error: CreateJailZoneExecutorEventError) -> Self {
        Self::Jail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StartZoneExecutorEventError {
    Jail(StartJailZoneExecutorEventError),
}

impl Debug for StartZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for StartZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for StartZoneExecutorEventError {}

impl From<StartJailZoneExecutorEventError> for StartZoneExecutorEventError {
    fn from(error: StartJailZoneExecutorEventError) -> Self {
        Self::Jail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StopZoneExecutorEventError {
    Jail(StopJailZoneExecutorEventError),
}

impl Debug for StopZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for StopZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for StopZoneExecutorEventError {}

impl From<StopJailZoneExecutorEventError> for StopZoneExecutorEventError {
    fn from(error: StopJailZoneExecutorEventError) -> Self {
        Self::Jail(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum DestroyZoneExecutorEventError {
    Jail(DestroyJailZoneExecutorEventError),
}

impl Debug for DestroyZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Debug::fmt(e, formatter),
        }
    }
}

impl Display for DestroyZoneExecutorEventError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Jail(e) => Display::fmt(e, formatter),
        }
    }
}

impl error::Error for DestroyZoneExecutorEventError {}

impl From<DestroyJailZoneExecutorEventError> for DestroyZoneExecutorEventError {
    fn from(error: DestroyJailZoneExecutorEventError) -> Self {
        Self::Jail(error)
    }
}
