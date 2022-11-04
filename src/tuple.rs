use vecmath::Vector4;

// Point is a rust tuple.
pub struct Point(Vector4<f64>);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point([x, y, z, 1.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A tuple with w=1.0 is a point.
    #[test]
    fn new_point() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.0[3], 1.0);
    }
}
