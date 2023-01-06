use std::{
    fmt::Display,
    ops::{Mul, Sub, Div, Add},
};

use crate::vectors::Vec2D;

/// Can be used when indexing into arrays is required
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UVec2D {
    pub x: usize,
    pub y: usize,
}

impl UVec2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_i32_or_throw(self) -> (i32, i32) {
        let ix: i32 = self.x.try_into().expect("X cannot be converted to i32");
        let iy: i32 = self.y.try_into().expect("Y cannot be converted to i32");
        (ix, iy)
    }
}

impl Sub<Vec2D> for UVec2D {
    type Output = Vec2D;

    fn sub(self, subtractor: Vec2D) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        Vec2D::new(ix - subtractor.x, iy - subtractor.y)
    }
}

impl Sub<i32> for UVec2D {
    type Output = Vec2D;

    fn sub(self, subtractor: i32) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        Vec2D::new(ix - subtractor, iy - subtractor)
    }
}

impl Add<Vec2D> for UVec2D{
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        Vec2D::new(ix + rhs.x, iy + rhs.y)
    }
}

impl Sub<UVec2D> for UVec2D {
    type Output = Vec2D;

    fn sub(self, subtractor: UVec2D) -> Self::Output {
        let (ix, iy) = self.to_i32_or_throw();
        let (ixs, iys) = subtractor.to_i32_or_throw();
        Vec2D::new(ix - ixs, iy - iys)
    }
}

impl Mul<u32> for UVec2D {
    type Output = UVec2D;

    fn mul(self, multiplier: u32) -> Self::Output {
        let multiplier: usize = multiplier.try_into().expect("Multiplier cannot be converted to usize");
        UVec2D::new(self.x * multiplier, self.y * multiplier)
    }
}

impl Div<u32> for UVec2D {
    type Output = UVec2D;

    fn div(self, divisor: u32) -> Self::Output {
        let divisor: usize = divisor.try_into().expect("Multiplier cannot be converted to usize");
        UVec2D::new(self.x / divisor, self.y / divisor)
    }
}

impl Display for UVec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
