#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand};
use serde_yaml::from_reader;
use std::error;
use std::fmt::Debug;
use std::io::{stdin as io_stdin, stdout, Stdin};
use zonys_core::namespace::{Namespace, NamespaceIdentifier};
use zonys_core::zone::{ZoneConfiguration, ZoneIdentifierUuid};

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
    Create {
        #[clap(short, long)]
        stdin: bool,
    },
    Destroy {
        uuid: ZoneIdentifierUuid,
    },
    Recreate {
        uuid: ZoneIdentifierUuid,
    },
    Start {
        uuid: ZoneIdentifierUuid,
    },
    Stop {
        uuid: ZoneIdentifierUuid,
    },
    Restart {
        uuid: ZoneIdentifierUuid,
    },
    Up {
        uuid: ZoneIdentifierUuid,
    },
    Down {
        uuid: ZoneIdentifierUuid,
    },
    Reup {
        uuid: ZoneIdentifierUuid,
    },
    Deploy {
        #[clap(short, long)]
        stdin: bool,
    },
    Undeploy {
        uuid: ZoneIdentifierUuid,
    },
    Redeploy {
        uuid: ZoneIdentifierUuid,
    },
    Send {
        uuid: ZoneIdentifierUuid,
    },
    Receive,
    Run {
        #[clap(short, long)]
        stdin: bool,
    },
    Status,
    List,
    Purge,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = MainArguments::parse();

    match arguments.command {
        MainCommand::Create { stdin } => {
            let configuration = if stdin {
                from_reader::<Stdin, ZoneConfiguration>(io_stdin())?
            } else {
                ZoneConfiguration::default()
            };

            let mut namespace = match Namespace::open(arguments.namespace_identifier.clone())? {
                Some(n) => n,
                None => {
                    Namespace::create(arguments.namespace_identifier.clone())?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            println!("{}", namespace.zones_mut().create(configuration)?.uuid());
        }
        MainCommand::Destroy { uuid } => match Namespace::open(arguments.namespace_identifier)? {
            Some(mut namespace) => {
                namespace
                    .zones_mut()
                    .open(uuid)?
                    .expect("Zone not found")
                    .destroy()?;
            }
            None => {}
        },
        MainCommand::Recreate { uuid } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let zone = namespace.zones_mut().open(uuid)?.expect("Zone not found");

            let configuration = zone.configuration()?;

            zone.destroy()?;

            println!("{}", namespace.zones_mut().create(configuration)?);
        }
        MainCommand::Start { uuid } => match Namespace::open(arguments.namespace_identifier)? {
            Some(mut namespace) => {
                namespace
                    .zones_mut()
                    .open(uuid)?
                    .expect("Zone not found")
                    .start()?;
            }
            None => {}
        },
        MainCommand::Stop { uuid } => match Namespace::open(arguments.namespace_identifier)? {
            Some(mut namespace) => {
                namespace
                    .zones_mut()
                    .open(uuid)?
                    .expect("Zone not found")
                    .stop()?;
            }
            None => {}
        },
        MainCommand::Restart { uuid } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let zone = namespace.zones_mut().open(uuid)?.expect("Zone not found");

            let configuration = zone.configuration()?;

            match zone.stop()? {
                Some(mut zone) => zone.start()?,
                None => {
                    let identifier = namespace.zones_mut().create(configuration)?;

                    let mut zone = namespace
                        .zones_mut()
                        .open(*identifier.uuid())?
                        .expect("Zone not found");

                    zone.start()?;
                }
            }
        }
        MainCommand::Up { uuid } => {
            let mut zone = Namespace::open(arguments.namespace_identifier)?
                .expect("Namespace not found")
                .zones_mut()
                .open(uuid)?
                .expect("Zone not found");

            match zone.is_running()? {
                true => {}
                false => {
                    zone.start()?;
                }
            }
        }
        MainCommand::Down { uuid } => {
            let zone = Namespace::open(arguments.namespace_identifier)?
                .expect("Namespace not found")
                .zones_mut()
                .open(uuid)?
                .expect("Zone not found");

            match zone.is_running()? {
                true => {
                    zone.stop()?;
                }
                false => {}
            }
        }
        MainCommand::Reup { uuid } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let mut zone = namespace.zones_mut().open(uuid)?.expect("Zone not found");

            let configuration = zone.configuration()?;

            if zone.is_running()? {
                match zone.stop()? {
                    Some(mut zone) => zone.start()?,
                    None => {
                        let identifier = namespace.zones_mut().create(configuration)?;

                        let mut zone = namespace
                            .zones_mut()
                            .open(*identifier.uuid())?
                            .expect("Zone not found");

                        zone.start()?;
                    }
                }
            } else {
                zone.start()?;
            }
        }
        MainCommand::Deploy { stdin } => {
            let configuration = if stdin {
                from_reader::<Stdin, ZoneConfiguration>(io_stdin())?
            } else {
                ZoneConfiguration::default()
            };

            let mut namespace = match Namespace::open(arguments.namespace_identifier.clone())? {
                Some(n) => n,
                None => {
                    Namespace::create(arguments.namespace_identifier.clone())?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            let zone_identifier = namespace.zones_mut().create(configuration)?;

            let mut zone = namespace
                .zones_mut()
                .open(*zone_identifier.uuid())?
                .expect("Zone not found");

            zone.start()?;

            println!("{}", zone_identifier);
        }
        MainCommand::Undeploy { uuid } => {
            let zone = Namespace::open(arguments.namespace_identifier)?
                .expect("Namespace not found")
                .zones_mut()
                .open(uuid)?
                .expect("Zone not found");

            if zone.is_running()? {
                match zone.stop()? {
                    Some(zone) => zone.destroy()?,
                    None => {}
                };
            } else {
                zone.destroy()?;
            }
        }
        MainCommand::Redeploy { uuid } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let zone = namespace.zones_mut().open(uuid)?.expect("Zone not found");

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

            println!("{}", zone_identifier);
        }
        MainCommand::Send { uuid } => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            let mut zone = namespace.zones_mut().open(uuid)?.expect("Zone not found");

            zone.send(&mut stdout())?;
        }
        MainCommand::Receive => {
            let mut namespace =
                Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found");

            println!("{}", namespace.zones_mut().receive(&mut io_stdin())?.uuid());
        }
        MainCommand::Run { stdin } => {
            let mut configuration = if stdin {
                from_reader::<Stdin, ZoneConfiguration>(io_stdin())?
            } else {
                ZoneConfiguration::default()
            };

            let mut namespace = match Namespace::open(arguments.namespace_identifier.clone())? {
                Some(n) => n,
                None => {
                    Namespace::create(arguments.namespace_identifier.clone())?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            match configuration {
                ZoneConfiguration::Version1(ref mut version1) => {
                    version1.set_start_after_create(Some(true));
                    version1.set_destroy_after_stop(Some(true));
                }
            }

            let zone_identifier = namespace.zones_mut().create(configuration)?;

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
        MainCommand::Purge => match Namespace::open(arguments.namespace_identifier)? {
            Some(namespace) => {
                for zone in namespace.zones().iter()? {
                    let zone = zone?;
                    println!("{:?}", zone.identifier().uuid());
                    zone.destroy()?;
                }
            }
            None => {}
        },
    };

    Ok(())
}
