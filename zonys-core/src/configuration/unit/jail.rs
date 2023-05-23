use crate::{
    TransformZoneConfiguration, TransformZoneConfigurationContext, TransformZoneConfigurationError,
    ZoneConfigurationVersion1JailCreateDirective, ZoneConfigurationVersion1JailDestroyDirective,
    ZoneConfigurationVersion1JailDirective, ZoneConfigurationVersion1JailExecuteDirective,
    ZoneConfigurationVersion1JailProgramDirective, ZoneConfigurationVersion1JailStartDirective,
    ZoneConfigurationVersion1JailStopDirective, ZoneConfigurationVersion1VolumeDirective,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1VolumeUnit {
    Automatic,
    Zfs,
    Directory,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1VolumeDirective>
    for ZoneConfigurationVersion1VolumeUnit
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1VolumeDirective, TransformZoneConfigurationError> {
        match self {
            Self::Automatic => Ok(ZoneConfigurationVersion1VolumeDirective::Automatic),
            Self::Zfs => Ok(ZoneConfigurationVersion1VolumeDirective::Zfs),
            Self::Directory => Ok(ZoneConfigurationVersion1VolumeDirective::Directory),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailUnit {
    from: Option<String>,
    from_work_path: Option<String>,
    volume: Option<ZoneConfigurationVersion1VolumeUnit>,
    execute: Option<ZoneConfigurationVersion1JailExecuteUnit>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailDirective>
    for ZoneConfigurationVersion1JailUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailDirective::new(
            self.from,
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

#[derive(Clone, Constructor, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailExecuteUnit {
    create: Option<ZoneConfigurationVersion1JailCreateUnit>,
    start: Option<ZoneConfigurationVersion1JailStartUnit>,
    stop: Option<ZoneConfigurationVersion1JailStopUnit>,
    destroy: Option<ZoneConfigurationVersion1JailDestroyUnit>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailExecuteDirective>
    for ZoneConfigurationVersion1JailExecuteUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailExecuteDirective, TransformZoneConfigurationError>
    {
        Ok(ZoneConfigurationVersion1JailExecuteDirective::new(
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

#[derive(Clone, Constructor, Default, Deserialize, Debug, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1JailCreateUnit {
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailCreateDirective>
    for ZoneConfigurationVersion1JailCreateUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailCreateDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailCreateDirective::new(
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
pub struct ZoneConfigurationVersion1JailStartUnit {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStartDirective>
    for ZoneConfigurationVersion1JailStartUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStartDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStartDirective::new(
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
pub struct ZoneConfigurationVersion1JailStopUnit {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    after: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailStopDirective>
    for ZoneConfigurationVersion1JailStopUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailStopDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1JailStopDirective::new(
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
pub struct ZoneConfigurationVersion1JailDestroyUnit {
    before: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
    on: Option<Vec<ZoneConfigurationVersion1JailProgramUnit>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailDestroyDirective>
    for ZoneConfigurationVersion1JailDestroyUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailDestroyDirective, TransformZoneConfigurationError>
    {
        Ok(ZoneConfigurationVersion1JailDestroyDirective::new(
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
pub struct ZoneConfigurationVersion1JailProgramUnit {
    program: String,
    arguments: Option<Vec<String>>,
    environment_variables: Option<HashMap<String, String>>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1JailProgramDirective>
    for ZoneConfigurationVersion1JailProgramUnit
{
    fn transform(
        self,
        _context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1JailProgramDirective, TransformZoneConfigurationError>
    {
        Ok(ZoneConfigurationVersion1JailProgramDirective::new(
            self.program,
            self.arguments,
            self.environment_variables,
        ))
    }
}
