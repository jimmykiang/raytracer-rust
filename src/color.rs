use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(r: impl Into<f64>, g: impl Into<f64>, b: impl Into<f64>) -> Self {
        Color {
            red: r.into(),
            green: g.into(),
            blue: b.into(),
        }
    }

    pub fn red(&self) -> f64 {
        self.red
    }
    pub fn green(&self) -> f64 {
        self.green
    }
    pub fn blue(&self) -> f64 {
        self.blue
    }
    pub fn add(&self, rhs: &Self) -> Self {
        Color::new(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        Color::new(
            self.red() - rhs.red(),
            self.green() - rhs.green(),
            self.blue - rhs.blue(),
        )
    }

    fn scale(&self, scalar: f64) -> Color {
        Color::new(
            self.red() * scalar,
            self.green() * scalar,
            self.blue() * scalar,
        )
    }
}

// Overload Color + Color.
impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::add(&self, &rhs)
    }
}

// Overload Color - Color.
impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::sub(&self, &rhs)
    }
}

// Overload f64 * Color.
impl<T> Mul<T> for Color
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Color::scale(&self, rhs.into())
    }
}

// Overload Color * f64.
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::scale(&rhs, self)
    }
}

// Overload Color * i32.
impl Mul<Color> for i32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::scale(&rhs, self as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approximate_equation::ApproximateEq;

    // Required by assert_eq for comparing equality of Color and approximating using Epsilon.
    impl PartialEq for Color {
        fn eq(&self, other: &Self) -> bool {
            self.approx_eq(other)
        }
    }

    // Colors are (red, green, blue) tuples.
    #[test]
    fn colors() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    // Adding colors.
    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    // Subtracting colors.
    #[test]
    fn substract_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    // Multiplying a color by a scalar.
    #[test]
    fn multiply_color_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2, Color::new(0.4, 0.6, 0.8));
        assert_eq!(2 * c, Color::new(0.4, 0.6, 0.8));
    }
}
