mod chroot;
mod jail;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::configuration::unit::jail::*;
pub use chroot::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    TemplateObject, TransformZoneConfiguration, TransformZoneConfigurationContext,
    TransformZoneConfigurationError, ZoneConfigurationDirective,
    ZoneConfigurationVersion1Directive, ZoneConfigurationVersion1TypeDirective,
    ZoneConfigurationVersionDirective,
};
use serde::{Deserialize, Serialize};
use ztd::{Constructor, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Debug, Default, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationUnit {
    version: ZoneConfigurationVersionUnit,
}

impl TransformZoneConfiguration<ZoneConfigurationDirective> for ZoneConfigurationUnit {
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationDirective::new(
            self.version.transform(context)?,
        ))
    }
}

impl ZoneConfigurationUnit {
    pub fn transform(self) -> Result<ZoneConfigurationDirective, TransformZoneConfigurationError> {
        <Self as TransformZoneConfiguration<ZoneConfigurationDirective>>::transform(
            self,
            &mut TransformZoneConfigurationContext::default(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersionUnit {
    Version1(ZoneConfigurationVersion1Unit),
}

impl Default for ZoneConfigurationVersionUnit {
    fn default() -> Self {
        Self::Version1(ZoneConfigurationVersion1Unit::default())
    }
}

impl TransformZoneConfiguration<ZoneConfigurationVersionDirective>
    for ZoneConfigurationVersionUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersionDirective, TransformZoneConfigurationError> {
        match self {
            Self::Version1(version1) => Ok(ZoneConfigurationVersionDirective::Version1(
                version1.transform(context)?,
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Constructor, Default, Debug, Deserialize, Method, Serialize)]
#[Method(all)]
pub struct ZoneConfigurationVersion1Unit {
    includes: Option<Vec<String>>,
    units: Option<Vec<ZoneConfigurationUnit>>,
    tags: Option<Vec<String>>,
    variables: Option<TemplateObject>,
    r#type: ZoneConfigurationVersion1TypeUnit,
    start_after_create: Option<bool>,
    destroy_after_stop: Option<bool>,
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1Directive>
    for ZoneConfigurationVersion1Unit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1Directive, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1Directive::new(
            self.includes,
            self.tags,
            self.variables,
            self.r#type.transform(context)?,
            self.start_after_create,
            self.destroy_after_stop,
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ZoneConfigurationVersion1TypeUnit {
    Jail(ZoneConfigurationVersion1JailUnit),
}

impl Default for ZoneConfigurationVersion1TypeUnit {
    fn default() -> Self {
        Self::Jail(ZoneConfigurationVersion1JailUnit::default())
    }
}

impl TransformZoneConfiguration<ZoneConfigurationVersion1TypeDirective>
    for ZoneConfigurationVersion1TypeUnit
{
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<ZoneConfigurationVersion1TypeDirective, TransformZoneConfigurationError> {
        Ok(ZoneConfigurationVersion1TypeDirective::Jail(match self {
            Self::Jail(jail) => jail.transform(context)?,
        }))
    }
}
