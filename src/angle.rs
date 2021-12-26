use std::{f32::consts::PI, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Radians(f32);
impl From<Radians> for f32 {
    fn from(Radians(f): Radians) -> Self {
        f
    }
}
impl From<f32> for Radians {
    fn from(arg: f32) -> Self {
        Self(arg)
    }
}
impl From<Degrees> for Radians {
    fn from(Degrees(deg): Degrees) -> Self {
        Radians(deg * PI / 180.0)
    }
}
impl fmt::Display for Radians {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}rad", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Degrees(f32);
impl From<Radians> for Degrees {
    fn from(Radians(rad): Radians) -> Self {
        Degrees(rad * 180.0 / PI)
    }
}
impl fmt::Display for Degrees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}deg", self.0)
    }
}
