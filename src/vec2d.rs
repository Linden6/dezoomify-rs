use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Vec2d {
    pub x: u32,
    pub y: u32,
}

impl Vec2d {
    pub fn max(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
    pub fn min(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
    pub fn ceil_div(self, other: Vec2d) -> Vec2d {
        let x = self.x / other.x + if self.x % other.x == 0 { 0 } else { 1 };
        let y = self.y / other.y + if self.y % other.y == 0 { 0 } else { 1 };
        Vec2d { x, y }
    }
}

impl std::fmt::Display for Vec2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x={} y={}", self.x, self.y)
    }
}

impl Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Mul<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<u32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: u32) -> Self::Output {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<u32> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: u32) -> Self::Output {
        Vec2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}