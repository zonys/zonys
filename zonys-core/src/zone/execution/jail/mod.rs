mod error;
mod iterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use error::*;
pub use iterator::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::template::{RenderTemplateError, TemplateEngine, TemplateObject};
use crate::zone::ZoneIdentifier;
use crate::zone::{
    CreateZoneExecutorEvent, CreateZoneExecutorEventError, DestroyZoneExecutorEvent,
    DestroyZoneExecutorEventError, RunningZoneExecutorEvent, RunningZoneExecutorEventError,
    StartZoneExecutorEvent, StartZoneExecutorEventError, StopZoneExecutorEvent,
    StopZoneExecutorEventError, ZoneExecutor,
};
use ::jail::{ExecuteJailError, Jail, JailId, JailName, JailParameter, TryIntoJailIdError};
use std::collections::HashMap;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct JailZoneExecuteSpecification {
    program: String,
    arguments: Vec<String>,
    environment_variables: HashMap<String, String>,
}

impl JailZoneExecuteSpecification {
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
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct JailZoneExecutor {}

impl ZoneExecutor for JailZoneExecutor {
    /*fn running(
        &self,
        mut event: RunningZoneExecutorEvent,
    ) -> Result<RunningZoneExecutorEvent, RunningZoneExecutorEventError> {
        event.set_running(
            Option::<JailId>::try_from(JailName::new(event.identifier().to_string()))
                .map_err(RunningJailZoneExecutorEventError::TryIntoJailIdError)?
                .is_some(),
        );

        Ok(event)
    }

    fn create(
        &mut self,
        event: CreateZoneExecutorEvent,
    ) -> Result<CreateZoneExecutorEvent, CreateZoneExecutorEventError> {
        let jail = Jail::create(self.jail_parameters(event.identifier(), event.root_path()))
            .map_err(CreateJailZoneExecutorEventError::CreateJail)?;

        for specification in CreateJailZoneExecutorIterator::new(event.configuration()) {
            let specification = match Self::prepare_specification(
                event.template_engine(),
                event.variables(),
                specification,
            )
            .map_err(CreateJailZoneExecutorEventError::RenderTemplate)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(CreateJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(s) => s,
            };

            match Self::execute(&jail, specification)
                .map_err(CreateJailZoneExecutorEventError::ExecuteJail)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(CreateJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(()) => {}
            }
        }

        jail.destroy()
            .map_err(CreateJailZoneExecutorEventError::DestroyJail)?;

        Ok(event)
    }

    fn start(
        &mut self,
        event: StartZoneExecutorEvent,
    ) -> Result<StartZoneExecutorEvent, StartZoneExecutorEventError> {
        match self
            .jail(event.identifier())
            .map_err(StartJailZoneExecutorEventError::GetJailId)?
        {
            Some(_) => {
                return Err(StartZoneExecutorEventError::from(
                    StartJailZoneExecutorEventError::JailIsRunning,
                ))
            }
            None => {}
        };

        let jail = Jail::create(self.jail_parameters(event.identifier(), event.root_path()))
            .map_err(StartJailZoneExecutorEventError::CreateJail)?;

        for specification in StartJailZoneExecutorIterator::new(event.configuration()) {
            let specification = match Self::prepare_specification(
                event.template_engine(),
                event.variables(),
                specification,
            )
            .map_err(StartJailZoneExecutorEventError::RenderTemplate)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(StartJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(s) => s,
            };

            match Self::execute(&jail, specification)
                .map_err(StartJailZoneExecutorEventError::ExecuteJail)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(StartJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(()) => {}
            };
        }

        Ok(event)
    }

    fn stop(
        &mut self,
        event: StopZoneExecutorEvent,
    ) -> Result<StopZoneExecutorEvent, StopZoneExecutorEventError> {
        let jail = match self
            .jail(event.identifier())
            .map_err(StopJailZoneExecutorEventError::GetJailId)?
        {
            None => {
                return Err(StopZoneExecutorEventError::from(
                    StopJailZoneExecutorEventError::JailIsNotRunning,
                ))
            }
            Some(jail) => jail,
        };

        for specification in StartJailZoneExecutorIterator::new(event.configuration()) {
            let specification = match Self::prepare_specification(
                event.template_engine(),
                event.variables(),
                specification,
            )
            .map_err(StopJailZoneExecutorEventError::RenderTemplate)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(StopJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(s) => s,
            };

            match Self::execute(&jail, specification)
                .map_err(StopJailZoneExecutorEventError::ExecuteJail)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(StopJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(()) => {}
            };
        }

        jail.destroy()
            .map_err(StopJailZoneExecutorEventError::DestroyJail)?;

        Ok(event)
    }

    fn destroy(
        &mut self,
        event: DestroyZoneExecutorEvent,
    ) -> Result<DestroyZoneExecutorEvent, DestroyZoneExecutorEventError> {
        let jail = Jail::create(self.jail_parameters(event.identifier(), event.root_path()))
            .map_err(DestroyJailZoneExecutorEventError::CreateJail)?;

        for specification in DestroyJailZoneExecutorIterator::new(event.configuration()) {
            let specification = match Self::prepare_specification(
                event.template_engine(),
                event.variables(),
                specification,
            )
            .map_err(DestroyJailZoneExecutorEventError::RenderTemplate)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(DestroyJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(s) => s,
            };

            match Self::execute(&jail, specification)
                .map_err(DestroyJailZoneExecutorEventError::ExecuteJail)
            {
                Err(e) => {
                    jail.destroy()
                        .map_err(DestroyJailZoneExecutorEventError::DestroyJail)?;

                    return Err(e.into());
                }
                Ok(()) => {}
            };
        }

        jail.destroy()
            .map_err(DestroyJailZoneExecutorEventError::DestroyJail)?;

        Ok(event)
    }*/
}

impl JailZoneExecutor {
    fn execute(
        jail: &Jail,
        specification: JailZoneExecuteSpecification,
    ) -> Result<(), ExecuteJailError> {
        jail.execute(
            specification.program(),
            specification.arguments(),
            specification.environment_variables(),
        )
    }

