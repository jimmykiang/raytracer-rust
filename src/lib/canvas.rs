use crate::color::Color;
use std::iter::Copied;
use std::ptr::write;
use std::slice::{ChunksExact, Iter};

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
    fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Color]> {
        self.data.chunks_exact_mut(self.width as usize)
    }

    // pub fn rows(&self) -> ChunksExact<'_, Color> {
    pub fn rows(&self) -> impl Iterator<Item = &[Color]> {
        self.data.chunks_exact(self.width as usize)
    }

    /// Returns the Color from a specific x, y coordinate in the Canvas.
    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        self.data
            .chunks_exact(self.width as usize)
            .skip(y as usize)
            .next()
            .unwrap()[x as usize]
    }

    pub fn canvas_to_ppm(&self, vec: &mut Vec<u8>) -> std::io::Result<()> {
        let mut line_guard = MaxWidthWriter::new(70, vec);
        self.write_ppm_header(&mut line_guard)?;
        self.write_ppm_data(&mut line_guard);

        Ok(())
    }

    /// The parameter struct MaxWidthWriter is replaced by impl std::io::Write
    /// since Vec<u8> can be replaced by T: std::io::Write for the member of the struct definition.
    // fn write_ppm_header(&self, line_guard: &mut MaxWidthWriter) -> std::io::Result<()> {
    fn write_ppm_header(&self, line_guard: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(line_guard, "P3\n{} {}\n255\n", self.width, self.height)
    }
    // fn write_ppm_data(&self, line_guard: &mut MaxWidthWriter<Vec<u8>>) {
    fn write_ppm_data(&self, line_guard: &mut impl std::io::Write) -> std::io::Result<()> {
        for row in self.rows() {
            for (i, pixel) in row.iter().enumerate() {
                if i > 0 {
                    write!(line_guard, " ")?;
                }
                let (r, g, b) = pixel.to_u8();
                write!(line_guard, "{}", r)?;
                write!(line_guard, " {}", g)?;
                write!(line_guard, " {}", b)?;
            }
            write!(line_guard, "\n")?;
        }
        Ok(())
    }
}

// struct MaxWidthWriter<'a> {
//     writer_vec: &'a mut Vec<u8>,
//     line_buffer: Vec<u8>,
//     number_columns: usize,
// }

/// MaxWidthWrite with generics.
/// Vec<u8> replaced by T: std::io::Write
struct MaxWidthWriter<'a, T>
where
    T: std::io::Write,
{
    writer_vec: &'a mut T,
    line_buffer: Vec<u8>,
    number_columns: usize,
}

// impl<'a> MaxWidthWriter<'a> {
//     fn new(number_columns: usize, writer_vec: &'a mut Vec<u8>) -> Self {
//         MaxWidthWriter {
//             writer_vec,
//             line_buffer: Vec::<u8>::new(),
//             number_columns,
//         }
//     }

impl<'a, T> MaxWidthWriter<'a, T>
where
    T: std::io::Write,
{
    // fn new(number_columns: usize, writer_vec: &'a mut Vec<u8>) -> Self {
    fn new(number_columns: usize, writer_vec: &'a mut T) -> Self {
        MaxWidthWriter {
            writer_vec,
            line_buffer: Vec::<u8>::new(),
            number_columns,
        }
    }

    pub fn flush_partial(&mut self, x: usize) -> std::io::Result<()> {
        self.line_buffer[x] = b'\n';
        self.writer_vec.write(&self.line_buffer[..=x])?;
        // println!(
        //     "Before: {:?}",
        //     String::from_utf8(Vec::from(self.line_buffer.clone())).unwrap()
        // );
        self.line_buffer.copy_within(x + 1.., 0);
        // println!(
        //     "after: {:?}",
        //     String::from_utf8(Vec::from(self.line_buffer.clone())).unwrap()
        // );
        self.line_buffer.truncate(self.line_buffer.len() - x - 1);
        Ok(())
    }
}

/// std::io::Write implementation required on MaxWidthWriter to satisfy: the write! macro.
impl<'a, T> std::io::Write for MaxWidthWriter<'a, T>
where
    T: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // println!("buf: {:?}", String::from_utf8(Vec::from(buf)).unwrap());
        self.line_buffer.extend_from_slice(buf);

        while let Some(x) = self.line_buffer.iter().position(|&pos| pos == b'\n') {
            // println!(
            //     "before: {:?}",
            //     String::from_utf8(Vec::from(self.line_buffer.clone())).unwrap()
            // );
            self.flush_partial(x)?;
            // println!(
            //     "after: {:?}",
            //     String::from_utf8(Vec::from(self.line_buffer.clone())).unwrap()
            // );
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer_vec.write(&self.line_buffer)?;
        self.line_buffer.clear();
        self.writer_vec.flush()
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

    /// Constructing the PPM header.
    #[test]
    fn construct_ppm_header() {
        let c = Canvas::new(5, 3);
        let mut buf = Vec::<u8>::new();
        c.canvas_to_ppm(&mut buf);
        let ppm = String::from_utf8(buf).unwrap();
        assert_eq!(
            ppm.lines().take(3).collect::<Vec<_>>().join("\n"),
            "\
P3
5 3
255"
        );
    }

    /// Constructing the PPM pixel data.
    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0, 0);
        let c2 = Color::new(0, 0.5, 0);
        let c3 = Color::new(-0.5, 0, 1);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let mut buf = Vec::<u8>::new();
        c.canvas_to_ppm(&mut buf).unwrap();
        let ppm = String::from_utf8(buf).unwrap();
        assert_eq!(
            ppm.lines().skip(3).take(3).collect::<Vec<_>>().join("\n"),
            "\
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        );
    }
}
