use std::f32::consts::PI;

use super::vec3d::Vec3D;

#[derive(Debug)]
pub struct Mat4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn new() -> Self {
        let m = [[0.0; 4]; 4];
        Self { m }
    }
}

pub fn multiply_vector(m: &Mat4x4, i: &Vec3D) -> Vec3D {
    Vec3D {
        x: i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0],
        y: i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1],
        z: i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2],
        w: i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3],
    }
}

pub fn multiply_matrix(m1: &Mat4x4, m2: &Mat4x4) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    for c in 0..4 {
        for r in 0..4 {
            matrix.m[r][c] = m1.m[r][0] * m2.m[0][c]
                + m1.m[r][1] * m2.m[1][c]
                + m1.m[r][2] * m2.m[2][c]
                + m1.m[r][3] * m2.m[3][c];
        }
    }
    matrix
}

pub fn make_identity() -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_rotation_x(angle: f32) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = angle.cos();
    matrix.m[1][2] = angle.sin();
    matrix.m[2][1] = -(angle.sin());
    matrix.m[2][2] = angle.cos();
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_rotation_y(angle: f32) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = angle.cos();
    matrix.m[0][2] = angle.sin();
    matrix.m[2][0] = -(angle.sin());
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = angle.cos();
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_rotation_z(angle: f32) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = angle.cos();
    matrix.m[0][1] = angle.sin();
    matrix.m[1][0] = -(angle.sin());
    matrix.m[1][1] = angle.cos();
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_translation(x: f32, y: f32, z: f32) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix.m[3][0] = x;
    matrix.m[3][1] = y;
    matrix.m[3][2] = z;
    matrix
}

pub fn make_projection(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4x4 {
    let fov_rad = 1.0 / (fov * 0.5 / 180.0 * PI).tan();
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = aspect_ratio * fov_rad;
    matrix.m[1][1] = fov_rad;
    matrix.m[2][2] = far / (far - near);
    matrix.m[3][2] = (-far * near) / (far - near);
    matrix.m[2][3] = 1.0;
    matrix.m[3][3] = 0.0;
    matrix
}
