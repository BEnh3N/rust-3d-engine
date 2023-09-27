use std::mem;

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
    pub col: [u8; 4],
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            p: [Vec3D::new(), Vec3D::new(), Vec3D::new()],
            col: [0xff, 0xff, 0xff, 0xff],
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
                col: [0xff, 0xff, 0xff, 0xff],
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

pub fn get_color(lum: f32) -> [u8; 4] {
    let r = (lum * 255.0) as u8;
    let g = (lum * 255.0) as u8;
    let b = (lum * 255.0) as u8;
    [r, g, b, 0xff]
}

pub fn draw_triangle(
    frame: &mut [u8],
    canvas_width: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    rgba: &[u8; 4],
) {
    let mut x1 = x1;
    let mut y1 = y1;
    let mut x2 = x2;
    let mut y2 = y2;
    let mut x3 = x3;
    let mut y3 = y3;

    let canvas_height = frame.len() as i32 / 4 / canvas_width;

    if y1 > y2 {
        mem::swap(&mut y1, &mut y2);
        mem::swap(&mut x1, &mut x2);
    }
    if y1 > y3 {
        mem::swap(&mut y1, &mut y3);
        mem::swap(&mut x1, &mut x3);
    }
    if y2 > y3 {
        mem::swap(&mut y2, &mut y3);
        mem::swap(&mut x2, &mut x3);
    }

    let mut dy1 = y2 - y1;
    let mut dx1 = x2 - x1;

    let dy2 = y3 - y1;
    let dx2 = x3 - x1;

    let mut dax_step = 0.0;
    let mut dbx_step = 0.0;

    if dy1 != 0 {
        dax_step = dx1 as f32 / dy1.abs() as f32;
    }
    if dy2 != 0 {
        dbx_step = dx2 as f32 / dy2.abs() as f32;
    }

    if dy1 != 0 {
        for i in y1..=y2 {
            let mut ax = x1 + ((i - y1) as f32 * dax_step) as i32;
            let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

            if ax > bx {
                mem::swap(&mut ax, &mut bx);
            }

            for j in ax..bx {
                color_position(j, i, canvas_width, canvas_height, frame, rgba)
            }
        }
    }

    dy1 = y3 - y2;
    dx1 = x3 - x2;

    if dy1 != 0 {
        dax_step = dx1 as f32 / dy1.abs() as f32;
    }
    if dy2 != 0 {
        dbx_step = dx2 as f32 / dy2.abs() as f32;
    }

    if dy1 != 0 {
        for i in y2..=y3 {
            let mut ax = x2 + ((i - y2) as f32 * dax_step) as i32;
            let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

            if ax > bx {
                mem::swap(&mut ax, &mut bx);
            }

            for j in ax..bx {
                color_position(j, i, canvas_width, canvas_height, frame, rgba)
            }
        }
    }
}

fn color_position(
    x: i32,
    y: i32,
    canvas_width: i32,
    canvas_height: i32,
    frame: &mut [u8],
    rgba: &[u8],
) {
    if (x < 0) || (y < 0) || (x >= canvas_width) || (y >= canvas_height) {
        return;
    }
    let index = get_starting_pixel_index(x, y, canvas_width);
    let pixel = &mut frame[index..index + 4];
    pixel.copy_from_slice(rgba);
}

fn get_starting_pixel_index(x: i32, y: i32, canvas_width: i32) -> usize {
    (((y * canvas_width) + (x)) * 4) as usize
}
