use serde::{Deserialize, Serialize};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JailZoneOperateCreateEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl JailZoneOperateCreateEntryConfigurationDirective {
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
pub struct JailZoneOperateDestroyEntryConfigurationDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl JailZoneOperateDestroyEntryConfigurationDirective {
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
pub struct JailZoneOperateConfigurationDirective {
    create: Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>>,
    destroy: Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>>,
}

impl JailZoneOperateConfigurationDirective {
    pub fn new(
        create: Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>>,
        destroy: Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>>,
    ) -> Self {
        Self { create, destroy }
    }

    pub fn create(&self) -> &Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>> {
        &self.create
    }

    pub fn create_mut(
        &mut self,
    ) -> &mut Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>> {
        &mut self.create
    }

    pub fn set_create(
        &mut self,
        create: Option<Vec<JailZoneOperateCreateEntryConfigurationDirective>>,
    ) {
        self.create = create
    }

    pub fn destroy(&self) -> &Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>> {
        &self.destroy
    }

    pub fn destroy_mut(
        &mut self,
    ) -> &mut Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>> {
        &mut self.destroy
    }

    pub fn set_destroy(
        &mut self,
        destroy: Option<Vec<JailZoneOperateDestroyEntryConfigurationDirective>>,
    ) {
        self.destroy = destroy
    }
}
