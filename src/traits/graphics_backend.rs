use crate::traits::{color::Color, source_image::SourceImage};

pub trait GraphicsBackend {
    type Texture: SourceImage;
    type Color: Color;
    type Error;
}
