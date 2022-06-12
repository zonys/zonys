use super::error::{ExecuteChildZoneError, ExecuteParentZoneError, ExecuteZoneError};
use crate::template::{TemplateEngine, TemplateObject};
use crate::zone::configuration::*;
use ::jail::Jail;
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
}

impl ZoneParentProgramExecution {
    pub fn new(program: String, arguments: Vec<String>) -> Self {
        Self { program, arguments }
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
}

impl<'a> From<&'a Version1BeforeCreateExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a Version1BeforeCreateExecuteJailZoneConfigurationParentEntry,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnCreateExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1OnCreateExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1AfterCreateExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1AfterCreateExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1BeforeStartExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1BeforeStartExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnStartExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1OnStartExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1AfterStartExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1AfterStartExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1BeforeStopExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1BeforeStopExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnStopExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1OnStopExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1AfterStopExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1AfterStopExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1BeforeDestroyExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a Version1BeforeDestroyExecuteJailZoneConfigurationParentEntry,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnDestroyExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(configuration: &'a Version1OnDestroyExecuteJailZoneConfigurationParentEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1AfterDestroyExecuteJailZoneConfigurationParentEntry>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a Version1AfterDestroyExecuteJailZoneConfigurationParentEntry,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
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
}

impl ZoneChildProgramExecution {
    pub fn new(program: String, arguments: Vec<String>) -> Self {
        Self { program, arguments }
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
}

impl<'a> From<&'a Version1OnCreateExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1OnCreateExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1AfterCreateExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1AfterCreateExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnStartExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1OnStartExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1AfterStartExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1AfterStartExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1BeforeStopExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1BeforeStopExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnStopExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1OnStopExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1BeforeDestroyExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a Version1BeforeDestroyExecuteJailZoneConfigurationChildEntry,
    ) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_default(),
        )
    }
}

impl<'a> From<&'a Version1OnDestroyExecuteJailZoneConfigurationChildEntry>
    for ZoneChildProgramExecution
{
    fn from(configuration: &'a Version1OnDestroyExecuteJailZoneConfigurationChildEntry) -> Self {
        Self::new(
            configuration.program().clone(),
            configuration
                .arguments()
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

impl<'a> From<&'a Version1OnCreateExecuteJailZoneConfigurationEntry> for ZoneProgramExecution {
    fn from(entry: &'a Version1OnCreateExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1OnCreateExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1OnCreateExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1AfterCreateExecuteJailZoneConfigurationEntry>
    for ZoneProgramExecution
{
    fn from(entry: &'a Version1AfterCreateExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1AfterCreateExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1AfterCreateExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1OnStartExecuteJailZoneConfigurationEntry> for ZoneProgramExecution {
    fn from(entry: &'a Version1OnStartExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1OnStartExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1OnStartExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1AfterStartExecuteJailZoneConfigurationEntry>
    for ZoneProgramExecution
{
    fn from(entry: &'a Version1AfterStartExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1AfterStartExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1AfterStartExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1BeforeStopExecuteJailZoneConfigurationEntry>
    for ZoneProgramExecution
{
    fn from(entry: &'a Version1BeforeStopExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1BeforeStopExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1BeforeStopExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1OnStopExecuteJailZoneConfigurationEntry> for ZoneProgramExecution {
    fn from(entry: &'a Version1OnStopExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1OnStopExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1OnStopExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1BeforeDestroyExecuteJailZoneConfigurationEntry>
    for ZoneProgramExecution
{
    fn from(entry: &'a Version1BeforeDestroyExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1BeforeDestroyExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1BeforeDestroyExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a Version1OnDestroyExecuteJailZoneConfigurationEntry> for ZoneProgramExecution {
    fn from(entry: &'a Version1OnDestroyExecuteJailZoneConfigurationEntry) -> Self {
        match entry {
            Version1OnDestroyExecuteJailZoneConfigurationEntry::Parent(parent) => {
                Self::Parent(parent.into())
            }
            Version1OnDestroyExecuteJailZoneConfigurationEntry::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteCreateBeforeZoneProgramExecutionIterator;

impl ExecuteCreateBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => {
                match version1.r#type() {
                    Version1ZoneConfigurationType::Jail(jail) => {
                        match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1BeforeCreateExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                    }
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteCreateOnZoneProgramExecutionIterator;

impl ExecuteCreateOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.on().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteCreateAfterZoneProgramExecutionIterator;

impl ExecuteCreateAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStartBeforeZoneProgramExecutionIterator;

impl ExecuteStartBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => {
                match version1.r#type() {
                    Version1ZoneConfigurationType::Jail(jail) => {
                        match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.start().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1BeforeStartExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                    }
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStartOnZoneProgramExecutionIterator;

impl ExecuteStartOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
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
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStartAfterZoneProgramExecutionIterator;

impl ExecuteStartAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
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
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStopBeforeZoneProgramExecutionIterator;

impl ExecuteStopBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
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
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStopOnZoneProgramExecutionIterator;

impl ExecuteStopOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
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
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteStopAfterZoneProgramExecutionIterator;

impl ExecuteStopAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.stop().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| {
                            b.inner().iter().map(|e| match e {
                                Version1AfterStopExecuteJailZoneConfigurationEntry::Parent(
                                    parent,
                                ) => ZoneParentProgramExecution::from(parent),
                            })
                        }) {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteDestroyBeforeZoneProgramExecutionIterator;

impl ExecuteDestroyBeforeZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteDestroyOnZoneProgramExecutionIterator;

impl ExecuteDestroyOnZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.on().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(ZoneProgramExecution::from))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(std::iter::empty()),
                    }
                }
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ExecuteDestroyAfterZoneProgramExecutionIterator;

impl ExecuteDestroyAfterZoneProgramExecutionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentProgramExecution> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => {
                match version1.r#type() {
                    Version1ZoneConfigurationType::Jail(jail) => {
                        match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1AfterDestroyExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                    }
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
        jail.execute(
            &self
                .template_engine
                .render(context.variables(), instruction.program())?,
            &instruction
                .arguments()
                .iter()
                .map(|a| self.template_engine.render(context.variables(), a))
                .collect::<Result<Vec<_>, _>>()?,
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
