use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{triangle::Triangle, vec3d::Vec3D};

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tri_list: Vec<[f32; 9]>) -> Self {
        let mut tris = vec![];
        for tri in tri_list {
            tris.push(Triangle::new(
                Vec3D::new(tri[0], tri[1], tri[2]),
                Vec3D::new(tri[3], tri[4], tri[5]),
                Vec3D::new(tri[6], tri[7], tri[8]),
            ));
        }
        Self { tris }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut vecs: Vec<Vec3D> = vec![];
        let mut tris: Vec<Triangle> = vec![];

        let file = File::open(filename).expect("Error opening file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_ascii_whitespace();
            if let Some(c) = line.next() {
                match c {
                    "v" => {
                        let nums = line
                            .map(|n| n.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>();
                        let vec = Vec3D::new(nums[0], nums[1], nums[2]);
                        vecs.push(vec);
                    }
                    "f" => {
                        let nums = line
                            .map(|n| n.parse::<usize>().unwrap() - 1)
                            .collect::<Vec<usize>>();
                        let tri = Triangle::new(
                            vecs[nums[0]].clone(),
                            vecs[nums[1]].clone(),
                            vecs[nums[2]].clone(),
                        );
                        tris.push(tri);
                    }
                    _ => {}
                }
            }
        }

        Self { tris }
    }
}
