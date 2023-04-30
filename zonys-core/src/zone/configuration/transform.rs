use crate::template::{RenderTemplateError, TemplateEngine, TemplateObject};
use crate::zone::configuration::ReadZoneConfigurationDirectiveError;
use std::path::PathBuf;
use url::ParseError;
use ztd::{Constructor, Display, Error, From, Method};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display, Error, From)]
#[From(unnamed)]
pub enum TransformZoneConfigurationError {
    ReadZoneConfigurationDirectiveError(ReadZoneConfigurationDirectiveError),
    ParseUrlParse(ParseError),
    #[From(skip)]
    UnsupportedScheme(String),
    RenderTemplateError(RenderTemplateError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Default, Method)]
#[Method(all)]
pub(crate) struct TransformZoneConfigurationContext {
    template_engine: TemplateEngine,
    variables: TemplateObject,
    work_paths: Vec<PathBuf>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) trait TransformZoneConfiguration<T> {
    fn transform(
        self,
        context: &mut TransformZoneConfigurationContext,
    ) -> Result<T, TransformZoneConfigurationError>;
}
