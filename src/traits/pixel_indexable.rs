pub trait PixelIndexable<T> {
    fn get_pixel_at(&self, x: usize, y: usize) -> &T;
}