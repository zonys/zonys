use super::error::{ExecuteChildZoneError, ExecuteParentZoneError, ExecuteZoneError};
use crate::template::{TemplateEngine, TemplateObject};
use crate::zone::configuration::*;
use ::jail::Jail;
use std::collections::HashMap;
use std::iter::empty;
use std::process::Command;
use std::process::Stdio;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default)]
pub struct ZoneExecutionContext {
    variables: TemplateObject,
}

impl ZoneExecutionContext {
    pub fn new(variables: TemplateObject) -> Self {
        Self { variables }
    }

    pub fn variables(&self) -> &TemplateObject {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> &mut TemplateObject {
        &mut self.variables
    }

    pub fn set_variables(&mut self, variables: TemplateObject) {
        self.variables = variables
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ZoneParentProgramExecution {
    program: String,
    arguments: Vec<String>,
    environment_variables: HashMap<String, String>,
}

impl ZoneParentProgramExecution {
    pub fn new(
        program: String,
        arguments: Vec<String>,
        environment_variables: HashMap<String, String>,
    ) -> Self {
        Self {
            program,
            arguments,
            environment_variables,
        }
    }

    pub fn program(&self) -> &String {
        &self.program
    }

    pub fn program_mut(&mut self) -> &mut String {
        &mut self.program
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program
    }

    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<String> {
        &mut self.arguments
    }

    pub fn set_arguments(&mut self, arguments: Vec<String>) {
        self.arguments = arguments
    }

    pub fn environment_variables(&self) -> &HashMap<String, String> {
        &self.environment_variables
    }

    pub fn environment_variables_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.environment_variables
    }

    pub fn set_environment_variables(&mut self, environment_variables: HashMap<String, String>) {
        self.environment_variables = environment_variables
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateBeforeParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateBeforeParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateOnParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailOperateCreateOnParentEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateAfterParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateAfterParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartBeforeParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartBeforeParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartOnParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStartOnParentEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartAfterParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartAfterParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopBeforeParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStopBeforeParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopOnParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStopOnParentEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopAfterParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStopAfterParentEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyBeforeParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyBeforeParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyOnParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailOperateDestroyOnParentEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyAfterParentEntryConfiguration>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyAfterParentEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ZoneChildProgramExecution {
    program: String,
    arguments: Vec<String>,
    environment_variables: HashMap<String, String>,
}

impl ZoneChildProgramExecution {
    pub fn new(
        program: String,
        arguments: Vec<String>,
        environment_variables: HashMap<String, String>,
    ) -> Self {
        Self {
            program,
            arguments,
            environment_variables,
        }
    }

    pub fn program(&self) -> &String {
        &self.program
    }

    pub fn program_mut(&mut self) -> &mut String {
        &mut self.program
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program
    }

    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<String> {
        &mut self.arguments
    }

    pub fn set_arguments(&mut self, arguments: Vec<String>) {
        self.arguments = arguments
    }

    pub fn environment_variables(&self) -> &HashMap<String, String> {
        &self.environment_variables
    }

    pub fn environment_variables_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.environment_variables
    }

    pub fn set_environment_variables(&mut self, environment_variables: HashMap<String, String>) {
        self.environment_variables = environment_variables
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateOnChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailOperateCreateOnChildEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateAfterChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateAfterChildEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartOnChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStartOnChildEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartAfterChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStartAfterChildEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopBeforeChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStopBeforeChildEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopOnChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailExecuteStopOnChildEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyBeforeChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyBeforeChildEntryConfiguration,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyOnChildEntryConfiguration>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a version1::ZoneJailOperateDestroyOnChildEntryConfiguration) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
            configuration
                .environment_variables()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum ZoneProgramExecution {
    Parent(ZoneParentProgramExecution),
    Child(ZoneChildProgramExecution),
}

impl<'a> From<&'a version1::ZoneJailOperateCreateBeforeEntryConfiguration>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateCreateBeforeEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailOperateCreateBeforeEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateOnEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailOperateCreateOnEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailOperateCreateOnEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateCreateOnEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateAfterEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailOperateCreateAfterEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailOperateCreateAfterEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateCreateAfterEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartOnEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailExecuteStartOnEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailExecuteStartOnEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStartOnEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartAfterEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailExecuteStartAfterEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailExecuteStartAfterEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStartAfterEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopBeforeEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailExecuteStopBeforeEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailExecuteStopBeforeEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStopBeforeEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopOnEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailExecuteStopOnEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailExecuteStopOnEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStopOnEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyBeforeEntryConfiguration>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateDestroyBeforeEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailOperateDestroyBeforeEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateDestroyBeforeEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyOnEntryConfiguration> for ZoneProgramExecution {
    fn from(entry: &'a version1::ZoneJailOperateDestroyOnEntryConfiguration) -> Self {
        match entry {
            version1::ZoneJailOperateDestroyOnEntryConfiguration::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateDestroyOnEntryConfiguration::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OperateCreateBeforeZoneProgramExecutionIterator;

impl OperateCreateBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                match version1.r#type() {
                    version1::ZoneConfigurationType::Jail(jail) => {
                        match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            version1::ZoneJailOperateCreateBeforeEntryConfiguration::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        })) {
                            Some(iter) => Box::new(iter),
                            None => Box::new(empty()),
                        }
                    }
                    version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OperateCreateOnZoneProgramExecutionIterator;

impl OperateCreateOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.on().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OperateCreateAfterZoneProgramExecutionIterator;

impl OperateCreateAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStartBeforeZoneProgramExecutionIterator;

impl ExecuteStartBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                match version1.r#type() {
                    version1::ZoneConfigurationType::Jail(jail) => {
                        match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.start().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            version1::ZoneJailExecuteStartBeforeEntryConfiguration::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                    }
                    version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStartOnZoneProgramExecutionIterator;

impl ExecuteStartOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.start().as_ref())
                        .flatten()
                        .map(|c| c.on().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStartAfterZoneProgramExecutionIterator;

impl ExecuteStartAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.start().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStopBeforeZoneProgramExecutionIterator;

impl ExecuteStopBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.stop().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStopOnZoneProgramExecutionIterator;

impl ExecuteStopOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.stop().as_ref())
                        .flatten()
                        .map(|c| c.on().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStopAfterZoneProgramExecutionIterator;

impl ExecuteStopAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.stop().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| {
                            b.inner().iter().map(|e| match e {
                                version1::ZoneJailExecuteStopAfterEntryConfiguration::Parent(
                                    parent,
                                ) => ZoneParentProgramExecution::from(parent),
                            })
                        }) {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OperateDestroyBeforeZoneProgramExecutionIterator;

impl OperateDestroyBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OperateDestroyOnZoneProgramExecutionIterator;

impl OperateDestroyOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => match version1.r#type() {
                version1::ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.on().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OperateDestroyAfterZoneProgramExecutionIterator;

impl OperateDestroyAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationDirective,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration.version() {
            ZoneConfigurationVersionDirective::Version1(version1) => {
                match version1.r#type() {
                    version1::ZoneConfigurationType::Jail(jail) => {
                        match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            version1::ZoneJailOperateDestroyAfterEntryConfiguration::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                    }
                    version1::ZoneConfigurationType::Undefined(_) => Box::new(empty()),
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ZoneExecutor {
    template_engine: TemplateEngine,
}

impl ZoneExecutor {
    pub fn execute_parent(
        &self,
        context: &mut ZoneExecutionContext,
        instruction: &ZoneParentProgramExecution,
    ) -> Result<(), ExecuteParentZoneError> {
        let mut environment_variables = HashMap::<String, String>::new();

        for (key, value) in instruction.environment_variables() {
            environment_variables.insert(
                self.template_engine.render(context.variables(), key)?,
                self.template_engine.render(context.variables(), value)?,
            );
        }

        Command::new(
            self.template_engine
                .render(context.variables(), instruction.program())?,
        )
        .args(
            instruction
                .arguments()
                .iter()
                .map(|a| self.template_engine.render(context.variables(), a))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .env_clear()
        .envs(environment_variables)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait()?
        .exit_ok()?;

        Ok(())
    }

    pub fn execute_child(
        &self,
        context: &mut ZoneExecutionContext,
        instruction: &ZoneChildProgramExecution,
        jail: &mut Jail,
    ) -> Result<(), ExecuteChildZoneError> {
        let mut environment_variables = HashMap::<String, String>::new();

        for (key, value) in instruction.environment_variables() {
            environment_variables.insert(
                self.template_engine.render(context.variables(), key)?,
                self.template_engine.render(context.variables(), value)?,
            );
        }

        jail.execute(
            &self
                .template_engine
                .render(context.variables(), instruction.program())?,
            &instruction
                .arguments()
                .iter()
                .map(|a| self.template_engine.render(context.variables(), a))
                .collect::<Result<Vec<_>, _>>()?,
            environment_variables,
        )?;

        Ok(())
    }
}

impl ZoneExecutor {
    pub fn execute(
        &self,
        context: &mut ZoneExecutionContext,
        instruction: &ZoneProgramExecution,
        jail: &mut Jail,
    ) -> Result<(), ExecuteZoneError> {
        match instruction {
            ZoneProgramExecution::Parent(parent_instruction) => {
                self.execute_parent(context, &parent_instruction)?
            }
            ZoneProgramExecution::Child(child_instruction) => {
                self.execute_child(context, &child_instruction, jail)?
            }
        }

        Ok(())
    }
}
