use std::ops::{Add, Div, Mul, Neg, Sub};
use vecmath::{
    vec3_cross, vec4_add, vec4_dot, vec4_len, vec4_normalized, vec4_scale, vec4_sub, Vector4,
};

/// Point is a rust tuple.
#[derive(Debug, Clone, Copy)]
pub struct Point(Vector4<f64>);

impl Point {
    /// New Point and enable conversion into f64.
    /// same as:
    /// #
    /// pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
    /// But then all passed arguments have to be the same int or f64,
    /// This works:
    /// let p_1 = Point::new(4, 5, 6);
    /// but mixed types will fail:
    /// let p_1 = Point::new(4, 5.0, 6);
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

/// Implement to enable conversion from vecmath::Vector4 into() Point.
impl From<vecmath::Vector4<f64>> for Point {
    fn from([x, y, z, _]: Vector4<f64>) -> Self {
        Point::new(x, y, z)
    }
}

/// Implement to enable conversion from vecmath::Vector4 into() Vector.
impl From<vecmath::Vector4<f64>> for Vector {
    fn from([x, y, z, _]: Vector4<f64>) -> Self {
        Vector::new(x, y, z)
    }
}

/// Overload Point + Vector.
impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

/// Overload Vector + Point.
impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

/// Overload Vector + Vector.
impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        vec4_add(self.0, rhs.0).into()
    }
}

/// OverLoad Point - Point.
impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        // Manual implementation.
        // let v = vec4_sub(self.0, rhs.0);
        // Vector::new(v[0], v[1], v[2])

        vec4_sub(self.0, rhs.0).into()
    }
}

/// Overload Point - Vector.
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        vec4_sub(self.0, rhs.0).into()
    }
}

/// Overload Vector - Vector.
impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        vec4_sub(self.0, rhs.0).into()
    }
}

/// Overload unary negation for Vector.
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

/// Overload Vector * f64.
/// impl<T: Into<f64>> Mul<T> for Vector {
impl<T> Mul<T> for Vector
where
    T: Into<f64>,
{
    type Output = Vector;

    fn mul(self, rhs: T) -> Self::Output {
        vec4_scale(self.0, rhs.into()).into()
    }
}

/// Overload f64 * Vector.
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        vec4_scale(rhs.0, self).into()
    }
}

/// Overload Vector / f64.
impl<T: Into<f64>> Div<T> for Vector {
    type Output = Vector;

    fn div(self, rhs: T) -> Self::Output {
        vec4_scale(self.0, 1.0 / rhs.into()).into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector(Vector4<f64>);

impl Vector {
    /// New Vector and enable conversion into f64.
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

    /// Magnitude of Vector.
    pub fn magnitude(&self) -> f64 {
        vec4_len(self.0)
    }

    /// Normalize Vector.
    pub fn normalize(&self) -> Self {
        Vector(vec4_normalized(self.0))
    }

    /// Dot product of: Vector * Vector.
    pub fn dot(&self, rhs: &Self) -> f64 {
        vec4_dot(self.0, rhs.0)
    }

    /// Cross product: Vector x Vector.
    pub fn cross(&self, rhs: &Self) -> Self {
        let cross = vec3_cross([self.x(), self.y(), self.z()], [rhs.x(), rhs.y(), rhs.z()]);
        Vector::new(cross[0], cross[1], cross[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approximate_equation::ApproximateEq;

    /// A tuple with w=1.0 is a point.
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

    /// A tuple with w=0 is a vector.
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

    /// Required by assert_eq for comparing equality of Point and approximating using Epsilon.
    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.approx_eq(other)
        }
    }

    /// Required by assert_eq for comparing equality of Vector and approximating using Epsilon.
    impl PartialEq for Vector {
        fn eq(&self, other: &Self) -> bool {
            self.approx_eq(other)
        }
    }

    /// Adding two tuples.
    #[test]
    fn add_tuples() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0));
        assert_eq!(v + p, Point::new(1.0, 1.0, 6.0));
        assert_eq!(v + v, Vector::new(-4.0, 6.0, 2.0));
    }

    /// Subtracting two points.
    #[test]
    fn substract_tuples() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    /// Subtracting a vector from a point.
    #[test]
    fn substract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    /// Subtracting a vector from the zero vector.
    #[test]
    fn sub_vec_from_zero() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    /// Negating a tuple.
    #[test]
    fn negating_tuple() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }

    /// Multiplying a tuple by a scalar.
    #[test]
    fn multiply_scalar_tuple() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(a * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(3.5 * a, Vector::new(3.5, -7, 10.5))
    }

    /// Multiplying a tuple by a fraction.
    #[test]
    fn multiply_tuple_fraction() {
        let a = Vector::new(1, -2, 3);
        assert_eq!(a * 0.5, Vector::new(0.5, -1, 1.5));
        assert_eq!(0.5 * a, Vector::new(0.5, -1, 1.5));
    }

    /// Dividing a tuple by a scalar.
    #[test]
    fn divide_tuple_scalar() {
        let a = Vector::new(1, -2, 3);
        assert_eq!(a / 2.0, Vector::new(0.5, -1, 1.5));
    }

    /// Computing the magnitude of vector(1, 0, 0).
    #[test]
    fn magnitude_vector_100() {
        let v = Vector::new(1, 0, 0);
        assert_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of vector(0, 1, 0).
    #[test]
    fn magnitude_vector_010() {
        let v = Vector::new(0, 1, 0);
        assert_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of vector(0, 0, 1).
    #[test]
    fn magnitude_vector_001() {
        let v = Vector::new(0, 1, 0);
        assert_eq!(v.magnitude(), 1.0);
    }

    /// Computing the magnitude of vector(1, 2, 3).
    #[test]
    fn magnitude_vector_123() {
        let v = Vector::new(1, 2, 3);
        assert_eq!(v.magnitude(), 14f64.sqrt());
    }

    /// Computing the magnitude of vector(-1, -2, -3).
    #[test]
    fn magnitude_vector_negative_123() {
        let v = Vector::new(-1, -2, -3);
        assert_eq!(v.magnitude(), 14.0f64.sqrt());
    }

    /// Normalizing vector(4, 0, 0) gives (1, 0, 0).
    #[test]
    fn normalize_vector_400() {
        let v = Vector::new(4, 0, 0);
        assert_eq!(v.normalize(), Vector::new(1, 0, 0));
    }

    /// Normalizing vector(1, 2, 3).
    #[test]
    fn normalize_vector_123() {
        let v = Vector::new(1, 2, 3);
        assert_eq!(
            v.normalize(),
            Vector::new(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
        );
    }

    /// The magnitude of a normalized vector.
    #[test]
    fn magnitude_nomalized_vector() {
        let v = Vector::new(1, 2, 3);
        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    /// The dot product of two tuples.
    #[test]
    fn dot_product() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);
        assert_eq!(a.dot(&b), 20.0);
    }

    /// The cross product of two vectors.
    #[test]
    fn cross_product() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);
        assert_eq!(a.cross(&b), Vector::new(-1, 2, -1));
        assert_eq!(b.cross(&a), Vector::new(1, -2, 1));
    }
}
