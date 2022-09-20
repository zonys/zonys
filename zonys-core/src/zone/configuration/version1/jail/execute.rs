use serde::{Deserialize, Serialize};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JailZoneExecuteStartEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl JailZoneExecuteStartEntryConfigurationDirective {
    pub fn new(
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
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

    pub fn arguments(&self) -> &Option<Vec<String>> {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.arguments
    }

    pub fn set_arguments(&mut self, arguments: Option<Vec<String>>) {
        self.arguments = arguments
    }

    pub fn environment_variables(&self) -> &Option<HashMap<String, String>> {
        &self.environment_variables
    }

    pub fn environment_variables_mut(&mut self) -> &mut Option<HashMap<String, String>> {
        &mut self.environment_variables
    }

    pub fn set_environment_variables(
        &mut self,
        environment_variables: Option<HashMap<String, String>>,
    ) {
        self.environment_variables = environment_variables
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JailZoneExecuteStopEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl JailZoneExecuteStopEntryConfigurationDirective {
    pub fn new(
        program: String,
        arguments: Option<Vec<String>>,
        environment_variables: Option<HashMap<String, String>>,
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

    pub fn arguments(&self) -> &Option<Vec<String>> {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.arguments
    }

    pub fn set_arguments(&mut self, arguments: Option<Vec<String>>) {
        self.arguments = arguments
    }

    pub fn environment_variables(&self) -> &Option<HashMap<String, String>> {
        &self.environment_variables
    }

    pub fn environment_variables_mut(&mut self) -> &mut Option<HashMap<String, String>> {
        &mut self.environment_variables
    }

    pub fn set_environment_variables(
        &mut self,
        environment_variables: Option<HashMap<String, String>>,
    ) {
        self.environment_variables = environment_variables
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JailZoneExecuteConfigurationDirective {
    start: Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>>,
    stop: Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>>,
}

impl JailZoneExecuteConfigurationDirective {
    pub fn new(
        start: Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>>,
        stop: Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>>,
    ) -> Self {
        Self { start, stop }
    }

    pub fn start(&self) -> &Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>> {
        &self.start
    }

    pub fn start_mut(
        &mut self,
    ) -> &mut Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>> {
        &mut self.start
    }

    pub fn set_start(
        &mut self,
        start: Option<Vec<JailZoneExecuteStartEntryConfigurationDirective>>,
    ) {
        self.start = start
    }

    pub fn stop(&self) -> &Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>> {
        &self.stop
    }

    pub fn stop_mut(&mut self) -> &mut Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>> {
        &mut self.stop
    }

    pub fn set_stop(&mut self, stop: Option<Vec<JailZoneExecuteStopEntryConfigurationDirective>>) {
        self.stop = stop
    }
}
