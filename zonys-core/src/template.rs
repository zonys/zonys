use liquid::model::{Scalar, Value};
use liquid::{Object, ObjectView, Parser};
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TemplateObject = Object;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TemplateValue = Value;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TemplateScalar = Scalar;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum RenderTemplateError {
    LiquidError(liquid::Error),
    EvaluationLoopExisting(String),
}

impl Debug for RenderTemplateError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::LiquidError(error) => Debug::fmt(error, formatter),
            Self::EvaluationLoopExisting(input) => write!(
                formatter,
                "Evaluation loop with expression \"{}\" is existing",
                input
            ),
        }
    }
}

impl Display for RenderTemplateError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::LiquidError(error) => Display::fmt(error, formatter),
            Self::EvaluationLoopExisting(input) => write!(
                formatter,
                "Evaluation loop with expression \"{}\" is existing",
                input
            ),
        }
    }
}

impl error::Error for RenderTemplateError {}

impl From<liquid::Error> for RenderTemplateError {
    fn from(error: liquid::Error) -> Self {
        Self::LiquidError(error)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct TemplateEngine {
    parser: Parser,
}

impl TemplateEngine {
    pub fn render<T>(&self, variables: &T, input: &str) -> Result<String, RenderTemplateError>
    where
        T: ObjectView,
    {
        let mut rendered_values = HashSet::<String>::new();

        let mut input = String::from(input);

        loop {
            let output = self.parser.parse(&input)?.render(&variables)?;

            if output == input {
                return Ok(output);
            }

            if !rendered_values.insert(output.clone()) {
                return Err(RenderTemplateError::EvaluationLoopExisting(output));
            }

            input = output;
        }
    }
}
