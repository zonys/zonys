use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use liquid::model::{
    ArrayView, DisplayCow, KString, KStringCow, ObjectIndex, ObjectRender, ObjectSource, Scalar,
    State, Value,
};
use liquid::{Object, ObjectView, Parser};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type Error = liquid::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TemplateObject = Object;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TemplateValue = Value;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub type TemplateScalar = Scalar;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct TemplateEngine {
    parser: Parser,
}

impl TemplateEngine {
    pub fn render<T>(&self, variables: &T, input: &str) -> Result<String, Error>
    where
        T: ObjectView,
    {
        let mut input = String::from(input);

        loop {
            let output = self.parser.parse(&input)?.render(&variables)?;

            if output == input {
                return Ok(output);
            }

            input = output;
        }
    }
}
