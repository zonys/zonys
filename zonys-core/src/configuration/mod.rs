mod directive;
mod reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use directive::*;
pub use reader::*;

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

impl<'a> ZoneConfiguration<&'a Zone> {
    pub fn file_path(&self) -> PathBuf {
        self.zone.paths().configuration_file()
    }

    pub fn reader(&self) -> Result<ZoneConfigurationReader, ReadZoneConfigurationError> {
        Ok(ZoneConfigurationReader::new(self.directive()?))
    }

    pub fn directive(&self) -> Result<ZoneConfigurationDirective, ReadZoneConfigurationError> {
        Ok(from_reader(BufReader::new(File::open(self.file_path())?))?)
    }

    pub fn set_directive(
        &self,
        persistence: &ZoneConfigurationDirective,
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
        writer.serialize(&self.directive()?)?;

        Ok(())
    }

    pub(super) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveZoneConfigurationError> {
        let configuration = Self::new(zone);
        configuration.set_directive(&reader.deserialize()?)?;

        Ok(configuration)
    }
}
