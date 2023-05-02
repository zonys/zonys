mod directive;
mod transform;
mod traverser;
mod unit;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use directive::*;
pub use transform::*;
pub use traverser::*;
pub use unit::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    DeserializeZoneTransmissionError, SerializeZoneTransmissionError, Zone, ZoneTransmissionReader,
    ZoneTransmissionWriter,
};
use serde_yaml::{from_reader, to_writer};
use std::fs::{remove_file, File};
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReadZoneConfigurationError {
    YamlError(serde_yaml::Error),
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum WriteZoneConfigurationError {
    YamlError(serde_yaml::Error),
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyZoneConfigurationError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupZoneConfigurationError {
    IOError(io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendZoneConfigurationError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    SerializeZoneTransmissionError(SerializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveZoneConfigurationError {
    WriteZoneConfigurationError(WriteZoneConfigurationError),
    DeserializeZoneTransmissionError(DeserializeZoneTransmissionError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
pub struct ZoneConfiguration<T> {
    zone: T,
}

impl ZoneConfiguration<&Zone> {
    pub fn file_path(&self) -> PathBuf {
        self.zone.paths().configuration_file()
    }

    pub fn unit(&self) -> Result<ZoneConfigurationUnit, ReadZoneConfigurationError> {
        Ok(from_reader(BufReader::new(File::open(self.file_path())?))?)
    }

    pub fn set_unit(
        &self,
        persistence: &ZoneConfigurationUnit,
    ) -> Result<(), WriteZoneConfigurationError> {
        Ok(to_writer(
            BufWriter::new(File::create(self.file_path())?),
            persistence,
        )?)
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyZoneConfigurationError> {
        remove_file(self.file_path())?;

        Ok(())
    }

    pub(super) fn cleanup(&self) -> Result<(), CleanupZoneConfigurationError> {
        let path = self.file_path();
        if path.exists() {
            remove_file(path)?;
        }

        Ok(())
    }

    pub(super) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendZoneConfigurationError> {
        writer.serialize(&self.unit()?)?;

        Ok(())
    }

    pub(super) fn receive(
        &self,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<(), ReceiveZoneConfigurationError> {
        self.set_unit(&reader.deserialize::<ZoneConfigurationUnit>()?)?;

        Ok(())
    }
}
