use crate::traits::{color::Color, source_image::SourceImage};

use std::path::Path;

pub trait GraphicsBackend {
    type Texture: SourceImage;
    type Color: Color;
    type Error;

    fn load(&mut self, source_image_path: &Path) -> Result<Self::Texture, Self::Error>;
}
