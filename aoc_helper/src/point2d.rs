use std::{fmt::Display, ops::{Mul, Sub, Add, Div}};

use crate::{upoint2d::UPoint2D, math};

/// Can be used when negative values are possible
#[derive(Clone, Copy)]
pub struct Point2D{
    pub x: i32,
    pub y: i32
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self{
        Self{x, y}
    }

    pub fn rotate_left(&mut self){
        let x = self.x;
        self.x = -self.y;
        self.y = x;
    }

    pub fn rotate_right(&mut self){
        let x = self.x;
        self.x = self.y;
        self.y = -x;
    }

    pub fn as_upoint2d(&self) -> UPoint2D{
        let x: usize = self.x.try_into().expect("X cannot be converted to usize");
        let y: usize = self.y.try_into().expect("X cannot be converted to usize");
        UPoint2D::new(x, y)
    }

    pub fn positive_mod(&self, modulus: &Point2D) -> Point2D{
        Point2D::new(math::positive_mod(self.x, modulus.x), math::positive_mod(self.y, modulus.y))
    }
}

impl Add<Point2D> for Point2D{
    type Output = Point2D;

    fn add(self, adder: Point2D) -> Self::Output {
        Point2D::new(self.x + adder.x, self.y + adder.y)
    }
}

impl Add<i32> for Point2D{
    type Output = Point2D;

    fn add(self, adder: i32) -> Self::Output {
        Point2D::new(self.x + adder, self.y + adder)
    }
}

impl Sub<Point2D> for Point2D{
    type Output = Point2D;

    fn sub(self, subtractor: Point2D) -> Self::Output {
        Point2D::new(self.x - subtractor.x, self.y - subtractor.y)
    }
}

impl Sub<i32> for Point2D{
    type Output = Point2D;

    fn sub(self, subtractor: i32) -> Self::Output {
        Point2D::new(self.x - subtractor, self.y - subtractor)
    }
}

impl Mul<i32> for Point2D{
    type Output = Point2D;

    fn mul(self, multiplier: i32) -> Self::Output {
        Point2D::new(self.x * multiplier, self.y * multiplier)
    }
}

impl Div<i32> for Point2D{
    type Output = Point2D;

    fn div(self, divisor: i32) -> Self::Output {
        Point2D::new(self.x / divisor, self.y / divisor)
    }
}

impl Display for Point2D{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}