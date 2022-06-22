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

impl<'a> From<&'a version1::ZoneJailOperateCreateBeforeParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateBeforeParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateCreateOnParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateOnParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateCreateAfterParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateAfterParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStartBeforeParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartBeforeParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStartOnParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartOnParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStartAfterParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartAfterParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStopBeforeParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStopBeforeParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStopOnParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStopOnParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStopAfterParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStopAfterParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateDestroyBeforeParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyBeforeParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateDestroyOnParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyOnParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateDestroyAfterParentEntryConfigurationDirective>
    for ZoneParentProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyAfterParentEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateCreateOnChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateOnChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateCreateAfterChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateCreateAfterChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStartOnChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartOnChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStartAfterChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStartAfterChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStopBeforeChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStopBeforeChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailExecuteStopOnChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailExecuteStopOnChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateDestroyBeforeChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyBeforeChildEntryConfigurationDirective,
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

impl<'a> From<&'a version1::ZoneJailOperateDestroyOnChildEntryConfigurationDirective>
    for ZoneChildProgramExecution
{
    fn from(
        configuration: &'a version1::ZoneJailOperateDestroyOnChildEntryConfigurationDirective,
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
pub enum ZoneProgramExecution {
    Parent(ZoneParentProgramExecution),
    Child(ZoneChildProgramExecution),
}

impl<'a> From<&'a version1::ZoneJailOperateCreateBeforeEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateCreateBeforeEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailOperateCreateBeforeEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateOnEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateCreateOnEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailOperateCreateOnEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateCreateOnEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateCreateAfterEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateCreateAfterEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailOperateCreateAfterEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateCreateAfterEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartOnEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailExecuteStartOnEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailExecuteStartOnEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStartOnEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStartAfterEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailExecuteStartAfterEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailExecuteStartAfterEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStartAfterEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopBeforeEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailExecuteStopBeforeEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailExecuteStopBeforeEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStopBeforeEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailExecuteStopOnEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailExecuteStopOnEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailExecuteStopOnEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailExecuteStopOnEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyBeforeEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateDestroyBeforeEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailOperateDestroyBeforeEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateDestroyBeforeEntryConfigurationDirective::Child(child) => {
                Self::Child(child.into())
            }
        }
    }
}

impl<'a> From<&'a version1::ZoneJailOperateDestroyOnEntryConfigurationDirective>
    for ZoneProgramExecution
{
    fn from(entry: &'a version1::ZoneJailOperateDestroyOnEntryConfigurationDirective) -> Self {
        match entry {
            version1::ZoneJailOperateDestroyOnEntryConfigurationDirective::Parent(parent) => {
                Self::Parent(parent.into())
            }
            version1::ZoneJailOperateDestroyOnEntryConfigurationDirective::Child(child) => {
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
                    version1::ZoneConfigurationTypeDirective::Jail(jail) => {
                        match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.create().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            version1::ZoneJailOperateCreateBeforeEntryConfigurationDirective::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        })) {
                            Some(iter) => Box::new(iter),
                            None => Box::new(empty()),
                        }
                    }
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                    version1::ZoneConfigurationTypeDirective::Jail(jail) => {
                        match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.start().as_ref())
                        .flatten()
                        .map(|c| c.before().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            version1::ZoneJailExecuteStartBeforeEntryConfigurationDirective::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                    }
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
                    match jail
                        .execute()
                        .as_ref()
                        .map(|e| e.stop().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| {
                            b.inner().iter().map(|e| match e {
                                version1::ZoneJailExecuteStopAfterEntryConfigurationDirective::Parent(
                                    parent,
                                ) => ZoneParentProgramExecution::from(parent),
                            })
                        }) {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                }
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                version1::ZoneConfigurationTypeDirective::Jail(jail) => {
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
                version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
                    version1::ZoneConfigurationTypeDirective::Jail(jail) => {
                        match jail
                        .operate()
                        .as_ref()
                        .map(|e| e.destroy().as_ref())
                        .flatten()
                        .map(|c| c.after().as_ref())
                        .flatten()
                        .map(|b| b.inner().iter().map(|e| match e {
                            version1::ZoneJailOperateDestroyAfterEntryConfigurationDirective::Parent(parent) => ZoneParentProgramExecution::from(parent),
                        }))
                    {
                        Some(iter) => Box::new(iter),
                        None => Box::new(empty()),
                    }
                    }
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => Box::new(empty()),
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
