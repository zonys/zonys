use crate::{
    CleanupZoneVolumeError, CreateZoneVolumeError, DestroyZoneVolumeError, FromHandler,
    FromHandlerError, JailZoneConfigurationStep, JailZoneConfigurationVolumeType,
    ReadZoneConfigurationError, ReceiveZoneVolumeError, SendZoneVolumeError, Zone,
    ZoneConfigurationTypeReader, ZoneDirectoryVolume, ZoneTransmissionReader,
    ZoneTransmissionWriter, ZoneVolume, ZoneZfsVolume,
};
use jail::{
    CreateJailError, DestroyJailError, ExecuteJailError, Jail, JailId, JailName, JailParameter,
    TryIntoJailIdError,
};
use std::path::PathBuf;
use ztd::{Constructor, Display, Error, From};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CreateJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CreateZoneVolumeError(CreateZoneVolumeError),
    FromHandlerError(FromHandlerError),
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    ExecuteJailError(ExecuteJailError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StartJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CreateJailError(CreateJailError),
    ExecuteJailError(ExecuteJailError),
    #[Display("Jail is already running")]
    AlreadyRunning,
    TryIntoJailIdError(TryIntoJailIdError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum StopJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    DestroyJailError(DestroyJailError),
    ExecuteJailError(ExecuteJailError),
    #[Display("Jail is not running")]
    NotRunning,
    TryIntoJailIdError(TryIntoJailIdError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum DestroyJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    DestroyZoneVolumeError(DestroyZoneVolumeError),
    CreateJailError(CreateJailError),
    DestroyJailError(DestroyJailError),
    ExecuteJailError(ExecuteJailError),
    #[Display("Jail is running")]
    Running,
    TryIntoJailIdError(TryIntoJailIdError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    SendZoneVolumeError(SendZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    ReceiveZoneVolumeError(ReceiveZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupJailZoneError {
    ReadZoneConfigurationError(ReadZoneConfigurationError),
    CleanupZoneVolumeError(CleanupZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct JailZone<T> {
    zone: T,
}

impl<'a> JailZone<&'a Zone> {
    fn root_directory_path(&self) -> PathBuf {
        self.zone.paths().root_directory()
    }

    fn jail_name(&self) -> JailName {
        JailName::new(self.zone.identifier().to_string())
    }

    fn jail_id(&self) -> Result<Option<JailId>, TryIntoJailIdError> {
        self.jail_name().try_into()
    }

    fn jail_parameters(&self) -> Vec<JailParameter> {
        vec![
            JailParameter::new("persist", "true"),
            JailParameter::new("name", self.jail_name().to_string()),
            JailParameter::new("path", self.root_directory_path().display().to_string()),
        ]
    }

    fn hold_jail<F, E>(&self, function: F) -> Result<(), E>
    where
        F: FnOnce(&Jail) -> Result<(), E>,
        E: From<CreateJailError> + From<DestroyJailError>,
    {
        let jail = Jail::create(self.jail_parameters())?;

        let result = function(&jail);

        jail.destroy()?;

        result
    }

    fn execute<E>(&self, jail: &Jail, step: &JailZoneConfigurationStep<'a>) -> Result<(), E>
    where
        E: From<ExecuteJailError>,
    {
        jail.execute(
            step.program(),
            &step.arguments().as_ref().cloned().unwrap_or_default(),
            step.environment_variables()
                .as_ref()
                .cloned()
                .unwrap_or_default(),
        )?;

        Ok(())
    }

    pub(super) fn volume(&self) -> Result<ZoneVolume<&Zone>, ReadZoneConfigurationError> {
        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        match jail.volume() {
            JailZoneConfigurationVolumeType::Automatic => {
                todo!()
            }
            JailZoneConfigurationVolumeType::Zfs => {
                Ok(ZoneVolume::Zfs(ZoneZfsVolume::new(self.zone)))
            }
            JailZoneConfigurationVolumeType::Directory => {
                Ok(ZoneVolume::Directory(ZoneDirectoryVolume::new(self.zone)))
            }
        }
    }

    pub(super) fn create(&self) -> Result<(), CreateJailZoneError> {
        let volume = self.volume()?;
        volume.create()?;

        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        if let Some(from) = jail.from() {
            FromHandler::handle(from, &volume.root_directory_path())?;
        }

        self.hold_jail::<_, CreateJailZoneError>(|handle| {
            for step in jail.create_steps() {
                self.execute::<CreateJailZoneError>(handle, &step)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    pub(super) fn start(&self) -> Result<(), StartJailZoneError> {
        if self.jail_id()?.is_some() {
            return Err(StartJailZoneError::AlreadyRunning);
        }

        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        let handle = Jail::create(self.jail_parameters())?;

        for step in jail.start_steps() {
            self.execute::<StartJailZoneError>(&handle, &step)?;
        }

        Ok(())
    }

    pub(super) fn stop(&self) -> Result<(), StopJailZoneError> {
        let jail_id = match self.jail_id()? {
            Some(jail_id) => jail_id,
            None => return Err(StopJailZoneError::NotRunning),
        };

        let handle = match Jail::open(jail_id) {
            Some(jail) => jail,
            None => return Err(StopJailZoneError::NotRunning),
        };

        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        for step in jail.stop_steps() {
            self.execute::<StopJailZoneError>(&handle, &step)?;
        }

        Ok(())
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyJailZoneError> {
        if self.jail_id()?.is_some() {
            return Err(DestroyJailZoneError::Running);
        }

        let volume = self.volume()?;

        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        self.hold_jail::<_, DestroyJailZoneError>(|handle| {
            for step in jail.destroy_steps() {
                self.execute::<DestroyJailZoneError>(handle, &step)?;
            }

            Ok(())
        })?;

        volume.destroy()?;

        Ok(())
    }

    pub(super) fn send(
        &self,
        writer: &mut ZoneTransmissionWriter,
    ) -> Result<(), SendJailZoneError> {
        self.volume()?.send(writer)?;

        Ok(())
    }

    pub(super) fn receive(
        zone: &'a Zone,
        reader: &mut ZoneTransmissionReader,
    ) -> Result<Self, ReceiveJailZoneError> {
        ZoneVolume::receive(zone, reader)?;

        Ok(Self::new(zone))
    }

    pub(super) fn cleanup(&self) -> Result<(), CleanupJailZoneError> {
        self.volume()?.cleanup()?;

        Ok(())
    }
}
