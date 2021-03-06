#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand};
use std::env::current_dir;
use std::error;
use std::fmt::Debug;
use std::io::{stdin as io_stdin, stdout, ErrorKind};
use zonys_core::namespace::{Namespace, NamespaceIdentifier};
use zonys_core::zone::{
    ReceiveZoneError, ZoneConfiguration, ZoneConfigurationDirective,
    ZoneConfigurationVersionDirective,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Parser)]
#[clap(name = "zonys")]
#[clap(about = "Another execution environment manager for the FreeBSD operating system.")]
#[clap(author, version, long_about = None)]
struct MainArguments {
    #[clap(default_value = "zroot/zonys")]
    namespace_identifier: NamespaceIdentifier,

    #[clap(subcommand)]
    command: MainCommand,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Subcommand)]
enum MainCommand {
    Show {
        regular_expression: String,
    },
    Create {
        #[clap(short, long)]
        include: Option<Vec<String>>,
    },
    Destroy {
        regular_expression: String,
    },
    Recreate {
        regular_expression: String,
    },
    Start {
        regular_expression: String,
    },
    Stop {
        regular_expression: String,
    },
    Restart {
        regular_expression: String,
    },
    Up {
        regular_expression: String,
    },
    Down {
        regular_expression: String,
    },
    Reup {
        regular_expression: String,
    },
    Deploy {
        #[clap(short, long)]
        include: Option<Vec<String>>,
    },
    Undeploy {
        regular_expression: String,
    },
    Redeploy {
        regular_expression: String,
    },
    Send {
        regular_expression: String,
    },
    Receive,
    Run {
        #[clap(short, long)]
        include: Option<Vec<String>>,
    },
    Status,
    List,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = MainArguments::parse();

    match arguments.command {
        MainCommand::Show { regular_expression } => {
            match Namespace::open(arguments.namespace_identifier)? {
                Some(namespace) => {
                    let matched_zones = namespace
                        .zones()
                        .r#match(&regular_expression)?
                        .collect::<Result<Vec<_>, _>>()?;

                    for zone in matched_zones {
                        println!("{}", zone.identifier().uuid());
                    }
                }
                None => {}
            }
        }
        MainCommand::Create { include } => {
            let mut configuration_directive = ZoneConfigurationDirective::default();

            match configuration_directive.version_mut() {
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.set_include(include);
                }
            };

            let configuration_directive = configuration_directive;

            let mut namespace = match Namespace::open(arguments.namespace_identifier.clone())? {
                Some(n) => n,
                None => {
                    Namespace::create(arguments.namespace_identifier.clone())?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            println!(
                "{}",
                namespace
                    .zones_mut()
                    .create(ZoneConfiguration::new(
                        configuration_directive,
                        current_dir()?,
                    ))?
                    .uuid()
            );
        }
        MainCommand::Destroy { regular_expression } => {
            match Namespace::open(arguments.namespace_identifier)? {
                Some(namespace) => {
                    let matched_zones = namespace
                        .zones()
                        .r#match(&regular_expression)?
                        .collect::<Result<Vec<_>, _>>()?;

                    for zone in matched_zones {
                        let uuid = zone.identifier().uuid().to_string();

                        zone.destroy()?;

                        println!("{}", uuid);
                    }
                }
                None => {}
            }
        }
        MainCommand::Recreate { regular_expression } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let configuration = zone.configuration()?;

                zone.destroy()?;

                println!("{}", namespace.zones_mut().create(configuration)?.uuid());
            }
        }
        MainCommand::Start { regular_expression } => {
            match Namespace::open(arguments.namespace_identifier)? {
                Some(namespace) => {
                    let matched_zones = namespace
                        .zones()
                        .r#match(&regular_expression)?
                        .collect::<Result<Vec<_>, _>>()?;

                    for mut zone in matched_zones {
                        zone.start()?;
                        println!("{}", zone.identifier().uuid().to_string());
                    }
                }
                None => {}
            }
        }
        MainCommand::Stop { regular_expression } => {
            match Namespace::open(arguments.namespace_identifier)? {
                Some(namespace) => {
                    let matched_zones = namespace
                        .zones()
                        .r#match(&regular_expression)?
                        .collect::<Result<Vec<_>, _>>()?;

                    for zone in matched_zones {
                        let uuid = zone.identifier().uuid().to_string();
                        zone.stop()?;
                        println!("{}", uuid);
                    }
                }
                None => {}
            }
        }
        MainCommand::Restart { regular_expression } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let configuration = zone.configuration()?;

                let zone = match zone.stop()? {
                    Some(mut zone) => {
                        zone.start()?;

                        zone
                    }
                    None => {
                        let identifier = namespace.zones_mut().create(configuration)?;

                        let mut zone = namespace
                            .zones_mut()
                            .open(*identifier.uuid())?
                            .expect("Zone not found");

                        zone.start()?;

                        zone
                    }
                };