    fn prepare_specification(
        template_engine: &TemplateEngine,
        variables: &TemplateObject,
        mut specification: JailZoneExecuteSpecification,
    ) -> Result<JailZoneExecuteSpecification, RenderTemplateError> {
        specification.set_program(template_engine.render(variables, specification.program())?);

        for argument in specification.arguments_mut().iter_mut() {
            *argument = template_engine.render(variables, argument)?;
        }

        for key in specification
            .environment_variables()
            .keys()
            .cloned()
            .collect::<Vec<String>>()
        {
            match specification.environment_variables_mut().remove(&key) {
                Some(value) => {
                    specification.environment_variables_mut().insert(
                        template_engine.render(variables, &key)?,
                        template_engine.render(variables, &value)?,
                    );
                }
                None => {}
            };
        }

        Ok(specification)
    }

    fn jail_name(&self, identifier: &ZoneIdentifier) -> String {
        identifier.uuid().to_string()
    }

    fn jail_parameters<T>(&self, identifier: &ZoneIdentifier, root_path: T) -> Vec<JailParameter>
    where
        T: AsRef<Path>,
    {
        vec![
            JailParameter::new("persist", "true"),
            JailParameter::new("name", self.jail_name(identifier)),
            JailParameter::new("path", root_path.as_ref().display().to_string()),
        ]
    }

    fn jail(&self, identifier: &ZoneIdentifier) -> Result<Option<Jail>, TryIntoJailIdError> {
        Ok(
            Option::<JailId>::try_from(JailName::new(self.jail_name(identifier)))?
                .map(Jail::open)
                .flatten(),
        )
    }
}

impl JailZoneExecutor {
    pub fn new() -> Self {
        Self {}
    }
}
