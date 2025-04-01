use super::height::errors::HeightError;
use std::fmt;

#[derive(Debug)]
pub enum AttributeError {
    _WeightError,
    HeightError(HeightError),
}
impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeError::_WeightError => write!(f, "weight error"),
            AttributeError::HeightError(e) => write!(f, "height error: {}", e),
        }
    }
}
impl std::error::Error for AttributeError {}

impl From<HeightError> for AttributeError {
    fn from(error: HeightError) -> Self {
        AttributeError::HeightError(error)
    }
}
