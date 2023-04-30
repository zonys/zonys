mod configuration;
mod error;
mod executor;
mod identifier;
mod iterator;
mod lock;
mod paths;
mod transmission;
mod volume;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use configuration::*;
pub use error::*;
pub use executor::*;
pub use identifier::*;
pub use iterator::*;
pub use lock::*;
pub use paths::*;
pub use transmission::*;
pub use volume::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::template::TemplateEngine;
use nix::unistd::{read, write};
use postcard::{from_bytes, to_allocvec};
use regex::Regex;
use reqwest::blocking::get;
use std::fmt::Debug;
use std::fs::{read_dir, File};
use std::io::{BufWriter, Seek, Write};
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::panic::{catch_unwind, resume_unwind};
use std::path::{Path, PathBuf};
use tar::Archive;
use tempfile::tempfile;
use url::{ParseError, Url};
use uuid::Uuid;
use xz2::read::XzDecoder;
use zfs::file_system::FileSystem;
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
pub enum ZoneFileSystem {
    Zfs,
    Directory,
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

    pub fn lock_owned(self) -> ZoneLock<Self> {
        ZoneLock::new(self)
    }

    pub fn status(&self) -> Result<ZoneStatus, ReadZoneStatusError> {
        Ok(ZoneStatus::new(false))
    }

    pub fn volume(&self) -> ZoneVolume<&Self> {
        ZoneVolume::new(self)
    }

    fn executor(&self) -> ZoneExecutor<&Self> {
        ZoneExecutor::new(self)
    }
}

impl Zone {
    fn handle_create_from_path(
        &self,
        path: &Path,
        overwritten_extension: Option<&str>,
    ) -> Result<(), CreateZoneError> {
        if path.is_file() {
            self.handle_create_from_local_file(
                File::open(path)?,
                overwritten_extension
                    .or(path
                        .extension()
                        .map(|extension| extension.to_str())
                        .flatten())
                    .unwrap_or(""),
            )
        } else {
            todo!()
        }
    }

    fn handle_create_from_local_file(
        &self,
        file: File,
        extension: &str,
    ) -> Result<(), CreateZoneError> {
        match extension {
            "txz" => {
                let mut archive = Archive::new(XzDecoder::new(file));

                archive
                    .unpack(self.paths().root_directory())
                    .map_err(CreateZoneError::from)
            }
            extension => Err(CreateZoneError::UnsupportedExtension(String::from(
                extension,
            ))),
        }
    }

    fn handle_create_from_remote_file(&self, url: &Url) -> Result<(), CreateZoneError> {
        match url.scheme() {
            "http" | "https" => {
                let mut response = get(url.to_string())?;
                let mut file = tempfile()?;
                response.copy_to(&mut file)?;
                file.sync_all()?;
                file.rewind()?;

                let path = PathBuf::from(url.path());

                self.handle_create_from_local_file(
                    file,
                    path.extension()
                        .map(|extension| extension.to_str())
                        .flatten()
                        .unwrap_or(""),
                )
            }
            scheme => Err(CreateZoneError::UnsupportedScheme(String::from(scheme))),
        }
    }

    fn handle_create_handle_from(&self, from: &str) -> Result<(), CreateZoneError> {
        match Url::parse(from) {
            Ok(url) if matches!(url.scheme(), "" | "file") => {
                self.handle_create_from_path(&PathBuf::from(url.path()), None)
            }
            Ok(url) => self.handle_create_from_remote_file(&url),
            Err(ParseError::RelativeUrlWithoutBase) => {
                self.handle_create_from_path(&PathBuf::from(from), None)
            }
            Err(error) => Err(CreateZoneError::from(error)),
        }
    }

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

        self.volume().create(&configuration_unit)?;

        if let Some(from) = configuration_unit
            .traverser()
            .inorder()
            .flat_map(|unit| unit.from().as_ref())
            .next()
        {
            self.handle_create_handle_from(from)?;
        }

        self.configuration().set_unit(&configuration_unit)?;

        self.executor().trigger_create()?;

        if configuration_unit
            .merged_start_after_create()
            .unwrap_or(false)
        {
            self.handle_start()?;
        }

