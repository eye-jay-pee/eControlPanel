use super::attributes::errors::AttributeError;
use std::fmt;

#[derive(Debug)]
pub enum PeopleError {
    _Incomplete,
    AttributeError(AttributeError),
}
impl fmt::Display for PeopleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeopleError::_Incomplete => write!(f, "Missing data"),
            PeopleError::AttributeError(ae) => write!(f, "Attribute {}", ae),
        }
    }
}
impl std::error::Error for PeopleError {}

impl From<AttributeError> for PeopleError {
    fn from(error: AttributeError) -> Self {
        PeopleError::AttributeError(error)
    }
}
