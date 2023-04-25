#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand};
use std::env::current_dir;
use std::error;
use std::fmt::Debug;
use std::io::{stdin as io_stdin, stdout, ErrorKind};
use std::path::PathBuf;
use zonys_core::zone::{
    ReceiveZoneError, Zone, ZoneConfiguration, ZoneConfigurationDirective,
    ZoneConfigurationVersionDirective,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Parser)]
#[clap(name = "zonys")]
#[clap(about = "Another execution environment manager for the FreeBSD operating system.")]
#[clap(author, version, long_about = None)]
struct MainArguments {
    #[clap(default_value = "/zroot/zonys")]
    base_path: PathBuf,

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
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                println!("{}", zone.identifier().uuid());
            }
        }
        MainCommand::Create { include } => {
            let mut configuration_directive = ZoneConfigurationDirective::default();

            match configuration_directive.version_mut() {
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.set_include(include);
                }
            };

            let mut configuration = ZoneConfiguration::default();
            configuration
                .directives_mut()
                .prepend(Some(&current_dir()?), configuration_directive.clone())?;

            println!(
                "{}",
                Zone::create(&arguments.base_path, configuration)?.uuid()
            );
        }
        MainCommand::Destroy { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let uuid = zone.identifier().uuid().to_string();

                zone.destroy()?;

                println!("{}", uuid);
            }
        }
        MainCommand::Recreate { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let configuration = zone.configuration()?;

                zone.destroy()?;

                println!(
                    "{}",
                    Zone::create(&arguments.base_path, configuration)?.uuid()
                );
            }
        }
        MainCommand::Start { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for mut zone in matched_zones {
                zone.start()?;
                println!("{}", zone.identifier().uuid().to_string());
            }
        }
        MainCommand::Stop { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let uuid = zone.identifier().uuid().to_string();
                zone.stop()?;
                println!("{}", uuid);
            }
        }
        MainCommand::Restart { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let configuration = zone.configuration()?;

                let zone = match zone.stop()? {
                    Some(mut zone) => {
                        zone.start()?;

                        zone
                    }
                    None => {
                        let identifier = Zone::create(&arguments.base_path, configuration)?;
                        let mut zone = Zone::open(identifier)?.expect("Zone not found");

                        zone.start()?;

                        zone
                    }
                };

                println!("{}", zone.identifier().uuid().to_string());
            }
        }
        MainCommand::Up { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for mut zone in matched_zones {
                match zone.running()? {
                    true => {}
                    false => {
                        zone.start()?;
                        println!("{}", zone.identifier().uuid().to_string());
                    }
                }
            }
        }
        MainCommand::Down { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                match zone.running()? {
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
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for mut zone in matched_zones {
                let configuration = zone.configuration()?;

                let zone = if zone.running()? {
                    match zone.stop()? {
                        Some(mut zone) => {
                            zone.start()?;

                            zone
                        }
                        None => {
                            let identifier = Zone::create(&arguments.base_path, configuration)?;

                            let mut zone = Zone::open(identifier)?.expect("Zone not found");

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

            let zone_identifier = Zone::create(
                &arguments.base_path,
                ZoneConfiguration::new(configuration_directive, Vec::default(), current_dir()?),
            )?;

            let mut zone = Zone::open(zone_identifier.clone())?.expect("Zone not found");

            zone.start()?;

            println!("{}", zone_identifier);
        }
        MainCommand::Undeploy { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let uuid = zone.identifier().uuid().to_string();

                if zone.running()? {
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
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            for zone in matched_zones {
                let configuration = zone.configuration()?;

                if zone.running()? {
                    match zone.stop()? {
                        Some(zone) => zone.destroy()?,
                        None => {}
                    };
                } else {
                    zone.destroy()?;
                }

                let zone_identifier = Zone::create(&arguments.base_path, configuration)?;

                let mut zone = Zone::open(zone_identifier.clone())?.expect("Zone not found");

                zone.start()?;

                println!("{}", zone_identifier.uuid().to_string());
            }
        }
        MainCommand::Send { regular_expression } => {
            let matched_zones = Zone::r#match(&arguments.base_path, &regular_expression)?
                .collect::<Result<Vec<_>, _>>()?;

            let mut stdout = stdout();

            for mut zone in matched_zones {
                zone.send(&mut stdout)?;
            }
        }
        MainCommand::Receive => {
            let mut stdin = io_stdin();

            loop {
                match Zone::receive(&arguments.base_path, &mut stdin) {
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

            match configuration_directive.version_mut() {
                ZoneConfigurationVersionDirective::Version1(ref mut version1) => {
                    version1.set_start_after_create(Some(true));
                    version1.set_destroy_after_stop(Some(true));
                }
            }

            let zone_identifier = Zone::create(
                &arguments.base_path,
                ZoneConfiguration::new(configuration_directive, Vec::default(), current_dir()?),
            )?;

            println!("{}", zone_identifier.uuid());
        }
        MainCommand::Status => {
            for zone in Zone::all(&arguments.base_path)? {
                println!("{:?}", zone?.identifier().uuid());
            }
        }
        MainCommand::List => {
            for zone in Zone::all(&arguments.base_path)? {
                println!("{:?}", zone?.identifier().uuid());
            }
        }
    };

    Ok(())
}
