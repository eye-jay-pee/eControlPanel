mod constants;
pub mod errors;
mod test;

pub struct Height(u32);

impl Height {
    fn validate(&self) -> Result<(), errors::HeightError> {
        use errors::HeightError as HError;
        match self.as_inches() {
            i if i < constants::MIN_INCHES => Err(HError::TooShort),
            i if constants::MAX_INCHES < i => Err(HError::TooTall),
            _ => Ok(()),
        }
    }
    pub fn new(feet: u32, inches: u32) -> Result<Self, errors::HeightError> {
        let new_height = Height(constants::INCHES_PER_FOOT * feet + inches);
        new_height.validate()?;
        Ok(new_height)
    }

    pub fn as_inches(&self) -> u32 {
        self.0
    }
    pub fn as_feet_and_inches(&self) -> (u32, u32) {
        (
            self.as_inches() / constants::INCHES_PER_FOOT,
            self.as_inches() % constants::INCHES_PER_FOOT,
        )
    }
}
impl Height {
    // metric
    pub fn _as_cm(&self) -> u32 {
        (self.as_inches() as f32 * constants::_CM_PER_INCH).round() as u32
    }
    pub fn _from_cm(cm: u32) -> Result<Self, errors::HeightError> {
        let inches = cm as f32 / constants::_CM_PER_INCH;
        Self::new(0, inches.round() as u32)
    }

    // surverying
    pub fn _as_fractional_feet(&self) -> f32 {
        (self.as_inches() as f32) / (constants::INCHES_PER_FOOT as f32)
    }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.as_feet_and_inches();
        write!(f, "{}'{}\"", h.0, h.1)
    }
}
