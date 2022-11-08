use std::ops::{Add, Sub};

#[derive(Debug)]
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
}
