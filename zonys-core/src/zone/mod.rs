pub mod configuration;
pub mod error;
mod execution;
pub mod identifier;
pub use configuration::*;
pub use error::*;
pub use identifier::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::namespace::{Namespace, NamespaceIdentifier};
use crate::template::{TemplateEngine, TemplateObject, TemplateScalar, TemplateValue};
use crate::utility::try_catch;
use ::jail::{Jail, JailId, JailName, JailParameter, TryIntoJailIdError};
use execution::*;
use liquid::{ObjectView, Parser};
use nix::errno::Errno;
use nix::fcntl::{flock, FlockArg};
use serde_yaml::{from_reader, to_writer};
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{remove_file, File};
use std::io;
use std::io::{BufReader, BufWriter};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use uuid::Uuid;
use zfs::file_system::{ChildIterator, FileSystem};

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
    fn file_system(&self) -> Result<FileSystem, zfs::Error> {
        match FileSystem::open(&self.identifier.to_string()) {
            Err(e) => Err(e.into()),
            Ok(None) => panic!(),
            Ok(Some(f)) => Ok(f),
        }
    }

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
            Ok(from_reader(&mut BufReader::new(File::open(
                configuration_path,
            )?))?)
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
        let configuration_file = File::create(self.configuration_path())?;
        to_writer(&mut BufWriter::new(configuration_file), &configuration)?;

        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context.variables_mut().extend(match configuration {
            ZoneConfiguration::Version1(ref version1) => version1
                .variables()
                .as_ref()
                .map(|x| x.clone())
                .unwrap_or_default(),
        });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        ExecuteCreateBeforeZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        FileSystem::create(&self.identifier().to_string())?;
        let mut file_system = FileSystem::open(&&self.identifier().to_string())?
            .ok_or(CreateZoneError::FileSystemNotExisting)?;
        file_system.mount()?;

        let mut jail = Jail::create(self.jail_parameters())?;

        ExecuteCreateOnZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteCreateAfterZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        Ok(())
    }

    fn handle_start(&mut self) -> Result<(), StartZoneError> {
        if self.jail()?.is_some() {
            return Err(StartZoneError::AlreadyRunning);
        }

        let configuration = self.configuration()?;
        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context.variables_mut().extend(match configuration {
            ZoneConfiguration::Version1(ref version1) => version1
                .variables()
                .as_ref()
                .map(|x| x.clone())
                .unwrap_or_default(),
        });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        ExecuteStartBeforeZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        let mut jail = Jail::create(self.jail_parameters())?;

        ExecuteStartOnZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteStartAfterZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        Ok(())
    }

    fn handle_stop(&mut self) -> Result<(), StopZoneError> {
        let mut jail = match self.jail() {
            Ok(Some(j)) => j,
            Ok(None) => return Err(StopZoneError::NotRunning),
            Err(e) => return Err(e.into()),
        };

        let configuration = self.configuration()?;
        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context.variables_mut().extend(match configuration {
            ZoneConfiguration::Version1(ref version1) => version1
                .variables()
                .as_ref()
                .map(|x| x.clone())
                .unwrap_or_default(),
        });
        context.variables_mut().insert(
            "zone".into(),
            TemplateValue::Object(self.context_variables()),
        );

        ExecuteStopBeforeZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteStopOnZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        ExecuteStopAfterZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        Ok(())
    }

    fn handle_destroy(&mut self) -> Result<(), DestroyZoneError> {
        if self.jail()?.is_some() {
            return Err(DestroyZoneError::IsRunning);
        }

        let mut file_system = self.file_system()?;
        let configuration = self.configuration()?;
        let executor = ZoneExecutor::default();

        let mut context = ZoneExecutionContext::default();
        context.variables_mut().extend(match configuration {
            ZoneConfiguration::Version1(ref version1) => version1
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

        ExecuteDestroyBeforeZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteDestroyOnZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        if file_system.is_mounted()? {
            file_system.unmount_all()?;
        }

        file_system.destroy()?;

        ExecuteDestroyAfterZoneExecutionInstructionIterator::new(&configuration)
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
}

impl Zone {
    pub fn open<'a, T>(identifier: T) -> Result<Option<Self>, OpenZoneError>
    where
        T: Into<Cow<'a, ZoneIdentifier>>,
    {
        let identifier = identifier.into();

        match FileSystem::open(&identifier.to_string()) {
            Err(e) => Err(e.into()),
            Ok(None) => Ok(None),
            Ok(Some(_)) => Ok(Some(Self::new(identifier.into_owned(), None))),
        }
    }

    pub fn create<'a, T>(
        namespace_identifier: T,
        configuration: ZoneConfiguration,
    ) -> Result<ZoneIdentifier, CreateZoneError>
    where
        T: Into<Cow<'a, NamespaceIdentifier>>,
    {
        let mut zone = Self::new(
            ZoneIdentifier::new(namespace_identifier.into().into_owned(), Uuid::new_v4()),
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

        match FileSystem::open(&&zone.identifier().to_string())? {
            Some(mut file_system) => {
                if file_system.is_mounted()? {
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

    pub fn stop(&mut self) -> Result<(), StopZoneError> {
        self.lock()?;
        let result = self.handle_stop();
        self.unlock()?;

        result
    }

    pub fn destroy(mut self) -> Result<(), DestroyZoneError> {
        self.lock()?;
        let result = self.handle_destroy();
        self.unlock()?;

        result
    }
}
