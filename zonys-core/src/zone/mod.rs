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
use ::jail::{Jail, JailId, JailName, JailParameter, TryIntoJailIdError};
use execution::*;
use liquid::{ObjectView, Parser};
use serde_yaml::{from_reader, to_writer};
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{create_dir, File};
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use uuid::Uuid;
use zfs::file_system::{ChildIterator, FileSystem};

////////////////////////////////////////////////////////////////////////////////////////////////////

const ZONE_ROOT_PATH_NAME: &str = "root";
const ZONE_CONFIGURATION_PATH_NAME: &str = "configuration";
const ZONE_LOCKFILE_PATH_NAME: &str = "lock";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum ZoneStatus {
    Running,
    NotRunning,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Zone {
    identifier: ZoneIdentifier,
}

impl Zone {
    fn new(identifier: ZoneIdentifier) -> Self {
        Self { identifier }
    }
}

impl Zone {
    fn configuration(&self) -> Result<ZoneConfiguration, OpenZoneConfigurationError> {
        Ok(from_reader(&mut BufReader::new(File::open(
            self.configuration_path(),
        )?))?)
    }

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
}

impl Zone {
    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::from("/");
        path.push(self.identifier.to_string());

        path
    }

    pub fn root_path(&self) -> PathBuf {
        self.path().join(ZONE_ROOT_PATH_NAME)
    }

    pub fn configuration_path(&self) -> PathBuf {
        self.path().join(ZONE_CONFIGURATION_PATH_NAME)
    }

    pub fn lockfile_path(&self) -> PathBuf {
        self.path().join(ZONE_LOCKFILE_PATH_NAME)
    }

    pub fn status(&self) -> Result<ZoneStatus, RetrieveZoneStatusError> {
        if self
            .jail()
            .map_err(RetrieveZoneStatusError::TryIntoJailIdError)?
            .is_some()
        {
            Ok(ZoneStatus::Running)
        } else {
            Ok(ZoneStatus::NotRunning)
        }
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
            Ok(Some(_)) => Ok(Some(Self::new(identifier.into_owned()))),
        }
    }

    pub fn create<'a, T>(
        namespace_identifier: T,
        configuration: ZoneConfiguration,
    ) -> Result<ZoneIdentifier, CreateZoneError>
    where
        T: Into<Cow<'a, NamespaceIdentifier>>,
    {
        let zone = Self::new(ZoneIdentifier::new(
            namespace_identifier.into().into_owned(),
            Uuid::new_v4(),
        ));
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
            TemplateValue::Object(zone.context_variables()),
        );

        ExecuteCreateBeforeZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute_parent(&mut context, &instruction))
            .collect::<Result<(), _>>()
            .map_err(|e| ExecuteZoneError::Parent(e))?;

        FileSystem::create(&zone.identifier().to_string())?;
        let mut file_system = FileSystem::open(&&zone.identifier().to_string())?
            .ok_or(CreateZoneError::FileSystemNotExisting)?;
        file_system.mount()?;

        create_dir(zone.root_path())?;
        let configuration_file = File::create(zone.configuration_path())?;
        to_writer(&mut BufWriter::new(configuration_file), &configuration)?;

        let mut jail = Jail::create(zone.jail_parameters())?;

        ExecuteCreateOnZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        ExecuteCreateAfterZoneExecutionInstructionIterator::new(&configuration)
            .map(|instruction| executor.execute(&mut context, &instruction, &mut jail))
            .collect::<Result<(), _>>()?;

        jail.destroy()?;

        Ok(zone.identifier)
    }
}

impl Zone {
    pub fn start(&mut self) -> Result<(), StartZoneError> {
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

    pub fn stop(&mut self) -> Result<(), StopZoneError> {
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

    pub fn destroy(self) -> Result<(), DestroyZoneError> {
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

        Ok(())
    }
}
