#[derive(Clone)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub p: [Vec3D; 3],
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            p: [Vec3D::new(), Vec3D::new(), Vec3D::new()],
        }
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tri_list: Vec<[f32; 9]>) -> Self {
        let mut tris = vec![];
        for tri in tri_list {
            tris.push(Triangle {
                p: [
                    Vec3D {
                        x: tri[0],
                        y: tri[1],
                        z: tri[2],
                    },
                    Vec3D {
                        x: tri[3],
                        y: tri[4],
                        z: tri[5],
                    },
                    Vec3D {
                        x: tri[6],
                        y: tri[7],
                        z: tri[8],
                    },
                ],
            })
        }
        Self { tris }
    }
}

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

pub fn multiply_matrix_vector(i: &Vec3D, m: &Mat4x4) -> Vec3D {
    let mut o = Vec3D::new();
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
    let w = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }

    return o;
}
