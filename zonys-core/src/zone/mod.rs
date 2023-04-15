pub mod configuration;
pub mod error;
pub mod executor;
pub mod identifier;
pub mod transmission;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use configuration::*;
pub use error::*;
pub use executor::*;
pub use identifier::*;
pub use transmission::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::namespace::NamespaceIdentifier;
use crate::template::{TemplateEngine, TemplateObject, TemplateScalar, TemplateValue};
use nix::errno::Errno;
use nix::fcntl::{flock, FlockArg};
use nix::unistd::{read, write};
use postcard::{from_bytes, to_allocvec};
use reqwest::blocking::get;
use serde_yaml::{from_reader, to_writer};
use std::fs::{create_dir_all, remove_dir_all, remove_file, File};
use std::io::{BufReader, BufWriter, Seek, Write};
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use tar::Archive;
use tempfile::tempfile;
use uuid::Uuid;
use xz2::read::XzDecoder;
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::file_system::FileSystem;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ZONE_CONFIGURATION_PATH_EXTENSION: &str = "yaml";
pub const ZONE_LOCK_PATH_EXTENSION: &str = "lock";

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ZoneFileSystem {
    Zfs,
    Directory,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Zone {
    identifier: ZoneIdentifier,
    lock_file: Option<File>,
}

impl Zone {
    fn new(identifier: ZoneIdentifier, lock_file: Option<File>) -> Self {
        Self {
            identifier,
            lock_file,
        }
    }
}

impl Zone {
    fn executor(&self) -> Box<dyn ZoneExecutor> {
        Box::new(JailZoneExecutor::new())
    }

    fn zone_paths_variables(&self) -> TemplateObject {
        let mut paths = TemplateObject::default();

        paths.insert(
            "root".into(),
            TemplateValue::Scalar(TemplateScalar::new(self.root_path().display().to_string())),
        );

        paths
    }

    fn zone_variables(&self) -> TemplateObject {
        let mut zone = TemplateObject::default();

        zone.insert(
            "identifier".into(),
            TemplateValue::Scalar(TemplateScalar::new(self.identifier().to_string())),
        );

        zone.insert(
            "paths".into(),
            TemplateValue::Object(self.zone_paths_variables()),
        );

        zone
    }

    fn variables(&self) -> TemplateObject {
        let mut root = TemplateObject::default();

        root.insert("zone".into(), TemplateValue::Object(self.zone_variables()));

        root
    }

    fn lock(&mut self) -> Result<(), LockZoneError> {
        if self.lock_file.is_some() {
            return Err(LockZoneError::AlreadyLocked);
        }

        let file = File::create(self.lock_path())?;
        let raw_fd = file.as_raw_fd();
        self.lock_file = Some(file);

        match flock(raw_fd, FlockArg::LockExclusiveNonblock) {
            Err(Errno::EAGAIN) => Err(LockZoneError::AlreadyLocked),
            Err(e) => Err(e.into()),
            Ok(()) => Ok(()),
        }
    }

    fn unlock(&mut self) -> Result<(), UnlockZoneError> {
        match &self.lock_file {
            None => return Err(UnlockZoneError::NotLocked),
            Some(file) => {
                flock(file.as_raw_fd(), FlockArg::UnlockNonblock)?;
            }
        };

        let lock_path = self.lock_path();
        self.lock_file = None;
        if lock_path.exists() {
            remove_file(self.lock_path())?;
        }

        Ok(())
    }
}

impl Zone {
    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn root_path(&self) -> PathBuf {
        self.identifier.clone().into()
    }

    pub fn configuration_path(&self) -> PathBuf {
        self.root_path()
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/"))
            .join(format!(
                "{}.{}",
                self.identifier.uuid(),
                ZONE_CONFIGURATION_PATH_EXTENSION
            ))
    }

    pub fn lock_path(&self) -> PathBuf {
        self.root_path()
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/"))
            .join(format!(
                "{}.{}",
                self.identifier.uuid(),
                ZONE_LOCK_PATH_EXTENSION
            ))
    }

    pub fn configuration(&self) -> Result<ZoneConfiguration, OpenZoneConfigurationError> {
        let configuration_path = self.configuration_path();
        if configuration_path.exists() {
            let processor = ZoneConfigurationProcessor::default();

            let configuration = processor.process(ZoneConfiguration::new(
                from_reader(&mut BufReader::new(File::open(&configuration_path)?))?,
                Vec::default(),
                configuration_path,
            ))?;

            Ok(configuration)
        } else {
            Ok(ZoneConfiguration::default())
        }
    }

    pub fn running(&self) -> Result<bool, RetrieveZoneRunningStatusError> {
        Ok(self
            .executor()
            .running(RunningZoneExecutorEvent::new(
                self.identifier.clone(),
                false,
            ))?
            .running())
    }
}

impl Zone {
    fn handle_create_write_configuration(
        &mut self,
        configuration: &ZoneConfiguration,
    ) -> Result<(), CreateZoneError> {
        let mut writer = &mut BufWriter::new(File::create(self.configuration_path())?);
        to_writer(&mut writer, configuration.directive())?;
        writer.flush().map_err(CreateZoneError::from)
    }

    fn handle_create_file_system(
        &mut self,
        configuration: &ZoneConfiguration,
    ) -> Result<(), CreateZoneError> {
        let file_system =
            configuration
                .directives()
                .read_first(|directive| match directive.version() {
                    ZoneConfigurationVersionDirective::Version1(version1) => {
                        version1.file_system().as_ref()
                    }
                });

        match file_system {
            Some(version1::ZoneConfigurationFileSystemDirective::Automatic) => {
                panic!("Currently unsupported");
            }
            // TODO
            Some(version1::ZoneConfigurationFileSystemDirective::Zfs) | None => {
                let file_system_identifier = FileSystemIdentifier::from(self.identifier().clone());
                FileSystem::create(&file_system_identifier)?;
                let mut file_system = FileSystem::open(&file_system_identifier)?
                    .ok_or(CreateZoneError::FileSystemNotExisting)?;
                file_system.mount()?;
            }
            Some(version1::ZoneConfigurationFileSystemDirective::Directory) => {
                create_dir_all(self.root_path())?;
            }
        }

        Ok(())
    }

    fn handle_create_from(&mut self, from: &str) -> Result<(), CreateZoneError> {
        let mut response = get(from)?;
        let mut file = tempfile()?;
        response.copy_to(&mut file).unwrap();
        file.sync_all()?;
        file.rewind()?;

        let mut archive = Archive::new(XzDecoder::new(file));
        archive
            .unpack(self.root_path())
            .map_err(CreateZoneError::from)
    }

    fn handle_create(&mut self, configuration: ZoneConfiguration) -> Result<(), CreateZoneError> {
        let configuration = ZoneConfigurationProcessor::default().process(configuration)?;

        self.handle_create_write_configuration(&configuration)?;

        self.handle_create_file_system(&configuration)?;

        let mut variables = match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                version1.variables().as_ref().cloned().unwrap_or_default()
            }
        };

        variables.extend(self.variables().into_iter());

        let template_engine = TemplateEngine::default();

        if let Some(from) = configuration.directive().from() {
            self.handle_create_from(&template_engine.render(&variables, from)?)?;
        }

        let event = self.executor().create(CreateZoneExecutorEvent::new(
            self.identifier.clone(),
            configuration,
            self.root_path(),
            template_engine,
            variables,
        ))?;

        let start_after_create = match event.configuration().directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                version1.start_after_create().unwrap_or(false)
            }
        };

        if start_after_create {
            self.handle_start()?;
        }

        Ok(())
    }

    fn handle_start(&mut self) -> Result<(), StartZoneError> {
        let configuration = self.configuration()?;

        let mut variables = match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                version1.variables().as_ref().cloned().unwrap_or_default()
            }
        };

        variables.extend(self.variables().into_iter());

        self.executor().start(StartZoneExecutorEvent::new(
            self.identifier.clone(),
            configuration,
            self.root_path(),
            TemplateEngine::default(),
            variables,
        ))?;

        Ok(())
    }

    fn handle_stop(self) -> Result<Option<Self>, StopZoneError> {
        let configuration = self.configuration()?;

        let mut variables = match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                version1.variables().as_ref().cloned().unwrap_or_default()
            }
        };

        variables.extend(self.variables().into_iter());

        let event = self.executor().stop(StopZoneExecutorEvent::new(
            self.identifier.clone(),
            configuration,
            self.root_path(),
            TemplateEngine::default(),
            variables,
        ))?;

        let destroy_after_stop = match event.configuration().directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                version1.destroy_after_stop().unwrap_or(false)
            }
        };

        if destroy_after_stop {
            self.handle_destroy()?;

            Ok(None)
        } else {
            Ok(Some(self))
        }
    }

    fn handle_destroy_file_system(
        &self,
        configuration: &ZoneConfiguration,
    ) -> Result<(), DestroyZoneError> {
        match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                match version1.file_system() {
                    Some(version1::ZoneConfigurationFileSystemDirective::Automatic) => {
                        panic!("Currently unsupported");
                    }
                    // TODO
                    Some(version1::ZoneConfigurationFileSystemDirective::Zfs) | None => {
                        let file_system_identifier =
                            FileSystemIdentifier::from(self.identifier().clone());
                        let mut file_system = FileSystem::open(&file_system_identifier)?
                            .ok_or(DestroyZoneError::FileSystemNotExisting)?;

                        if file_system.mount_status()?.is_mounted() {
                            file_system.unmount_all()?;
                        }

                        file_system.destroy()?;
                    }
                    Some(version1::ZoneConfigurationFileSystemDirective::Directory) => {
                        remove_dir_all(self.root_path())?;
                    }
                }
            }
        };

        Ok(())
    }

    fn handle_destroy_remove_configuration(&self) -> Result<(), DestroyZoneError> {
        let configuration_path = self.configuration_path();
        if configuration_path.exists() {
            remove_file(configuration_path)?;
        }

        Ok(())
    }

    fn handle_destroy(self) -> Result<(), DestroyZoneError> {
        let configuration = self.configuration()?;

        let mut variables = match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                version1.variables().as_ref().cloned().unwrap_or_default()
            }
        };

        variables.extend(self.variables().into_iter());

        let event = self
            .executor()
            .destroy(DestroyZoneExecutorEvent::new(
                self.identifier.clone(),
                configuration,
                self.root_path(),
                TemplateEngine::default(),
                variables,
            ))?
            .into_record();

        self.handle_destroy_file_system(&event.configuration)?;

        self.handle_destroy_remove_configuration()?;

        let lock_path = self.lock_path();
        if lock_path.exists() {
            remove_file(lock_path)?;
        }

        Ok(())
    }

    fn handle_send<T>(&mut self, writer: &mut T) -> Result<(), SendZoneError>
    where
        T: AsRawFd,
    {
        if self.running()? {
            return Err(SendZoneError::ZoneIsRunning);
        }

        let mut file_system = match FileSystem::open(&self.identifier().clone().into())? {
            None => return Err(SendZoneError::MissingFileSystem),
            Some(f) => f,
        };

        let header = to_allocvec(&ZoneTransmissionHeader::Version1(
            ZoneTransmissionVersion1Header::new(
                to_allocvec(&self.configuration()?.directive())?,
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

    fn handle_receive<T>(&mut self, reader: &mut T) -> Result<(), ReceiveZoneError>
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
                        FileSystem::receive(self.identifier.clone().into(), reader.as_raw_fd())?;
                    }
                };

                let writer = &mut BufWriter::new(File::create(self.configuration_path())?);
                writer.write(version1.configuration())?;
            }
        };

        Ok(())
    }
}

