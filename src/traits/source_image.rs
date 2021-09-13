pub trait SourceImage {
    fn get_pixel_dimensions(&self) -> (usize, usize);
}
