pub mod configuration;
pub mod error;
pub mod execution;
pub mod identifier;
pub mod transmission;
pub use configuration::*;
pub use error::*;
pub use identifier::*;
pub use transmission::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::namespace::NamespaceIdentifier;
use crate::template::{TemplateObject, TemplateScalar, TemplateValue};
use ::jail::{Jail, JailId, JailName, JailParameter, TryIntoJailIdError};
use bincode::serde::{decode_from_slice, encode_to_vec};
use execution::*;
use nix::errno::Errno;
use nix::fcntl::{flock, FlockArg};
use nix::unistd::{read, write};
use serde_yaml::{from_reader, to_vec, to_writer};
use std::fs::{remove_file, File};
use std::io::{BufReader, BufWriter, Write};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use uuid::Uuid;
use zfs::file_system::identifier::FileSystemIdentifier;
use zfs::file_system::FileSystem;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ZONE_CONFIGURATION_PATH_EXTENSION: &str = "yaml";
pub const ZONE_LOCK_PATH_EXTENSION: &str = "lock";

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
    fn jail_name(&self) -> String {
        self.identifier().uuid().to_string()
    }

    fn jail(&self) -> Result<Option<Jail>, TryIntoJailIdError> {
        Ok(Option::<JailId>::try_from(JailName::new(self.jail_name()))?
            .map(Jail::open)
            .flatten())
    }

    fn jail_parameters(&self) -> Vec<JailParameter> {
        vec![
            JailParameter::new("persist", "true"),
            JailParameter::new("name", self.jail_name()),
            JailParameter::new("path", self.root_path().display().to_string()),
        ]
    }

    fn context_variables(&self) -> TemplateObject {
        let mut zone = TemplateObject::default();

        zone.insert(
            "identifier".into(),
            TemplateValue::Scalar(TemplateScalar::new(self.identifier().to_string())),
        );

        let mut paths = TemplateObject::default();
        paths.insert(
            "root".into(),
            TemplateValue::Scalar(TemplateScalar::new(self.root_path().display().to_string())),
        );

        zone.insert("paths".into(), paths.into());

        zone
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
            remove_file(&self.lock_path())?;
        }

        Ok(())
    }
}

impl Zone {
    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn root_path(&self) -> PathBuf {
        let mut path = PathBuf::from("/");
        path.push(self.identifier.to_string());

        path
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
                configuration_path,
            ))?;

            Ok(configuration)
        } else {
            Ok(ZoneConfiguration::default())
        }
    }

    pub fn is_running(&self) -> Result<bool, RetrieveZoneRunningStatusError> {
        Ok(self
            .jail()
            .map_err(RetrieveZoneRunningStatusError::TryIntoJailIdError)?
            .is_some())
    }
}

