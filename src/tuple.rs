use std::ops::Add;
use vecmath::{vec4_add, Vector4};

// Point is a rust tuple.
#[derive(Debug)]
pub struct Point(Vector4<f64>);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point([x, y, z, 1.0])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn w(&self) -> f64 {
        self.0[3]
    }
}

impl From<vecmath::Vector4<f64>> for Point {
    fn from([x, y, z, _]: Vector4<f64>) -> Self {
        Point::new(x, y, z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

pub struct Vector(Vector4<f64>);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector([x, y, z, 0.0])
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn w(&self) -> f64 {
        self.0[3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approximate_equation::ApproximateEq;

    // A tuple with w=1.0 is a point.
    #[test]
    fn new_point() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.0[3], 1.0);
        assert_eq!(p.y(), -4.2);
        assert_eq!(p.z(), 3.1);
        assert_eq!(p.w(), 1.0);
    }

    // A tuple with w=0 is a vector.
    #[test]
    fn new_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.0[3], 0.0);
        assert_eq!(v.y(), -4.2);
        assert_eq!(v.z(), 3.1);
        assert_eq!(v.w(), 0.0);
    }

    // Required for comparing equality and aproximating using Epsilon.
    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.approx_eq(other)
        }
    }

    // Adding two tuples.
    #[test]
    fn add_tuples() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0));
    }
}
