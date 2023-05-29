#![forbid(unsafe_code)]
#![feature(exit_status_error)]

////////////////////////////////////////////////////////////////////////////////////////////////////

mod configuration;
mod error;
mod handler;
mod identifier;
mod iterator;
mod lock;
mod paths;
mod template;
mod transmission;
mod r#type;
mod volume;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use configuration::*;
pub use error::*;
pub use handler::*;
pub use identifier::*;
pub use iterator::*;
pub use lock::*;
pub use paths::*;
pub use r#type::*;
pub use template::*;
pub use transmission::*;
pub use volume::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::template::TemplateEngine;
use byteorder::{ReadBytesExt, WriteBytesExt};
use regex::Regex;
use std::fmt::Debug;
use std::fs::read_dir;
use std::os::unix::io::AsRawFd;
use std::panic::{catch_unwind, resume_unwind};
use std::path::Path;
use uuid::Uuid;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Constructor, Method)]
#[Constructor(visibility = pub(self))]
#[Method(accessors)]
pub struct ZoneStatus {
    running: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Zone {
    identifier: ZoneIdentifier,
}

impl Zone {
    fn new(identifier: ZoneIdentifier) -> Self {
        Self { identifier }
    }
}

impl Zone {
    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn paths(&self) -> ZonePaths<&Self> {
        ZonePaths::new(self)
    }

    pub fn configuration(&self) -> ZoneConfiguration<&Self> {
        ZoneConfiguration::new(self)
    }

    pub fn lock(&self) -> ZoneLock<&Self> {
        ZoneLock::new(self)
    }

    pub fn status(&self) -> Result<ZoneStatus, ReadZoneStatusError> {
        Ok(ZoneStatus::new(false))
    }

    pub fn r#type(&self) -> Result<ZoneType<&Self>, ReadZoneConfigurationError> {
        ZoneType::new(self)
    }
}

impl Zone {
    fn handle_create(
        &self,
        configuration_path: &Path,
        configuration_directive: ZoneConfigurationDirective,
    ) -> Result<(), CreateZoneError> {
        let variables = configuration_directive
            .variables()
            .clone()
            .unwrap_or_default();
        let configuration_unit =
            configuration_directive.transform(&mut TransformZoneConfigurationContext::new(
                TemplateEngine::default(),
                variables,
                vec![configuration_path.to_path_buf()],
            ))?;

        self.configuration().set_unit(&configuration_unit)?;
        let reader = self.configuration().reader()?;
        self.r#type()?.create()?;

        if reader.start_after_create() {
            self.handle_start()?;
        }

        Ok(())
    }

    fn handle_start(&self) -> Result<(), StartZoneError> {
        self.r#type()?.start()?;

        Ok(())
    }

    fn handle_stop(&self) -> Result<bool, StopZoneError> {
        self.r#type()?.stop()?;

        let reader = self.configuration().reader()?;

        if reader.destroy_after_stop() {
            self.handle_destroy()?;
            return Ok(true);
        };

        Ok(false)
    }

    fn handle_destroy(&self) -> Result<(), DestroyZoneError> {
        if self.status()?.running() {
            return Err(DestroyZoneError::IsRunning);
        }

        self.r#type()?.destroy()?;
        self.configuration().destroy()?;

        Ok(())
    }

    fn handle_send<T>(&self, writer: &mut T) -> Result<(), SendZoneError>
    where
        T: AsRawFd + 'static,
    {
        let mut writer = ZoneTransmissionWriter::new(writer.as_raw_fd());
        writer.write_u64::<ZoneTransmissionEndian>(ZONE_TRANSMISSION_MAGIC_NUMBER)?;

        self.configuration().send(&mut writer)?;
        self.r#type()?.send(&mut writer)?;

        Ok(())
    }

