#[derive(Clone)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn normalise(&self) -> Vec3D {
        let l = length(self);
        Vec3D {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
            w: 0.0,
        }
    }
}

pub fn dot_product(v1: &Vec3D, v2: &Vec3D) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn length(v: &Vec3D) -> f32 {
    dot_product(v, v).sqrt()
}

// pub fn normalise(v: &Vec3D) -> Vec3D {
//     let l = length(v);
//     Vec3D {
//         x: v.x / l,
//         y: v.y / l,
//         z: v.z / l,
//         w: 0.0,
//     }
// }

pub fn cross_product(v1: &Vec3D, v2: &Vec3D) -> Vec3D {
    Vec3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
        w: 0.0,
    }
}

impl std::ops::Add<&Vec3D> for &Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: &Vec3D) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: 0.0,
        }
    }
}

impl std::ops::Sub<&Vec3D> for &Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: &Vec3D) -> Self::Output {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: 0.0,
        }
    }
}

impl std::ops::Mul<f32> for &Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 0.0,
        }
    }
}

impl std::ops::Div<f32> for &Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 0.0,
        }
    }
}
