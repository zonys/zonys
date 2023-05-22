use crate::{
    ZoneConfigurationReaderTraverser, ZoneConfigurationUnit,
    ZoneConfigurationVersion1JailProgramUnit, ZoneConfigurationVersion1TypeUnit,
    ZoneConfigurationVersion1VolumeUnit, ZoneConfigurationVersionUnit, ZoneVolumeType,
};
use std::collections::HashMap;
use std::iter::empty;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug, Method)]
#[Method(accessors)]
#[Constructor(visibility = pub(self))]
pub struct JailZoneConfigurationStep<'a> {
    program: &'a String,
    arguments: &'a Option<Vec<String>>,
    environment_variables: &'a Option<HashMap<String, String>>,
}

impl<'a> From<&'a ZoneConfigurationVersion1JailProgramUnit> for JailZoneConfigurationStep<'a> {
    fn from(unit: &'a ZoneConfigurationVersion1JailProgramUnit) -> Self {
        Self::new(
            unit.program(),
            unit.arguments(),
            unit.environment_variables(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(crate))]
pub struct JailZoneConfigurationReader<'a> {
    unit: &'a ZoneConfigurationUnit,
}

impl<'a> JailZoneConfigurationReader<'a> {
    pub fn volume(&self) -> ZoneVolumeType {
        for unit in ZoneConfigurationReaderTraverser::new(vec![self.unit]).inorder() {
            match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    if let Some(volume) = jail.volume() {
                        return match volume {
                            ZoneConfigurationVersion1VolumeUnit::Automatic => {
                                ZoneVolumeType::Automatic
                            }
                            ZoneConfigurationVersion1VolumeUnit::Directory => {
                                ZoneVolumeType::Directory
                            }
                            ZoneConfigurationVersion1VolumeUnit::Zfs => ZoneVolumeType::Zfs,
                        };
                    }
                }
            }
        }

        ZoneVolumeType::Automatic
    }

    pub fn from(&self) -> Option<&String> {
        for unit in ZoneConfigurationReaderTraverser::new(vec![self.unit]).inorder() {
            match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    if jail.from().is_some() {
                        return jail.from().as_ref();
                    }
                }
            }
        }

        None
    }

    pub fn create_steps(&self) -> impl Iterator<Item = JailZoneConfigurationStep<'a>> {
        ZoneConfigurationReaderTraverser::new(vec![self.unit])
            .inorder()
            .flat_map(|unit| match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    let execute = match jail.execute() {
                        Some(execute) => execute,
                        None => return None,
                    };

                    let create = match execute.create() {
                        Some(create) => create,
                        None => return None,
                    };

                    let on = match create.on() {
                        Some(on) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(on.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    let after = match create.after() {
                        Some(after) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(after.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    Some(on.chain(after))
                }
            })
            .flatten()
    }

    pub fn start_steps(&self) -> impl Iterator<Item = JailZoneConfigurationStep<'a>> {
        ZoneConfigurationReaderTraverser::new(vec![self.unit])
            .inorder()
            .flat_map(|unit| match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    let execute = match jail.execute() {
                        Some(execute) => execute,
                        None => return None,
                    };

                    let start = match execute.start() {
                        Some(start) => start,
                        None => return None,
                    };

                    let before = match start.before() {
                        Some(before) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(before.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    let on = match start.on() {
                        Some(on) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(on.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    let after = match start.after() {
                        Some(after) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(after.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    Some(before.chain(on).chain(after))
                }
            })
            .flatten()
    }

    pub fn stop_steps(&self) -> impl Iterator<Item = JailZoneConfigurationStep<'a>> {
        ZoneConfigurationReaderTraverser::new(vec![self.unit])
            .inorder()
            .flat_map(|unit| match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    let execute = match jail.execute() {
                        Some(execute) => execute,
                        None => return None,
                    };

                    let stop = match execute.stop() {
                        Some(stop) => stop,
                        None => return None,
                    };

                    let before = match stop.before() {
                        Some(before) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(before.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    let on = match stop.on() {
                        Some(on) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(on.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    let after = match stop.after() {
                        Some(after) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(after.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    Some(before.chain(on).chain(after))
                }
            })
            .flatten()
    }

    pub fn destroy_steps(&self) -> impl Iterator<Item = JailZoneConfigurationStep<'a>> {
        ZoneConfigurationReaderTraverser::new(vec![self.unit])
            .inorder()
            .flat_map(|unit| match unit.version() {
                ZoneConfigurationVersionUnit::Version1(version1) => {
                    let jail = match version1.r#type() {
                        ZoneConfigurationVersion1TypeUnit::Jail(jail) => jail,
                    };

                    let execute = match jail.execute() {
                        Some(execute) => execute,
                        None => return None,
                    };

                    let destroy = match execute.destroy() {
                        Some(destroy) => destroy,
                        None => return None,
                    };

                    let before = match destroy.before() {
                        Some(before) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(before.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    let on = match destroy.on() {
                        Some(on) => {
                            Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                                Box::new(on.iter().map(JailZoneConfigurationStep::from)),
                            )
                        }
                        None => Box::<dyn Iterator<Item = JailZoneConfigurationStep<'a>>>::from(
                            Box::new(empty()),
                        ),
                    };

                    Some(before.chain(on))
                }
            })
            .flatten()
    }
}
