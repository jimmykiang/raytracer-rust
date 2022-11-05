use std::ops::{Add, Mul, Neg, Sub};
use vecmath::{vec4_add, vec4_scale, vec4_sub, Vector4};

// Point is a rust tuple.
#[derive(Debug, Clone, Copy)]
pub struct Point(Vector4<f64>);

impl Point {
    // new Point and enable conversion into f64.
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Point([x.into(), y.into(), z.into(), 1.0])
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

// Implement to enable conversion from vecmath::Vector4 into() Point.
impl From<vecmath::Vector4<f64>> for Point {
    fn from([x, y, z, _]: Vector4<f64>) -> Self {
        Point::new(x, y, z)
    }
}

// Implement to enable conversion from vecmath::Vector4 into() Vector.
impl From<vecmath::Vector4<f64>> for Vector {
    fn from([x, y, z, _]: Vector4<f64>) -> Self {
        Vector::new(x, y, z)
    }
}

// Overload Point + Vector.
impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

// Overload Vector + Point.
impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

// Overload Vector + Vector.
impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

// OverLoad Point - Point.
impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        // Manual implementation.
        // let v = vec4_sub(self.0, rhs.0);
        // Vector::new(v[0], v[1], v[2])

        vec4_sub(self.0, rhs.0).into()
    }
}

// Overload Point - Vector.
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        vec4_sub(self.0, rhs.0).into()
    }
}

// Overload Vector - Vector.
impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        vec4_sub(self.0, rhs.0).into()
    }
}

// Overload unary negation for Vector.
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

// Overload Vector * f64.
impl<T: Into<f64>> Mul<T> for Vector {
    type Output = Vector;

    fn mul(self, rhs: T) -> Self::Output {
        vec4_scale(self.0, rhs.into()).into()
    }
}

// Overload f64 * Vector.
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        vec4_scale(rhs.0, self).into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector(Vector4<f64>);

impl Vector {
    // new Vector and enable conversion into f64.
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Vector([x.into(), y.into(), z.into(), 0.0])
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
        let p_1 = Point::new(4, 5, 6);
        assert_eq!(p_1.0[3], 1.0);
        assert_eq!(p_1.x(), 4.0);
        assert_eq!(p_1.y(), 5.0);
        assert_eq!(p_1.z(), 6.0);
    }

    // A tuple with w=0 is a vector.
    #[test]
    fn new_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.0[3], 0.0);
        assert_eq!(v.y(), -4.2);
        assert_eq!(v.z(), 3.1);
        assert_eq!(v.w(), 0.0);
        let v_1 = Vector::new(7, 88, 9);
        assert_eq!(v_1.0[3], 0.0);
        assert_eq!(v_1.x(), 7.0);
        assert_eq!(v_1.y(), 88.0);
        assert_eq!(v_1.z(), 9.0);
    }

    // Required by assert_eq for comparing equality of Point and approximating using Epsilon.
    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.approx_eq(other)
        }
    }

    // Required by assert_eq for comparing equality of Vector and approximating using Epsilon.
    impl PartialEq for Vector {
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
        assert_eq!(v + p, Point::new(1.0, 1.0, 6.0));
        assert_eq!(v + v, Vector::new(-4.0, 6.0, 2.0));
    }

    // Subtracting two points.
    #[test]
    fn substract_tuples() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    // Subtracting a vector from a point.
    #[test]
    fn substract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    // Subtracting a vector from the zero vector.
    #[test]
    fn sub_vec_from_zero() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    // Negating a tuple.
    #[test]
    fn negating_tuple() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }

    // Multiplying a tuple by a scalar.
    #[test]
    fn multiply_scalar_tuple() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(a * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(3.5 * a, Vector::new(3.5, -7, 10.5))
    }

    // Multiplying a tuple by a fraction.
    #[test]
    fn multiply_tuple_fraction() {
        let a = Vector::new(1, -2, 3);
        assert_eq!(a * 0.5, Vector::new(0.5, -1, 1.5));
        assert_eq!(0.5 * a, Vector::new(0.5, -1, 1.5));
    }
}
