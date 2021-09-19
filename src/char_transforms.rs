use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

#[derive(Clone, Copy)]
pub enum CharRotation {
    None,
    Rotation90,
    Rotation180,
    Rotation270,
}

impl CharRotation {
    pub fn into_rotation(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Rotation90 => FRAC_PI_2,
            Self::Rotation180 => PI,
            Self::Rotation270 => 3.0 * FRAC_PI_4,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CharMirror {
    None,
    MirrorX,
    MirrorY,
    MirrorBoth,
}

impl CharMirror {
    pub fn into_scale(&self) -> (f32, f32) {
        match self {
            Self::None => (1.0, 1.0),
            Self::MirrorX => (-1.0, 1.0),
            Self::MirrorY => (1.0, -1.0),
            Self::MirrorBoth => (-1.0, -1.0),
        }
    }
}
