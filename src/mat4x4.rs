use std::f64::consts::PI;

use crate::vec3d::{cross_product, dot_product};

use super::vec3d::Vec3D;

#[derive(Debug)]
pub struct Mat4x4 {
    pub m: [[f64; 4]; 4],
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

pub fn point_at(pos: &Vec3D, target: &Vec3D, up: &Vec3D) -> Mat4x4 {
    // Calculate new forward direction
    let mut new_forward = target - pos;
    new_forward = new_forward.normalise();

    // Calculate new up direction
    let a = &new_forward * dot_product(up, &new_forward);
    let mut new_up = up - &a;
    new_up = new_up.normalise();

    // Calculate new right direction
    let new_right = cross_product(&new_up, &new_forward);

    // Construct dimensioning and translation matrix
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = new_right.x;
    matrix.m[0][1] = new_right.y;
    matrix.m[0][2] = new_right.z;
    matrix.m[0][3] = 0.0;
    matrix.m[1][0] = new_up.x;
    matrix.m[1][1] = new_up.y;
    matrix.m[1][2] = new_up.z;
    matrix.m[1][3] = 0.0;
    matrix.m[2][0] = new_forward.x;
    matrix.m[2][1] = new_forward.y;
    matrix.m[2][2] = new_forward.z;
    matrix.m[2][3] = 0.0;
    matrix.m[3][0] = pos.x;
    matrix.m[3][1] = pos.y;
    matrix.m[3][2] = pos.z;
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn quick_inverse(m: &Mat4x4) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = m.m[0][0];
    matrix.m[0][1] = m.m[1][0];
    matrix.m[0][2] = m.m[2][0];
    matrix.m[0][3] = 0.0;
    matrix.m[1][0] = m.m[0][1];
    matrix.m[1][1] = m.m[1][1];
    matrix.m[1][2] = m.m[2][1];
    matrix.m[1][3] = 0.0;
    matrix.m[2][0] = m.m[0][2];
    matrix.m[2][1] = m.m[1][2];
    matrix.m[2][2] = m.m[2][2];
    matrix.m[2][3] = 0.0;
    matrix.m[3][0] =
        -(m.m[3][0] * matrix.m[0][0] + m.m[3][1] * matrix.m[1][0] + m.m[3][2] * matrix.m[2][0]);
    matrix.m[3][1] =
        -(m.m[3][0] * matrix.m[0][1] + m.m[3][1] * matrix.m[1][1] + m.m[3][2] * matrix.m[2][1]);
    matrix.m[3][2] =
        -(m.m[3][0] * matrix.m[0][2] + m.m[3][1] * matrix.m[1][2] + m.m[3][2] * matrix.m[2][2]);
    matrix.m[3][3] = 1.0;
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

pub fn make_rotation_x(angle: f64) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = angle.cos();
    matrix.m[1][2] = angle.sin();
    matrix.m[2][1] = -(angle.sin());
    matrix.m[2][2] = angle.cos();
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_rotation_y(angle: f64) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = angle.cos();
    matrix.m[0][2] = angle.sin();
    matrix.m[2][0] = -(angle.sin());
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = angle.cos();
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_rotation_z(angle: f64) -> Mat4x4 {
    let mut matrix = Mat4x4::new();
    matrix.m[0][0] = angle.cos();
    matrix.m[0][1] = angle.sin();
    matrix.m[1][0] = -(angle.sin());
    matrix.m[1][1] = angle.cos();
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix
}

pub fn make_translation(x: f64, y: f64, z: f64) -> Mat4x4 {
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

pub fn make_projection(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> Mat4x4 {
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
