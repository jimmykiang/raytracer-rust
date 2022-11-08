use std::ops::Add;

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
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::add(&self, &rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approximate_equation::ApproximateEq;

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
}