        Ok(())
    }

    fn handle_start(&self) -> Result<(), StartZoneError> {
        if self.status()?.running() {
            return Err(StartZoneError::AlreadyRunning);
        }

        self.executor().trigger_start()?;

        Ok(())
    }

    fn handle_stop(&self) -> Result<bool, StopZoneError> {
        if !self.status()?.running() {
            return Err(StopZoneError::NotRunning);
        }

        self.executor().trigger_stop()?;

        if self
            .configuration()
            .unit()?
            .merged_destroy_after_stop()
            .unwrap_or(false)
        {
            self.handle_destroy()?;
            return Ok(true);
        };

        Ok(false)
    }

    fn handle_destroy(&self) -> Result<(), DestroyZoneError> {
        if self.status()?.running() {
            return Err(DestroyZoneError::IsRunning);
        }

        self.executor().trigger_destroy()?;
        self.configuration().destroy()?;
        self.volume().destroy()?;

        Ok(())
    }

    fn handle_send<T>(&self, writer: &mut T) -> Result<(), SendZoneError>
    where
        T: AsRawFd,
    {
        let mut file_system = match FileSystem::open(&self.identifier().clone().try_into()?)? {
            None => return Err(SendZoneError::MissingFileSystem),
            Some(f) => f,
        };

        let header = to_allocvec(&ZoneTransmissionHeader::Version1(
            ZoneTransmissionVersion1Header::new(
                to_allocvec(&self.configuration().unit()?)?,
                ZoneTransmissionVersion1Type::Zfs,
            ),
        ))?;

        write(
            writer.as_raw_fd(),
            &to_allocvec(&ZONE_TRANSMISSION_MAGIC_NUMBER)?,
        )?;

        write(
            writer.as_raw_fd(),
            &to_allocvec(&(header.len() as ZoneTransmissionHeaderLength))?,
        )?;

        write(writer.as_raw_fd(), &header)?;

        Ok(file_system.send(writer.as_raw_fd())?)
    }

    fn handle_receive<T>(&self, reader: &mut T) -> Result<(), ReceiveZoneError>
    where
        T: AsRawFd,
    {
        let mut buffer: [u8; size_of::<ZoneTransmissionMagicNumberLength>()] =
            [0; size_of::<ZoneTransmissionMagicNumberLength>()];

        if read(reader.as_raw_fd(), &mut buffer)? == 0 {
            return Err(ReceiveZoneError::EmptyInput);
        }

        let magic_number: ZoneTransmissionMagicNumberLength = from_bytes(&buffer)?;
        if magic_number != ZONE_TRANSMISSION_MAGIC_NUMBER {
            return Err(ReceiveZoneError::MissingMagicNumber);
        }

        read(reader.as_raw_fd(), &mut buffer)?;
        let header_len: ZoneTransmissionHeaderLength = from_bytes(&buffer)?;

        let mut header: Vec<u8> = vec![0; header_len as usize];
        read(reader.as_raw_fd(), &mut header)?;
        let header: ZoneTransmissionHeader = from_bytes(&buffer)?;

        match header {
            ZoneTransmissionHeader::Version1(version1) => {
                match version1.r#type() {
                    ZoneTransmissionVersion1Type::Zfs => {
                        FileSystem::receive(
                            self.identifier.clone().try_into()?,
                            reader.as_raw_fd(),
                        )?;
                    }
                };

                let writer = &mut BufWriter::new(File::create(self.configuration().file_path())?);
                writer.write(version1.configuration())?;
            }
        };

        Ok(())
    }

    fn cleanup(&self) -> Result<(), CleanupZoneError> {
        let mut cleanup_errors = Vec::default();

        if let Err(error) = self.configuration().cleanup() {
            cleanup_errors.push(CleanupZoneError::from(error));
        }

        if let Err(error) = self.volume().cleanup() {
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

    pub fn send<T>(&mut self, writer: &mut T) -> Result<(), SendZoneError>
    where
        T: AsRawFd,
    {
        self.lock().hold(|zone| zone.handle_send(writer))?
    }

    pub fn receive<T>(base_path: &Path, reader: &mut T) -> Result<ZoneIdentifier, ReceiveZoneError>
    where
        T: AsRawFd,
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