impl Zone {
    pub fn open(identifier: ZoneIdentifier) -> Result<Option<Self>, OpenZoneError> {
        match FileSystem::open(&FileSystemIdentifier::from(identifier.clone())) {
            Err(e) => Err(e.into()),
            Ok(None) => Ok(None),
            Ok(Some(_)) => Ok(Some(Self::new(identifier, None))),
        }
    }

    pub fn create(
        namespace_identifier: NamespaceIdentifier,
        configuration: ZoneConfiguration,
    ) -> Result<ZoneIdentifier, CreateZoneError> {
        let mut zone = Self::new(
            ZoneIdentifier::new(namespace_identifier, Uuid::new_v4()),
            None,
        );

        zone.lock()?;
        let result = zone.handle_create(configuration);
        zone.unlock()?;

        let error = match result {
            Ok(()) => return Ok(zone.identifier),
            Err(e) => e,
        };

        let configuration_path = zone.configuration_path();
        if configuration_path.exists() {
            remove_file(configuration_path)?;
        }

        let lock_path = zone.lock_path();
        if lock_path.exists() {
            remove_file(lock_path)?;
        }

        let file_system_identifier = FileSystemIdentifier::from(zone.identifier().clone());
        match FileSystem::open(&file_system_identifier)? {
            Some(mut file_system) => {
                if file_system.mount_status()?.is_mounted() {
                    file_system.unmount_all()?;
                }

                file_system.destroy()?;
            }
            None => {}
        };

        Err(error)
    }
}