                println!("{}", zone.identifier().uuid().to_string());
            }
        }
        MainCommand::Up { regular_expression } => {
            let namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for mut zone in matched_zones {
                match zone.is_running()? {
                    true => {}
                    false => {
                        zone.start()?;
                        println!("{}", zone.identifier().uuid().to_string());
                    }
                }
            }
        }
        MainCommand::Down { regular_expression } => {
            let namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                match zone.is_running()? {
                    true => {
                        let uuid = zone.identifier().uuid().to_string();
                        zone.stop()?;
                        println!("{}", uuid);
                    }
                    false => {}
                }
            }
        }
        MainCommand::Reup { regular_expression } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for mut zone in matched_zones {
                let configuration = zone.configuration()?;

                let zone = if zone.is_running()? {
                    match zone.stop()? {
                        Some(mut zone) => {
                            zone.start()?;

                            zone
                        }
                        None => {
                            let identifier = namespace.zones_mut().create(configuration)?;

                            let mut zone = namespace
                                .zones_mut()
                                .open(*identifier.uuid())?
                                .expect("Zone not found");

                            zone.start()?;

                            zone
                        }
                    }
                } else {
                    zone.start()?;

                    zone
                };

                println!("{}", zone.identifier().uuid().to_string());
            }
        }
        MainCommand::Deploy { include } => {
            let mut configuration_directive = ZoneConfigurationDirective::default();

            match configuration_directive.version_mut() {
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.set_include(include);
                }
            };

            let configuration_directive = configuration_directive;

            let mut namespace = match Namespace::open(arguments.namespace_identifier.clone())? {
                Some(n) => n,
                None => {
                    Namespace::create(arguments.namespace_identifier.clone())?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            let zone_identifier = namespace.zones_mut().create(ZoneConfiguration::new(
                configuration_directive,
                current_dir()?,
            ))?;

            let mut zone = namespace
                .zones_mut()
                .open(*zone_identifier.uuid())?
                .expect("Zone not found");

            zone.start()?;

            println!("{}", zone_identifier);
        }
        MainCommand::Undeploy { regular_expression } => {
            let namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let uuid = zone.identifier().uuid().to_string();

                if zone.is_running()? {
                    match zone.stop()? {
                        Some(zone) => zone.destroy()?,
                        None => {}
                    };
                } else {
                    zone.destroy()?;
                }

                println!("{}", uuid);
            }
        }
        MainCommand::Redeploy { regular_expression } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let configuration = zone.configuration()?;

                if zone.is_running()? {
                    match zone.stop()? {
                        Some(zone) => zone.destroy()?,
                        None => {}
                    };
                } else {
                    zone.destroy()?;
                }

                let zone_identifier = namespace.zones_mut().create(configuration)?;

                let mut zone = namespace
                    .zones_mut()
                    .open(*zone_identifier.uuid())?
                    .expect("Zone not found");

                zone.start()?;

                println!("{}", zone_identifier.uuid().to_string());
            }
        }
        MainCommand::Send { regular_expression } => {
            let namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");
            let matched_zones = namespace
                .zones()
                .r#match(&regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;
            let mut stdout = stdout();

            for mut zone in matched_zones {
                zone.send(&mut stdout)?;
            }
        }
        MainCommand::Receive => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");
            let mut stdin = io_stdin();

            loop {
                match namespace.zones_mut().receive(&mut stdin) {
                    Ok(z) => {
                        println!("{}", z.uuid().to_string());
                    }
                    Err(ReceiveZoneError::EmptyInput) => return Ok(()),
                    Err(ReceiveZoneError::IoError(e)) => match e.kind() {
                        ErrorKind::UnexpectedEof => return Ok(()),
                        _ => return Err(e.into()),
                    },
                    Err(e) => return Err(e.into()),
                };
            }
        }
        MainCommand::Run { include } => {
            let mut configuration_directive = ZoneConfigurationDirective::default();

            match configuration_directive.version_mut() {
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.set_include(include);
                }
            };

            let mut namespace = match Namespace::open(arguments.namespace_identifier.clone())? {
                Some(n) => n,
                None => {
                    Namespace::create(arguments.namespace_identifier.clone())?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            match configuration_directive.version_mut() {
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.set_start_after_create(Some(true));
                    version1.set_destroy_after_stop(Some(true));
                }
            }

            let zone_identifier = namespace.zones_mut().create(ZoneConfiguration::new(
                configuration_directive,
                current_dir()?,
            ))?;

            println!("{}", zone_identifier.uuid());
        }
        MainCommand::Status => match Namespace::open(arguments.namespace_identifier)? {
            Some(namespace) => {
                for zone in namespace.zones().iter()? {
                    println!("{:?}", zone?.identifier().uuid());
                }
            }
            None => {}
        },
        MainCommand::List => match Namespace::open(arguments.namespace_identifier)? {
            Some(namespace) => {
                for zone in namespace.zones().iter()? {
                    println!("{:?}", zone?.identifier().uuid());
                }
            }
            None => {}
        },
    };

    Ok(())
}
