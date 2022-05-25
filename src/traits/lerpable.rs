pub trait Lerpable {
    fn lerp(a: &Self, b: &Self, scalar: f32) -> Self;
}