impl Zone {
    pub fn start(&mut self) -> Result<(), StartZoneError> {
        self.lock()?;
        let result = self.handle_start();
        self.unlock()?;

        result
    }

    pub fn stop(mut self) -> Result<Option<Self>, StopZoneError> {
        self.lock()?;
        match self.handle_stop()? {
            Some(mut zone) => {
                zone.unlock()?;

                Ok(Some(zone))
            }
            None => Ok(None),
        }
    }

    pub fn destroy(mut self) -> Result<(), DestroyZoneError> {
        self.lock()?;
        self.handle_destroy()
    }

    pub fn send<T>(&mut self, writer: &mut T) -> Result<(), SendZoneError>
    where
        T: AsRawFd,
    {
        self.lock()?;
        let result = self.handle_send(writer);
        self.unlock()?;

        result
    }

    pub fn receive<T>(
        namespace_identifier: NamespaceIdentifier,
        reader: &mut T,
    ) -> Result<ZoneIdentifier, ReceiveZoneError>
    where
        T: AsRawFd,
    {
        let mut zone = Self::new(
            ZoneIdentifier::new(namespace_identifier, Uuid::new_v4()),
            None,
        );
        zone.lock()?;
        let result = zone.handle_receive(reader);
        zone.unlock()?;

        result?;

        Ok(zone.identifier)
    }
}
