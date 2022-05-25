#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    char_transforms::{CharMirror, CharRotation},
    traits::color::Color,
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BunnyChar<C> {
    pub index: usize,
    pub foreground: C,
    pub background: Option<C>,
    pub rotation: CharRotation,
    pub mirror: CharMirror,
}

impl<C> BunnyChar<C>
where
    C: Color,
{
    pub fn new(index: usize, foreground: C, background: Option<C>, rotation: CharRotation, mirror: CharMirror) -> Self {
        Self {
            index,
            foreground,
            background,
            rotation,
            mirror,
        }
    }

    pub fn foreground(self, foreground: C) -> Self {
        Self { foreground, ..self }
    }

    pub fn background(self, background: Option<C>) -> Self {
        Self { background, ..self }
    }

    pub fn rotation(self, rotation: CharRotation) -> Self {
        Self { rotation, ..self }
    }

    pub fn mirror(self, mirror: CharMirror) -> Self {
        Self { mirror, ..self }
    }
}
