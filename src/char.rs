use crate::{char_transforms::{CharMirror, CharRotation}, traits::color::Color};

pub struct BunnyChar<C>
where
    C: Color,
{
    pub index: usize,
    pub foreground: C,
    pub background: Option<C>,
    pub rotation: CharRotation,
    pub mirror: CharMirror,
}
