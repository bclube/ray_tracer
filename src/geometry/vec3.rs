use std::f64;
use std::ops::{Add, Div, Mul, Sub};

pub type Dimension = f64;
pub const MAX_DIMENSION: Dimension = f64::MAX;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: Dimension,
    pub y: Dimension,
    pub z: Dimension,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: Dimension, y: Dimension, z: Dimension) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length(&self) -> Dimension {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> Dimension {
        let xx = self.x * self.x;
        let yy = self.y * self.y;
        let zz = self.z * self.z;
        xx + yy + zz
    }

    pub fn dot(&self, other: Vec3) -> Dimension {
        let xx = self.x * other.x;
        let yy = self.y * other.y;
        let zz = self.z * other.z;
        xx + yy + zz
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div<Dimension> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: Dimension) -> Vec3 {
        self * scalar.recip()
    }
}

impl Mul<Dimension> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: Dimension) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
