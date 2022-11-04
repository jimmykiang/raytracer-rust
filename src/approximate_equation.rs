use crate::tuple::{Point, Vector};

pub const EPSILON: f64 = 1e-5;

impl ApproximateEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        if self.is_infinite() {
            other.is_infinite() && self.signum() == other.signum()
        } else {
            (self - other).abs() < EPSILON
        }
    }
}

pub trait ApproximateEq<T = Self> {
    fn approx_eq(&self, other: &T) -> bool;
}

impl ApproximateEq for Point {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x().approx_eq(&other.x())
            && self.y().approx_eq(&other.y())
            && self.z().approx_eq(&other.z())
            && self.w().approx_eq(&other.w())
    }
}

impl ApproximateEq for Vector {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x().approx_eq(&other.x())
            && self.y().approx_eq(&other.y())
            && self.z().approx_eq(&other.z())
            && self.w().approx_eq(&other.w())
    }
}