    fn handle_receive<T>(&self, reader: &mut T) -> Result<(), ReceiveZoneError>
    where
        T: AsRawFd + 'static,
    {
        let mut reader = ZoneTransmissionReader::new(reader.as_raw_fd());

        let magic_number: ZoneTransmissionMagicNumberLength =
            reader.read_u64::<ZoneTransmissionEndian>()?;

        if magic_number != ZONE_TRANSMISSION_MAGIC_NUMBER {
            return Err(ReceiveZoneError::MissingMagicNumber);
        }

        ZoneConfiguration::receive(self, &mut reader)?;
        ZoneType::receive(self, &mut reader)?;

        Ok(())
    }

    fn cleanup(&self) -> Result<(), CleanupZoneError> {
        let mut cleanup_errors = Vec::default();

        if let Ok(r#type) = self.r#type() {
            if let Err(error) = r#type.cleanup() {
                cleanup_errors.push(CleanupZoneError::from(error));
            }
        };

        if let Err(error) = self.configuration().cleanup() {
            cleanup_errors.push(CleanupZoneError::from(error));
        }

        if let Err(error) = self.lock().cleanup() {
            cleanup_errors.push(CleanupZoneError::from(error));
        }

        if cleanup_errors.len() > 1 {
            return Err(CleanupZoneError::from(cleanup_errors));
        }

        if let Some(error) = cleanup_errors.pop() {
            return Err(error);
        }

        Ok(())
    }
}

impl Zone {
    pub fn open(identifier: ZoneIdentifier) -> Result<Option<Self>, OpenZoneError> {
        let zone = Self::new(identifier);

        if !zone.paths().configuration_file().is_file() {
            return Ok(None);
        }

        Ok(Some(zone))
    }

    pub fn create(
        base_path: &Path,
        configuration_path: &Path,
        configuration_directive: ZoneConfigurationDirective,
    ) -> Result<ZoneIdentifier, CreateZoneError> {
        let identifier = ZoneIdentifier::new(base_path.try_into()?, Uuid::new_v4());
        let zone = Self::new(identifier.clone());

        let result = catch_unwind(move || {
            zone.lock()
                .hold(move |zone| zone.handle_create(configuration_path, configuration_directive))?
        });

        match result {
            Ok(Ok(())) => Ok(identifier),
            Ok(Err(error)) => {
                Self::new(identifier).cleanup()?;
                Err(error)
            }
            Err(error) => {
                // TODO: Check
                Self::new(identifier).cleanup()?;
                resume_unwind(error)
            }
        }
    }
}

impl Zone {
    pub fn start(&mut self) -> Result<(), StartZoneError> {
        self.lock().hold(|zone| zone.handle_start())?
    }

    pub fn stop(self) -> Result<Option<Self>, StopZoneError> {
        let result = self.lock().hold(|zone| zone.handle_stop())??;

        if result {
            return Ok(None);
        }

        Ok(Some(self))
    }

    pub fn destroy(self) -> Result<(), DestroyZoneError> {
        self.lock().hold(|zone| zone.handle_destroy())?
    }

    pub fn send<T>(&self, writer: &mut T) -> Result<(), SendZoneError>
    where
        T: AsRawFd + 'static,
    {
        self.lock().hold(|zone| zone.handle_send(writer))?
    }

    pub fn receive<T>(base_path: &Path, reader: &mut T) -> Result<ZoneIdentifier, ReceiveZoneError>
    where
        T: AsRawFd + 'static,
    {
        let zone = Self::new(ZoneIdentifier::new(base_path.try_into()?, Uuid::new_v4()));

        zone.lock().hold(|zone| zone.handle_receive(reader))??;

        Ok(zone.identifier)
    }

    pub fn all(base_path: &Path) -> Result<AllZoneIterator, AllZoneIteratorError> {
        Ok(AllZoneIterator::new(read_dir(base_path)?))
    }

    pub fn r#match(
        base_path: &Path,
        regular_expression: &String,
    ) -> Result<MatchZoneIterator, MatchZoneIteratorError> {
        Ok(MatchZoneIterator::new(
            AllZoneIterator::new(read_dir(base_path)?),
            Regex::new(&format!("^{}$", regular_expression))?,
        ))
    }
}
