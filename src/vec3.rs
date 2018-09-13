pub type Dimension = f64;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: Dimension,
    y: Dimension,
    z: Dimension,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

    pub fn new(x: Dimension, y: Dimension, z: Dimension) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
}