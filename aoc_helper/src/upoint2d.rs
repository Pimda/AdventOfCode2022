use std::{
    fmt::Display,
    ops::{Mul, Sub, Div, Add},
};

use crate::point2d::Point2D;

/// Can be used when indexing into arrays is required
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UPoint2D {
    pub x: usize,
    pub y: usize,
}

impl UPoint2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_i32_or_throw(&self) -> (i32, i32) {
        let ix: i32 = self.x.try_into().expect("X cannot be converted to i32");
        let iy: i32 = self.y.try_into().expect("Y cannot be converted to i32");
        (ix, iy)
    }
}

impl Sub<Point2D> for UPoint2D {
    type Output = Point2D;

    fn sub(self, subtractor: Point2D) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        Point2D::new(ix - subtractor.x, iy - subtractor.y)
    }
}

impl Sub<i32> for UPoint2D {
    type Output = Point2D;

    fn sub(self, subtractor: i32) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        Point2D::new(ix - subtractor, iy - subtractor)
    }
}

impl Add<Point2D> for UPoint2D{
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        Point2D::new(ix + rhs.x, iy + rhs.y)
    }
}

impl Sub<UPoint2D> for UPoint2D {
    type Output = Point2D;

    fn sub(self, subtractor: UPoint2D) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        let (ixs, iys) = subtractor.to_i32_or_throw();
        Point2D::new(ix - ixs, iy - iys)
    }
}

impl Mul<u32> for UPoint2D {
    type Output = UPoint2D;

    fn mul(self, multiplier: u32) -> Self::Output {
        let multiplier: usize = multiplier.try_into().expect("Multiplier cannot be converted to usize");
        UPoint2D::new(self.x * multiplier, self.y * multiplier)
    }
}

impl Div<u32> for UPoint2D {
    type Output = UPoint2D;

    fn div(self, divisor: u32) -> Self::Output {
        let divisor: usize = divisor.try_into().expect("Multiplier cannot be converted to usize");
        UPoint2D::new(self.x / divisor, self.y / divisor)
    }
}

impl Display for UPoint2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
