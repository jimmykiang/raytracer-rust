use crate::color::Color;
use std::iter::Copied;
use std::slice::Iter;

pub struct Canvas {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            // Create a Vec from a given element and size.
            data: vec![Color::new(0, 0, 0); (width * height) as usize],
        }
    }

    // Returns an iterator which copies all of its elements.
    // pub fn flat(&self) -> impl Iterator<Item = Color> + '_ {
    pub fn flat(&self) -> Copied<Iter<'_, Color>> {
        self.data.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Creating a canvas.
    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.flat().all(|x| x == Color::new(0, 0, 0)));
    }
}
