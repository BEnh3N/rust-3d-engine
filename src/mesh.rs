use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{triangle::Triangle, vec2d::Vec2D, vec3d::Vec3D};

// pub CUBE: Mesh = Mesh::new(vec![
//     // SOUTH
//     [
//         0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
//     ],
//     [
//         0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
//     ],
//     // EAST
//     [
//         1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
//     ],
//     [
//         1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
//     ],
//     // NORTH
//     [
//         1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
//     ],
//     [
//         1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
//     ],
//     // WEST
//     [
//         0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
//     ],
//     [
//         0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
//     ],
//     // TOP
//     [
//         0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
//     ],
//     [
//         0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
//     ],
//     // BOTTOM
//     [
//         1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
//     ],
//     [
//         1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
//     ],
// ]);

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tri_list: Vec<[f64; 15]>) -> Self {
        let mut tris = vec![];
        for tri in tri_list {
            tris.push(Triangle::new_uv(
                Vec3D::new(tri[0], tri[1], tri[2]),
                Vec3D::new(tri[3], tri[4], tri[5]),
                Vec3D::new(tri[6], tri[7], tri[8]),
                Vec2D::new(tri[9], tri[10]),
                Vec2D::new(tri[11], tri[12]),
                Vec2D::new(tri[13], tri[14]),
            ));
        }
        Self { tris }
    }

    pub fn from_file(filename: &str, has_tex: bool) -> Self {
        let mut verts: Vec<Vec3D> = vec![];
        let mut texs: Vec<Vec2D> = vec![];
        let mut tris: Vec<Triangle> = vec![];

        let mut x_a = 0.0;
        let mut y_a = 0.0;
        let mut z_a = 0.0;

        let file = File::open(filename).expect("Error opening file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_ascii_whitespace();
            if let Some(c) = line.next() {
                match c {
                    "v" => {
                        let nums = line
                            .map(|n| n.parse::<f64>().unwrap())
                            .collect::<Vec<f64>>();
                        let vert = Vec3D::new(nums[0], nums[1], nums[2]);
                        x_a += nums[0];
                        y_a += nums[1];
                        z_a += nums[2];
                        verts.push(vert);
                    }
                    "vt" => {
                        let nums = line
                            .map(|n| n.parse::<f64>().unwrap())
                            .collect::<Vec<f64>>();
                        let tex = Vec2D::new(nums[0], 1.0 - nums[1]);
                        texs.push(tex);
                    }
                    "f" => {
                        if !has_tex {
                            let nums = line
                                .map(|n| n.parse::<usize>().unwrap() - 1)
                                .collect::<Vec<usize>>();
                            let tri = Triangle::new(verts[nums[0]], verts[nums[1]], verts[nums[2]]);
                            tris.push(tri);
                        } else {
                            let mut vert_indices = vec![];
                            let mut tex_indices = vec![];
                            line.for_each(|p| {
                                let mut pair =
                                    p.split('/').map(|n| n.parse::<usize>().unwrap() - 1);
                                vert_indices.push(pair.next().unwrap());
                                tex_indices.push(pair.next().unwrap());
                            });

                            if vert_indices.len() == 3 {
                                // Normal Triangle
                                let tri = Triangle::new_uv(
                                    verts[vert_indices[0]],
                                    verts[vert_indices[1]],
                                    verts[vert_indices[2]],
                                    texs[tex_indices[0]],
                                    texs[tex_indices[1]],
                                    texs[tex_indices[2]],
                                );
                                tris.push(tri);
                            } else {
                                // Quad
                                let tri1 = Triangle::new_uv(
                                    verts[vert_indices[0]],
                                    verts[vert_indices[1]],
                                    verts[vert_indices[2]],
                                    texs[tex_indices[0]],
                                    texs[tex_indices[1]],
                                    texs[tex_indices[2]],
                                );
                                tris.push(tri1);
                                let tri2 = Triangle::new_uv(
                                    verts[vert_indices[0]],
                                    verts[vert_indices[2]],
                                    verts[vert_indices[3]],
                                    texs[tex_indices[0]],
                                    texs[tex_indices[2]],
                                    texs[tex_indices[3]],
                                );
                                tris.push(tri2);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let x_a = x_a / verts.len() as f64;
        let y_a = y_a / verts.len() as f64;
        let z_a = z_a / verts.len() as f64;

        for tri in &mut tris {
            for vec in &mut tri.p {
                vec.x -= x_a;
                vec.y -= y_a;
                vec.z -= z_a;
            }
        }

        Self { tris }
    }
}
