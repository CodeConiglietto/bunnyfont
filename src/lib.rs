#[cfg(feature = "utilities")]
pub mod utilities;

pub mod char;
pub mod char_transforms;
pub mod font;
pub mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
