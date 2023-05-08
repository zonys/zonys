mod chroot;
mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::r#type::jail::*;
pub use chroot::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    DeserializeZoneTransmissionError, ReadZoneConfigurationError, SerializeZoneTransmissionError,
    Zone, ZoneConfigurationTypeReader, ZoneTransmissionReader, ZoneTransmissionWriter,
};
use serde::{Deserialize, Serialize};
use ztd::{Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateZoneTypeError {
    CreateChrootZoneError(CreateChrootZoneError),
    CreateJailZoneError(CreateJailZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartZoneTypeError {
    StartChrootZoneError(StartChrootZoneError),
    StartJailZoneError(StartJailZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopZoneTypeError {
    StopChrootZoneError(StopChrootZoneError),
    StopJailZoneError(StopJailZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneTypeError {
    DestroyChrootZoneError(DestroyChrootZoneError),
    DestroyJailZoneError(DestroyJailZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneTypeError {
    SendChrootZoneError(SendChrootZoneError),
    SendJailZoneError(SendJailZoneError),
    SerializeZoneTransmissionError(SerializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneTypeError {
    ReceiveChrootZoneError(ReceiveChrootZoneError),
    ReceiveJailZoneError(ReceiveJailZoneError),
    DeserializeZoneTransmissionError(DeserializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneTypeError {
    CleanupChrootZoneError(CleanupChrootZoneError),
    CleanupJailZoneError(CleanupJailZoneError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub enum ZoneTypeTransmissionHeaderVersion1Type {
    Chroot,
    Jail,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub enum ZoneTypeTransmissionHeader {
    Version1 {
        r#type: ZoneTypeTransmissionHeaderVersion1Type,
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ZoneType<T> {
    Chroot(ChrootZone<T>),
    Jail(JailZone<T>),
}

impl<'a> ZoneType<&'a Zone> {
    pub(super) fn new(zone: &'a Zone) -> Result<Self, ReadZoneConfigurationError> {
        let reader = zone.configuration().reader()?;

        match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(_jail) => Ok(Self::Jail(JailZone::new(zone))),
            ZoneConfigurationTypeReader::Chroot(_chroot) => Ok(Self::Chroot(ChrootZone::new(zone))),
        }
    }

    pub(super) fn create(&self) -> Result<(), CreateZoneTypeError> {
        match &self {
            Self::Chroot(chroot) => Ok(chroot.create()?),
            Self::Jail(jail) => Ok(jail.create()?),
        }
    }

    pub(super) fn start(&self) -> Result<(), StartZoneTypeError> {
        match &self {
            Self::Chroot(chroot) => Ok(chroot.start()?),
            Self::Jail(jail) => Ok(jail.start()?),
        }
    }

    pub(super) fn stop(&self) -> Result<(), StopZoneTypeError> {
        match &self {
            Self::Chroot(chroot) => Ok(chroot.stop()?),
            Self::Jail(jail) => Ok(jail.stop()?),
        }
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyZoneTypeError> {
        match &self {
            Self::Chroot(chroot) => Ok(chroot.destroy()?),
            Self::Jail(jail) => Ok(jail.destroy()?),
        }
    }

    pub(super) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendZoneTypeError> {
        match &self {
            Self::Chroot(chroot) => {
                writer.serialize(&ZoneTypeTransmissionHeader::Version1 {
                    r#type: ZoneTypeTransmissionHeaderVersion1Type::Chroot,
                })?;

                Ok(chroot.send(writer)?)
            }
            Self::Jail(jail) => {
                writer.serialize(&ZoneTypeTransmissionHeader::Version1 {
                    r#type: ZoneTypeTransmissionHeaderVersion1Type::Jail,
                })?;

                Ok(jail.send(writer)?)
            }
        }
    }

    pub(super) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveZoneTypeError> {
        match reader.deserialize::<ZoneTypeTransmissionHeader>()? {
            ZoneTypeTransmissionHeader::Version1 { r#type } => match r#type {
                ZoneTypeTransmissionHeaderVersion1Type::Chroot => {
                    Ok(Self::Chroot(ChrootZone::receive(zone, reader)?))
                }
                ZoneTypeTransmissionHeaderVersion1Type::Jail => {
                    Ok(Self::Jail(JailZone::receive(zone, reader)?))
                }
            },
        }
    }

    pub(super) fn cleanup(&self) -> Result<(), CleanupZoneTypeError> {
        match &self {
            Self::Chroot(chroot) => Ok(chroot.cleanup()?),
            Self::Jail(jail) => Ok(jail.cleanup()?),
        }
    }
}
