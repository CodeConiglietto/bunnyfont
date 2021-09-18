use crate::traits::color::Color;

pub trait SourceImage {
    type Color: Color;

    fn get_pixel_dimensions(&self) -> (usize, usize);
}
