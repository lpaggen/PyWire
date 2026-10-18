use std::borrow::Borrow;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionError {
    path: String,
    message: String,
}

impl ConversionError {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

impl Error for ConversionError {}

pub fn required<T, U, V>(
    value: Option<&T>,
    path: &str,
    convert: impl FnOnce(&U) -> Result<V, ConversionError>,
) -> Result<V, ConversionError>
where
    T: Borrow<U>,
{
    let value = value.ok_or_else(|| ConversionError::new(path, "missing required field"))?;
    convert(value.borrow())
}

pub fn optional<T, U, V>(
    value: Option<&T>,
    _path: &str,
    convert: impl FnOnce(&U) -> Result<V, ConversionError>,
) -> Result<Option<V>, ConversionError>
where
    T: Borrow<U>,
{
    value.map(|value| convert(value.borrow())).transpose()
}

pub fn borrowed<T, U, V>(
    value: &T,
    convert: impl FnOnce(&U) -> Result<V, ConversionError>,
) -> Result<V, ConversionError>
where
    T: Borrow<U>,
{
    convert(value.borrow())
}
