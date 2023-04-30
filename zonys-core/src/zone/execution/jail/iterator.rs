use crate::zone::jail::JailZoneExecuteSpecification;
use crate::zone::{ZoneConfigurationPersistence, ZoneConfigurationVersionDirective};
use std::iter::empty;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct CreateJailZoneExecutorIterator;

impl CreateJailZoneExecutorIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationPersistence,
    ) -> Box<dyn Iterator<Item = JailZoneExecuteSpecification> + 'a> {
        todo!()
        /*match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version) => {
                let jail = match version.r#type() {
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => {
                        return Box::new(empty())
                    }
                    version1::ZoneConfigurationTypeDirective::Jail(j) => j,
                };

                let operate = match jail.operate() {
                    None => return Box::new(empty()),
                    Some(o) => o,
                };

                let create = match operate.create() {
                    None => return Box::new(empty()),
                    Some(c) => c,
                };

                Box::new(create.iter().map(|e| {
                    JailZoneExecuteSpecification::new(
                        e.program().to_string(),
                        e.arguments().as_ref().cloned().unwrap_or_default(),
                        e.environment_variables()
                            .as_ref()
                            .cloned()
                            .unwrap_or_default(),
                    )
                }))
            }
        }*/
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct StartJailZoneExecutorIterator;

impl StartJailZoneExecutorIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationPersistence,
    ) -> Box<dyn Iterator<Item = JailZoneExecuteSpecification> + 'a> {
        /*match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version) => {
                let jail = match version.r#type() {
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => {
                        return Box::new(empty())
                    }
                    version1::ZoneConfigurationTypeDirective::Jail(j) => j,
                };

                let execute = match jail.execute() {
                    None => return Box::new(empty()),
                    Some(o) => o,
                };

                let start = match execute.start() {
                    None => return Box::new(empty()),
                    Some(s) => s,
                };

                Box::new(start.iter().map(|e| {
                    JailZoneExecuteSpecification::new(
                        e.program().to_string(),
                        e.arguments().as_ref().cloned().unwrap_or_default(),
                        e.environment_variables()
                            .as_ref()
                            .cloned()
                            .unwrap_or_default(),
                    )
                }))
            }
        }*/
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct StopJailZoneExecutorIterator;

impl StopJailZoneExecutorIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationPersistence,
    ) -> Box<dyn Iterator<Item = JailZoneExecuteSpecification> + 'a> {
        /*match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version) => {
                let jail = match version.r#type() {
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => {
                        return Box::new(empty())
                    }
                    version1::ZoneConfigurationTypeDirective::Jail(j) => j,
                };

                let execute = match jail.execute() {
                    None => return Box::new(empty()),
                    Some(o) => o,
                };

                let stop = match execute.stop() {
                    None => return Box::new(empty()),
                    Some(s) => s,
                };

                Box::new(stop.iter().map(|e| {
                    JailZoneExecuteSpecification::new(
                        e.program().to_string(),
                        e.arguments().as_ref().cloned().unwrap_or_default(),
                        e.environment_variables()
                            .as_ref()
                            .cloned()
                            .unwrap_or_default(),
                    )
                }))
            }
        }*/

        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DestroyJailZoneExecutorIterator;

impl DestroyJailZoneExecutorIterator {
    pub fn new<'a>(
        configuration: &'a ZoneConfigurationPersistence,
    ) -> Box<dyn Iterator<Item = JailZoneExecuteSpecification> + 'a> {
        /*match configuration.directive().version() {
            ZoneConfigurationVersionDirective::Version1(version) => {
                let jail = match version.r#type() {
                    version1::ZoneConfigurationTypeDirective::Undefined(_) => {
                        return Box::new(empty())
                    }
                    version1::ZoneConfigurationTypeDirective::Jail(j) => j,
                };

                let operate = match jail.operate() {
                    None => return Box::new(empty()),
                    Some(o) => o,
                };

                let destroy = match operate.destroy() {
                    None => return Box::new(empty()),
                    Some(d) => d,
                };

                Box::new(destroy.iter().map(|e| {
                    JailZoneExecuteSpecification::new(
                        e.program().to_string(),
                        e.arguments().as_ref().cloned().unwrap_or_default(),
                        e.environment_variables()
                            .as_ref()
                            .cloned()
                            .unwrap_or_default(),
                    )
                }))
            }
        }*/

        todo!()
    }
}
