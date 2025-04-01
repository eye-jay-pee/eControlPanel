pub const _CM_PER_INCH: f32 = 2.54;
pub const INCHES_PER_FOOT: u32 = 12;

pub const MIN_FEET: u32 = 1;
pub const MAX_FEET: u32 = 9;

pub const MIN_INCHES: u32 = INCHES_PER_FOOT * MIN_FEET;
pub const MAX_INCHES: u32 = INCHES_PER_FOOT * MAX_FEET;

pub const MIN_HEIGHT: super::Height = super::Height(MIN_INCHES);
pub const MAX_HEIGHT: super::Height = super::Height(MAX_INCHES);
