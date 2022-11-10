use crate::color::Color;
use std::iter::Copied;
use std::slice::Iter;

pub struct Canvas {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Canvas {
    /// Return a new Canvas.
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            // Create a Vec from a given element and size.
            data: vec![Color::new(0, 0, 0); (width * height) as usize],
        }
    }

    /// Returns an iterator which copies all of its elements.
    /// Same as:
    ///
    /// pub fn flat(&self) -> impl Iterator<Item = Color> + '_ {
    ///
    pub fn flat(&self) -> Copied<Iter<'_, Color>> {
        self.data.iter().copied()
    }

    /// Set color to specific pixel in the canvas.
    pub fn write_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.rows_mut().skip(y as usize).next().unwrap()[x as usize] = color;
    }

    /// Returns an iterator which returns entire Y rows of the canvas.
    /// Same as:
    ///
    /// fn rows_mut(&mut self) -> ChunksExactMut<'_, Color> {
    ///
    fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Color]> + '_ {
        self.data.chunks_exact_mut(self.width as usize)
    }

    /// Returns the Color from a specific x, y coordinate in the Canvas.
    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        self.data
            .chunks_exact(self.width as usize)
            .skip(y as usize)
            .next()
            .unwrap()[x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creating a canvas.
    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.flat().all(|x| x == Color::new(0, 0, 0)));
        // equivalent to:
        assert!(c.data.iter().map(|x| *x).all(|x| x == Color::new(0, 0, 0)))
    }

    /// Writing pixels to a canvas.
    #[test]
    fn write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1, 0, 0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
}
