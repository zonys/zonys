use crate::{
    TransformZoneConfiguration, TransformZoneConfigurationContext, TransformZoneConfigurationError,
    ZoneConfigurationVersion1JailCreateUnit, ZoneConfigurationVersion1JailDestroyUnit,
    ZoneConfigurationVersion1JailExecuteUnit, ZoneConfigurationVersion1JailProgramUnit,
    ZoneConfigurationVersion1JailStartUnit, ZoneConfigurationVersion1JailStopUnit,
    ZoneConfigurationVersion1JailUnit, ZoneConfigurationVersion1VolumeUnit,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailDirective {
    from: Option<String>,
    volume: Option<ZoneConfigurationVersion1VolumeDirective>,
    execute: Option<ZoneConfigurationVersion1JailExecuteDirective>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailUnit>
    for ZoneConfigurationVersion1JailDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailUnit::new(
            self.from,
            context
                .work_paths()
                .last()
                .map(|path| path.display().to_string()),
            match self.volume {
                Some(volume) => Some(volume.transform(context)?),
                None => None,
            },
            match self.execute {
                Some(execute) => Some(execute.transform(context)?),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1VolumeDirective {
    #[serde(alias = "auto", rename = "automatic")]
    Automatic,
    #[serde(rename = "zfs")]
    Zfs,
    #[serde(rename = "directory")]
    Directory,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1VolumeUnit>
    for ZoneConfigurationVersion1VolumeDirective
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1VolumeUnit, TransformZoneConfigurationError> {
        match self {
            Self::Automatic => Ok(ZoneConfigurationVersion1VolumeUnit::Automatic),
            Self::Zfs => Ok(ZoneConfigurationVersion1VolumeUnit::Zfs),
            Self::Directory => Ok(ZoneConfigurationVersion1VolumeUnit::Directory),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailExecuteDirective {
    create: Option<ZoneConfigurationVersion1JailCreateDirective>,
    start: Option<ZoneConfigurationVersion1JailStartDirective>,
    stop: Option<ZoneConfigurationVersion1JailStopDirective>,
    destroy: Option<ZoneConfigurationVersion1JailDestroyDirective>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailExecuteUnit>
    for ZoneConfigurationVersion1JailExecuteDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailExecuteUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailExecuteUnit::new(
            match self.create {
                Some(create) => Some(create.transform(context)?),
                None => None,
            },
            match self.start {
                Some(start) => Some(start.transform(context)?),
                None => None,
            },
            match self.stop {
                Some(stop) => Some(stop.transform(context)?),
                None => None,
            },
            match self.destroy {
                Some(destroy) => Some(destroy.transform(context)?),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailCreateDirective {
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailCreateUnit>
    for ZoneConfigurationVersion1JailCreateDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailCreateUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailCreateUnit::new(
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.after {
                Some(after) => Some(
                    after
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailStartDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStartUnit>
    for ZoneConfigurationVersion1JailStartDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStartUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStartUnit::new(
            match self.before {
                Some(before) => Some(
                    before
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.after {
                Some(after) => Some(
                    after
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailStopDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStopUnit>
    for ZoneConfigurationVersion1JailStopDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStopUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStopUnit::new(
            match self.before {
                Some(before) => Some(
                    before
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.after {
                Some(after) => Some(
                    after
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailDestroyDirective {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramDirective>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailDestroyUnit>
    for ZoneConfigurationVersion1JailDestroyDirective
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailDestroyUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailDestroyUnit::new(
            match self.before {
                Some(before) => Some(
                    before
                        .into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
            match self.on {
                Some(on) => Some(
                    on.into_iter()
                        .map(|program| program.transform(context))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                None => None,
            },
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailProgramDirective {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailProgramUnit>
    for ZoneConfigurationVersion1JailProgramDirective
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailProgramUnit, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailProgramUnit::new(
            self.program,
            self.arguments,
            self.environment_variables,
        ))
    }
}
