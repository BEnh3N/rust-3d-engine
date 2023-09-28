use super::vec3d::Vec3D;

#[derive(Clone)]
pub struct Triangle {
    pub p: [Vec3D; 3],
    pub col: [u8; 4],
}

impl Triangle {
    pub fn new(v1: Vec3D, v2: Vec3D, v3: Vec3D) -> Self {
        Self {
            p: [v1, v2, v3],
            col: [0xff, 0xff, 0xff, 0xff],
        }
    }

    pub fn empty() -> Self {
        Self {
            p: [Vec3D::empty(), Vec3D::empty(), Vec3D::empty()],
            col: [0xff, 0xff, 0xff, 0xff],
        }
    }
}
