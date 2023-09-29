use crate::vec2d::Vec2D;

use super::vec3d::Vec3D;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub p: [Vec3D; 3],
    pub t: [Vec2D; 3],
    pub col: [u8; 4],
}

impl Triangle {
    pub fn new(v1: Vec3D, v2: Vec3D, v3: Vec3D) -> Self {
        Self {
            p: [v1, v2, v3],
            t: [Vec2D::empty(), Vec2D::empty(), Vec2D::empty()],
            col: [0xff, 0xff, 0xff, 0xff],
        }
    }

    pub fn new_uv(v1: Vec3D, v2: Vec3D, v3: Vec3D, uv1: Vec2D, uv2: Vec2D, uv3: Vec2D) -> Self {
        Self {
            p: [v1, v2, v3],
            t: [uv1, uv2, uv3],
            col: [0xff, 0xff, 0xff, 0xff],
        }
    }

    pub fn empty() -> Self {
        Self {
            p: [Vec3D::empty(), Vec3D::empty(), Vec3D::empty()],
            t: [Vec2D::empty(), Vec2D::empty(), Vec2D::empty()],
            col: [0xff, 0xff, 0xff, 0xff],
        }
    }
}
