#[derive(Debug)]
pub enum HeightError {
    TooShort,
    TooTall,
}
impl std::fmt::Display for HeightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use super::constants as con;
        match self {
            Self::TooShort => write!(f, "below min, {}", &con::MIN_HEIGHT),
            Self::TooTall => write!(f, "above max, {}", &con::MAX_HEIGHT),
        }
    }
}
impl std::error::Error for HeightError {}
