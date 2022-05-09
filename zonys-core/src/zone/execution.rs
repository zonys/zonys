use super::error::{ExecuteChildZoneError, ExecuteParentZoneError, ExecuteZoneError};
use crate::template::{TemplateEngine, TemplateObject, TemplateValue};
use crate::zone::configuration::*;
use std::iter::empty;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use ::jail::{Jail};

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
pub struct ZoneParentExecutionInstruction {
    program: String,
    arguments: Vec<String>,
}

impl ZoneParentExecutionInstruction {
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
    for ZoneParentExecutionInstruction
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
pub struct ZoneChildExecutionInstruction {
    program: String,
    arguments: Vec<String>,
}

impl ZoneChildExecutionInstruction {
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
    for ZoneChildExecutionInstruction
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
pub enum ZoneExecutionInstruction {
    Parent(ZoneParentExecutionInstruction),
    Child(ZoneChildExecutionInstruction),
}

impl<'a> From<&'a Version1OnCreateExecuteJailZoneConfigurationEntry> for ZoneExecutionInstruction {
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
    for ZoneExecutionInstruction
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

impl<'a> From<&'a Version1OnStartExecuteJailZoneConfigurationEntry> for ZoneExecutionInstruction {
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
    for ZoneExecutionInstruction
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
    for ZoneExecutionInstruction
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

impl<'a> From<&'a Version1OnStopExecuteJailZoneConfigurationEntry> for ZoneExecutionInstruction {
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
    for ZoneExecutionInstruction
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

impl<'a> From<&'a Version1OnDestroyExecuteJailZoneConfigurationEntry> for ZoneExecutionInstruction {
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

pub struct ExecuteCreateBeforeZoneExecutionInstructionIterator;

impl ExecuteCreateBeforeZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentExecutionInstruction> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1BeforeCreateExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentExecutionInstruction::from(parent),
                        }))
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

pub struct ExecuteCreateOnZoneExecutionInstructionIterator;

impl ExecuteCreateOnZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteCreateAfterZoneExecutionInstructionIterator;

impl ExecuteCreateAfterZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteStartBeforeZoneExecutionInstructionIterator;

impl ExecuteStartBeforeZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentExecutionInstruction> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.start().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1BeforeStartExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentExecutionInstruction::from(parent),
                        }))
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

pub struct ExecuteStartOnZoneExecutionInstructionIterator;

impl ExecuteStartOnZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteStartAfterZoneExecutionInstructionIterator;

impl ExecuteStartAfterZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteStopBeforeZoneExecutionInstructionIterator;

impl ExecuteStopBeforeZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteStopOnZoneExecutionInstructionIterator;

impl ExecuteStopOnZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteStopAfterZoneExecutionInstructionIterator;

impl ExecuteStopAfterZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1AfterStopExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentExecutionInstruction::from(parent),
                        }))
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

pub struct ExecuteDestroyBeforeZoneExecutionInstructionIterator;

impl ExecuteDestroyBeforeZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteDestroyOnZoneExecutionInstructionIterator;

impl ExecuteDestroyOnZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneExecutionInstruction> + 'a> {
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
                        .map(|b| b.inner().iter().map(ZoneExecutionInstruction::from))
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

pub struct ExecuteDestroyAfterZoneExecutionInstructionIterator;

impl ExecuteDestroyAfterZoneExecutionInstructionIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfiguration,
    ) -> Box<dyn Iterator<Item = ZoneParentExecutionInstruction> + 'a> {
        match configuration {
            ZoneConfiguration::Version1(version1) => match version1.r#type() {
                Version1ZoneConfigurationType::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            Version1AfterDestroyExecuteJailZoneConfigurationEntry::Parent(parent) => ZoneParentExecutionInstruction::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
            },
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
        instruction: &ZoneParentExecutionInstruction,
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
        instruction: &ZoneChildExecutionInstruction,
        jail: &mut Jail,
    ) -> Result<(), ExecuteChildZoneError> {
        todo!()
    }
}

impl ZoneExecutor {
    pub fn execute(
        &self,
        context: &mut ZoneExecutionContext,
        instruction: &ZoneExecutionInstruction,
        jail: &mut Jail,
    ) -> Result<(), ExecuteZoneError> {
        match instruction {
            ZoneExecutionInstruction::Parent(parent_instruction) => {
                self.execute_parent(context, &parent_instruction)?
            }
            ZoneExecutionInstruction::Child(child_instruction) => {
                self.execute_child(context, &child_instruction, jail)?
            }
        }

        Ok(())
    }
}
