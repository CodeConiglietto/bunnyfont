use crate::{
    char_transforms::{CharMirror, CharRotation},
    traits::color::Color,
};

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
    pub fn new(index: usize) -> Self {
        Self {
            index,
            foreground: Color::new(1.0, 1.0, 1.0, 1.0),
            background: None,
            rotation: CharRotation::None,
            mirror: CharMirror::None,
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

impl<C> Default for BunnyChar<C>
where
    C: Color,
{
    fn default() -> Self {
        Self::new(0)
    }
}
