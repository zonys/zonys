#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand};
use serde_yaml::from_reader;
use std::error;
use std::fmt::Debug;
use std::io::{stdin as io_stdin, Stdin};
use zonys_core::namespace::{Namespace, NamespaceIdentifier};
use zonys_core::zone::{ZoneConfiguration, ZoneIdentifierUuid, ZoneStatus};

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

            let mut namespace = match Namespace::open(&arguments.namespace_identifier)? {
                Some(n) => n,
                None => {
                    Namespace::create(&arguments.namespace_identifier)?;
                    Namespace::open(arguments.namespace_identifier)?.expect("Namespace not found")
                }
            };

            println!("{}", namespace.zones_mut().create(configuration)?);
        }
        MainCommand::Destroy { uuid } => match Namespace::open(&arguments.namespace_identifier)? {
            Some(mut namespace) => {
                namespace
                    .zones_mut()
                    .open(uuid)?
                    .expect("Zone not found")
                    .destroy()?;
            }
            None => {}
        },
        MainCommand::Start { uuid } => match Namespace::open(&arguments.namespace_identifier)? {
            Some(mut namespace) => {
                namespace
                    .zones_mut()
                    .open(uuid)?
                    .expect("Zone not found")
                    .start()?;
            }
            None => {}
        },
        MainCommand::Stop { uuid } => match Namespace::open(&arguments.namespace_identifier)? {
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
            let mut zone = Namespace::open(&arguments.namespace_identifier)?
                .expect("Namespace not found")
                .zones_mut()
                .open(uuid)?
                .expect("Zone not found");

            zone.stop()?;
            zone.start()?;
        }
        MainCommand::Up { uuid } => {
            let mut zone = Namespace::open(&arguments.namespace_identifier)?
                .expect("Namespace not found")
                .zones_mut()
                .open(uuid)?
                .expect("Zone not found");

            match zone.status()? {
                ZoneStatus::Running => {},
                ZoneStatus::NotRunning => {
                    zone.start()?;
                },
            }
        },
        MainCommand::Status => match Namespace::open(&arguments.namespace_identifier)? {
            Some(namespace) => {
                for zone in namespace.zones().iter()? {
                    println!("{:?}", zone?.identifier().uuid());
                }
            }
            None => {}
        },
        MainCommand::List => match Namespace::open(&arguments.namespace_identifier)? {
            Some(namespace) => {
                for zone in namespace.zones().iter()? {
                    println!("{:?}", zone?.identifier().uuid());
                }
            }
            None => {}
        },
        MainCommand::Purge => match Namespace::open(&arguments.namespace_identifier)? {
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