impl Zone {
    fn handle_create(&mut self, configuration: ZoneConfiguration) -> Result<(), CreateZoneError> {
        let processor = ZoneConfigurationProcessor::default();
        let configuration = processor.process(configuration)?;

        let mut writer = &mut BufWriter::new(File::create(self.configuration_path())?);
        to_writer(&mut writer, configuration.directive())?;
        writer.flush()?;

        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context
            .variables_mut()
            .extend(match configuration.directive().version() {
                ZoneConfigurationVersionDirective::Version1(ref version1) => version1
                    .variables()
                    .as_ref()
                    .map(|x| x.clone())
                    .unwrap_or_default(),
            });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        OperateCreateBeforeZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        let file_system_identifier = FileSystemIdentifier::from(self.identifier().clone());
        FileSystem::create(&file_system_identifier)?;
        let mut file_system = FileSystem::open(&file_system_identifier)?
            .ok_or(CreateZoneError::FileSystemNotExisting)?;
        file_system.mount()?;

        let mut jail = Jail::create(self.jail_parameters())?;

        OperateCreateOnZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        OperateCreateAfterZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        let start_after_create = match configuration.directive().version() {
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
        if self.jail()?.is_some() {
            return Err(StartZoneError::AlreadyRunning);
        }

        let configuration = self.configuration()?;
        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context
            .variables_mut()
            .extend(match configuration.directive().version() {
                ZoneConfigurationVersionDirective::Version1(ref version1) => version1
                    .variables()
                    .as_ref()
                    .map(|x| x.clone())
                    .unwrap_or_default(),
            });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        ExecuteStartBeforeZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        let mut jail = Jail::create(self.jail_parameters())?;

        ExecuteStartOnZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteStartAfterZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        Ok(())
    }

    fn handle_stop(self) -> Result<Option<Self>, StopZoneError> {
        let mut jail = match self.jail() {
            Ok(Some(j)) => j,
            Ok(None) => return Err(StopZoneError::NotRunning),
            Err(e) => return Err(e.into()),
        };

        let configuration = self.configuration()?;
        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context
            .variables_mut()
            .extend(match configuration.directive().version() {
                ZoneConfigurationVersionDirective::Version1(ref version1) => version1
                    .variables()
                    .as_ref()
                    .map(|x| x.clone())
                    .unwrap_or_default(),
            });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        ExecuteStopBeforeZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteStopOnZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        ExecuteStopAfterZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        let destroy_after_stop = match configuration.directive().version() {
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

    fn handle_destroy(self) -> Result<(), DestroyZoneError> {
        if self.jail()?.is_some() {
            return Err(DestroyZoneError::IsRunning);
        }

        let file_system_identifier = FileSystemIdentifier::from(self.identifier().clone());
        let mut file_system = FileSystem::open(&file_system_identifier)?
            .ok_or(DestroyZoneError::FileSystemNotExisting)?;
        let configuration = self.configuration()?;
        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context
            .variables_mut()
            .extend(match configuration.directive().version() {
                ZoneConfigurationVersionDirective::Version1(ref version1) => version1
                    .variables()
                    .as_ref()
                    .map(|x| x.clone())
                    .unwrap_or_default(),
            });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        let mut jail = Jail::create(self.jail_parameters())?;

        OperateDestroyBeforeZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        OperateDestroyOnZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        if file_system.mount_status()?.is_mounted() {
            file_system.unmount_all()?;
        }

        file_system.destroy()?;

        OperateDestroyAfterZoneProgramExecutionIterator::new(configuration.directive())
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        let configuration_path = self.configuration_path();
        if configuration_path.exists() {
            remove_file(configuration_path)?;
        }

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
        if self.jail()?.is_some() {
            return Err(SendZoneError::ZoneIsRunning);
        }

        let mut file_system = match FileSystem::open(&self.identifier().clone().into())? {
            None => return Err(SendZoneError::MissingFileSystem),
            Some(f) => f,
        };

        let bincode_configuration = create_bincode_configuration();

        let header = encode_to_vec(
            ZoneTransmissionHeader::Version1(ZoneTransmissionVersion1Header::new(
                to_vec(&self.configuration()?.directive())?,
                ZoneTransmissionVersion1Type::Zfs,
            )),
            bincode_configuration,
        )?;

        write(
            writer.as_raw_fd(),
            &encode_to_vec(ZONE_TRANSMISSION_MAGIC_NUMBER, bincode_configuration)?,
        )?;

        write(
            writer.as_raw_fd(),
            &encode_to_vec(
                header.len() as ZoneTransmissionHeaderLength,
                bincode_configuration,
            )?,
        )?;

        write(writer.as_raw_fd(), &header)?;

        Ok(file_system.send(writer.as_raw_fd())?)
    }

    fn handle_receive<T>(&mut self, reader: &mut T) -> Result<(), ReceiveZoneError>
    where
        T: AsRawFd,
    {
        let bincode_configuration = create_bincode_configuration();
        let mut buffer: [u8; 8] = [0; 8];

        if read(reader.as_raw_fd(), &mut buffer)? == 0 {
            return Err(ReceiveZoneError::EmptyInput);
        }

        let (magic_number, _): (ZoneTransmissionMagicNumberLength, _) =
            decode_from_slice(&buffer, bincode_configuration)?;
        if magic_number != ZONE_TRANSMISSION_MAGIC_NUMBER {
            return Err(ReceiveZoneError::MissingMagicNumber);
        }

        read(reader.as_raw_fd(), &mut buffer)?;
        let (header_len, _): (ZoneTransmissionHeaderLength, _) =
            decode_from_slice(&buffer, bincode_configuration)?;

        let mut header: Vec<u8> = vec![0; header_len as usize];
        read(reader.as_raw_fd(), &mut header)?;
        let (header, _): (ZoneTransmissionHeader, _) =
            decode_from_slice(&header, bincode_configuration)?;

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

        match zone.jail()? {
            Some(j) => j.destroy()?,
            None => {}
        }

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
