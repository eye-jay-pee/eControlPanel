use super::people::errors::PeopleError;

#[derive(Debug)]
pub enum AppLabError {
    EFrameError(eframe::Error),
    PeopleError(super::people::errors::PeopleError),
}
pub type Result = std::result::Result<(), AppLabError>;

impl std::fmt::Display for AppLabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mssg = "Applab error: ";
        match self {
            AppLabError::EFrameError(e) => write!(f, "{}", e),
            AppLabError::PeopleError(p) => write!(f, "{}{}", mssg, p),
        }
    }
}
impl std::error::Error for AppLabError {}

impl From<PeopleError> for AppLabError {
    fn from(error: PeopleError) -> Self {
        AppLabError::PeopleError(error)
    }
}
