use crate::template::{TemplateEngine, TemplateObject};
use crate::zone::{ZoneConfiguration, ZoneIdentifier};
use std::path::PathBuf;
use ztd::Record;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct RunningZoneExecutorEvent {
    identifier: ZoneIdentifier,
    running: bool,
}

impl RunningZoneExecutorEvent {
    pub fn new(identifier: ZoneIdentifier, running: bool) -> Self {
        Self {
            identifier,
            running,
        }
    }

    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn running_mut(&mut self) -> &mut bool {
        &mut self.running
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct CreateZoneExecutorEvent {
    identifier: ZoneIdentifier,
    configuration: ZoneConfiguration,
    root_path: PathBuf,
    template_engine: TemplateEngine,
    variables: TemplateObject,
}

impl CreateZoneExecutorEvent {
    pub fn new(
        identifier: ZoneIdentifier,
        configuration: ZoneConfiguration,
        root_path: PathBuf,
        template_engine: TemplateEngine,
        variables: TemplateObject,
    ) -> Self {
        Self {
            identifier,
            configuration,
            root_path,
            template_engine,
            variables,
        }
    }

    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn configuration(&self) -> &ZoneConfiguration {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut ZoneConfiguration {
        &mut self.configuration
    }

    pub fn set_configuration(&mut self, configuration: ZoneConfiguration) {
        self.configuration = configuration
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
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

pub struct StartZoneExecutorEvent {
    identifier: ZoneIdentifier,
    configuration: ZoneConfiguration,
    root_path: PathBuf,
    template_engine: TemplateEngine,
    variables: TemplateObject,
}

impl StartZoneExecutorEvent {
    pub fn new(
        identifier: ZoneIdentifier,
        configuration: ZoneConfiguration,
        root_path: PathBuf,
        template_engine: TemplateEngine,
        variables: TemplateObject,
    ) -> Self {
        Self {
            identifier,
            configuration,
            root_path,
            template_engine,
            variables,
        }
    }

    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn configuration(&self) -> &ZoneConfiguration {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut ZoneConfiguration {
        &mut self.configuration
    }

    pub fn set_configuration(&mut self, configuration: ZoneConfiguration) {
        self.configuration = configuration
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
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

pub struct StopZoneExecutorEvent {
    identifier: ZoneIdentifier,
    configuration: ZoneConfiguration,
    root_path: PathBuf,
    template_engine: TemplateEngine,
    variables: TemplateObject,
}

impl StopZoneExecutorEvent {
    pub fn new(
        identifier: ZoneIdentifier,
        configuration: ZoneConfiguration,
        root_path: PathBuf,
        template_engine: TemplateEngine,
        variables: TemplateObject,
    ) -> Self {
        Self {
            identifier,
            configuration,
            root_path,
            template_engine,
            variables,
        }
    }

    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn configuration(&self) -> &ZoneConfiguration {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut ZoneConfiguration {
        &mut self.configuration
    }

    pub fn set_configuration(&mut self, configuration: ZoneConfiguration) {
        self.configuration = configuration
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
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

#[derive(Record)]
pub struct DestroyZoneExecutorEvent {
    identifier: ZoneIdentifier,
    configuration: ZoneConfiguration,
    root_path: PathBuf,
    template_engine: TemplateEngine,
    variables: TemplateObject,
}

impl DestroyZoneExecutorEvent {
    pub fn new(
        identifier: ZoneIdentifier,
        configuration: ZoneConfiguration,
        root_path: PathBuf,
        template_engine: TemplateEngine,
        variables: TemplateObject,
    ) -> Self {
        Self {
            identifier,
            configuration,
            root_path,
            template_engine,
            variables,
        }
    }

    pub fn identifier(&self) -> &ZoneIdentifier {
        &self.identifier
    }

    pub fn configuration(&self) -> &ZoneConfiguration {
        &self.configuration
    }

    pub fn configuration_mut(&mut self) -> &mut ZoneConfiguration {
        &mut self.configuration
    }

    pub fn set_configuration(&mut self, configuration: ZoneConfiguration) {
        self.configuration = configuration
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
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
