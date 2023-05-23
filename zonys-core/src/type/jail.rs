use crate::{
    CleanupZoneVolumeError, CreateZoneVolumeError, DestroyZoneVolumeError, FromHandler,
    FromHandlerError, JailZoneConfigurationStep, OpenZoneVolumeError, ReadZoneConfigurationError,
    ReceiveZoneVolumeError, RenderTemplateError, SendZoneVolumeError, TemplateEngine,
    TemplateObject, Zone, ZoneConfigurationTypeReader, ZoneTransmissionReader,
    ZoneTransmissionWriter, ZoneVolume,
};
use jail::{
    CreateJailError, DestroyJailError, ExecuteJailError, Jail, JailId, JailName, JailParameter,
    TryIntoJailIdError,
};
use std::collections::HashMap;
use std::path::PathBuf;
use url::{ParseError, Url};
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
    OpenZoneVolumeError(OpenZoneVolumeError),
    #[Display("Volume does not exist")]
    VolumeNotExisting,
    RenderTemplateError(RenderTemplateError),
    UnsupportedScheme(String),
    UrlParseError(ParseError),
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
    RenderTemplateError(RenderTemplateError),
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
    RenderTemplateError(RenderTemplateError),
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
    OpenZoneVolumeError(OpenZoneVolumeError),
    #[Display("Volume does not exist")]
    VolumeNotExisting,
    RenderTemplateError(RenderTemplateError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum SendJailZoneError {
    OpenZoneVolumeError(OpenZoneVolumeError),
    #[Display("Volume does not exist")]
    VolumeNotExisting,
    SendZoneVolumeError(SendZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum ReceiveJailZoneError {
    ReceiveZoneVolumeError(ReceiveZoneVolumeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum CleanupJailZoneError {
    OpenZoneVolumeError(OpenZoneVolumeError),
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

    fn execute<E>(
        &self,
        jail: &Jail,
        step: &JailZoneConfigurationStep<'a>,
        template_engine: &TemplateEngine,
        variables: &TemplateObject,
    ) -> Result<(), E>
    where
        E: From<ExecuteJailError> + From<RenderTemplateError>,
    {
        jail.execute(
            &template_engine.render(&variables, step.program())?,
            &step
                .arguments()
                .as_ref()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|argument| template_engine.render(&variables, &argument))
                .collect::<Result<Vec<String>, RenderTemplateError>>()?,
            step.environment_variables()
                .as_ref()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|(key, value)| {
                    Ok((
                        template_engine.render(&variables, &key)?,
                        template_engine.render(&variables, &value)?,
                    ))
                })
                .collect::<Result<HashMap<String, String>, RenderTemplateError>>()?,
        )?;

        Ok(())
    }

    pub(super) fn volume(&self) -> Result<Option<ZoneVolume<&Zone>>, OpenZoneVolumeError> {
        ZoneVolume::open(self.zone)
    }

    pub(super) fn create(&self) -> Result<(), CreateJailZoneError> {
        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        ZoneVolume::create(self.zone, jail.volume())?;
        let volume = match self.volume()? {
            None => return Err(CreateJailZoneError::VolumeNotExisting),
            Some(volume) => volume,
        };

        let engine = TemplateEngine::default();
        let variables = reader.variables();

        if let Some(from) = jail.from() {
            let from = &engine.render(&variables, from)?;

            let from = match Url::parse(from) {
                Ok(url) if url.scheme() == "" || url.scheme() == "file" => {
                    let url_path = PathBuf::from(url.path());
                    if url_path.is_relative() {
                        jail.from_work_path()
                            .map(PathBuf::from)
                            .unwrap_or_default()
                            .join(url_path)
                            .display()
                            .to_string()
                    } else {
                        url_path.display().to_string()
                    }
                }
                Ok(_url) => from.clone(),
                Err(ParseError::RelativeUrlWithoutBase) => {
                    let url_path = PathBuf::from(from);
                    if url_path.is_relative() {
                        jail.from_work_path()
                            .map(PathBuf::from)
                            .unwrap_or_default()
                            .join(url_path)
                            .display()
                            .to_string()
                    } else {
                        url_path.display().to_string()
                    }
                }
                Err(error) => return Err(CreateJailZoneError::from(error)),
            };

            FromHandler::handle(&from, &volume.root_directory_path())?;
        }

        self.hold_jail::<_, CreateJailZoneError>(|handle| {
            for step in jail.create_steps() {
                self.execute::<CreateJailZoneError>(handle, &step, &engine, &variables)?;
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

        let engine = TemplateEngine::default();
        let variables = reader.variables();

        let handle = Jail::create(self.jail_parameters())?;

        for step in jail.start_steps() {
            self.execute::<StartJailZoneError>(&handle, &step, &engine, &variables)?;
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

        let engine = TemplateEngine::default();
        let variables = reader.variables();

        for step in jail.stop_steps() {
            self.execute::<StopJailZoneError>(&handle, &step, &engine, &variables)?;
        }

        handle.destroy()?;

        Ok(())
    }

    pub(super) fn destroy(&self) -> Result<(), DestroyJailZoneError> {
        if self.jail_id()?.is_some() {
            return Err(DestroyJailZoneError::Running);
        }

        let volume = match self.volume()? {
            None => return Err(DestroyJailZoneError::VolumeNotExisting),
            Some(volume) => volume,
        };

        let reader = self.zone.configuration().reader()?;

        let jail = match reader.r#type() {
            ZoneConfigurationTypeReader::Jail(jail) => jail,
            ZoneConfigurationTypeReader::Chroot(_chroot) => unreachable!(),
        };

        let engine = TemplateEngine::default();
        let variables = reader.variables();

        self.hold_jail::<_, DestroyJailZoneError>(|handle| {
            for step in jail.destroy_steps() {
                self.execute::<DestroyJailZoneError>(handle, &step, &engine, &variables)?;
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
        let volume = match self.volume()? {
            None => return Err(SendJailZoneError::VolumeNotExisting),
            Some(volume) => volume,
        };

        volume.send(writer)?;

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
        if let Some(volume) = self.volume()? {
            volume.cleanup()?;
        }

        Ok(())
    }
}
