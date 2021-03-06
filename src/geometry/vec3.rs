use rand::{thread_rng, Rng};
use std::f64;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Dimension = f64;
pub const MAX_DIMENSION: Dimension = f64::MAX;
pub const PI_DIMENSION: Dimension = PI;

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

    pub fn new(x: Dimension, y: Dimension, z: Dimension) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let rx = rng.gen_range::<Dimension>(-1.0, 1.0);
            let ry = rng.gen_range::<Dimension>(-1.0, 1.0);
            let xx = rx * rx;
            let yy = ry * ry;
            if xx + yy < 1.0 {
                return Vec3::new(rx, ry, 0.0);
            }
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let rx = rng.gen_range::<Dimension>(-1.0, 1.0);
            let ry = rng.gen_range::<Dimension>(-1.0, 1.0);
            let rz = rng.gen_range::<Dimension>(-1.0, 1.0);
            let p = Vec3::new(rx, ry, rz);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
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

    pub fn cross(&self, other: Vec3) -> Vec3 {
        let xx = self.y * other.z - self.z * other.y;
        let yy = -(self.x * other.z - self.z * other.x);
        let zz = self.x * other.y - self.y * other.x;
        Vec3::new(xx, yy, zz)
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

impl Mul<Vec3> for Dimension {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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
