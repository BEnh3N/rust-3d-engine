use crate::{triangle::Triangle, vec2d::Vec2D};

#[derive(Clone, Copy, Debug)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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

pub fn dot_product(v1: &Vec3D, v2: &Vec3D) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn length(v: &Vec3D) -> f64 {
    dot_product(v, v).sqrt()
}

pub fn cross_product(v1: &Vec3D, v2: &Vec3D) -> Vec3D {
    Vec3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
        w: 0.0,
    }
}

pub fn intersect_plane(
    plane_p: &Vec3D,
    plane_n: &Vec3D,
    line_start: &Vec3D,
    line_end: &Vec3D,
    t: &mut f64,
) -> Vec3D {
    let plane_n = plane_n.normalise();
    let plane_d = -dot_product(&plane_n, plane_p);
    let ad = dot_product(line_start, &plane_n);
    let bd = dot_product(line_end, &plane_n);
    *t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = &line_start_to_end * *t;
    line_start + &line_to_intersect
}

pub fn clip_against_plane(
    plane_p: Vec3D,
    plane_n: Vec3D,
    tri: &Triangle,
) -> (usize, [Triangle; 2]) {
    // Make sure plane is indeed normal
    let plane_n = plane_n.normalise();

    // Return signed shortest distance from point to plane, plane normal must be normalised
    let dist = |p: &Vec3D| -> f64 {
        plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - dot_product(&plane_n, &plane_p)
    };

    // Create two temporary storage arrays to classify points either side of plane
    // If distance sign is positive, point lines on "inside" of plane
    let mut inside_points = [Vec3D::empty(); 3];
    let mut inside_point_count = 0;
    let mut outside_points = [Vec3D::empty(); 3];
    let mut outside_point_count = 0;
    let mut inside_tex = [Vec2D::empty(); 3];
    let mut inside_tex_count = 0;
    let mut outside_tex = [Vec2D::empty(); 3];
    let mut outside_tex_count = 0;

    // Get signed distance of each point in triangle to plane
    let d0 = dist(&tri.p[0]);
    let d1 = dist(&tri.p[1]);
    let d2 = dist(&tri.p[2]);

    if d0 >= 0.0 {
        inside_points[inside_point_count] = tri.p[0];
        inside_point_count += 1;
        inside_tex[inside_tex_count] = tri.t[0];
        inside_tex_count += 1;
    } else {
        outside_points[outside_point_count] = tri.p[0];
        outside_point_count += 1;
        outside_tex[outside_tex_count] = tri.t[0];
        outside_tex_count += 1;
    }

    if d1 >= 0.0 {
        inside_points[inside_point_count] = tri.p[1];
        inside_point_count += 1;
        inside_tex[inside_tex_count] = tri.t[1];
        inside_tex_count += 1;
    } else {
        outside_points[outside_point_count] = tri.p[1];
        outside_point_count += 1;
        outside_tex[outside_tex_count] = tri.t[1];
        outside_tex_count += 1;
    }

    if d2 >= 0.0 {
        inside_points[inside_point_count] = tri.p[2];
        inside_point_count += 1;
        inside_tex[inside_tex_count] = tri.t[2];
        // inside_tex_count += 1;
    } else {
        outside_points[outside_point_count] = tri.p[2];
        outside_point_count += 1;
        outside_tex[outside_tex_count] = tri.t[2];
        // outside_tex_count += 1;
    }

    // Now classify triangle points, and break the input triangle into
    // smaller output triangles if required. There are four possible
    // outcomes...
    if inside_point_count == 0 {
        // All points lie on the outside of plane, so clip whole triangle
        // It ceases to exist

        (0, [Triangle::empty(), Triangle::empty()])
    } else if inside_point_count == 3 {
        // All points lie on the inside of plane, so do nothing
        // and allow triangle to simply pass through

        (1, [*tri, Triangle::empty()])
    } else if inside_point_count == 1 && outside_point_count == 2 {
        // Triangle should be clipped. As two points lie outside
        // the plane, the triangle simply becomes a smaller triangle

        // Copy appearence info to new triangle
        let mut out_tri = Triangle::empty();
        out_tri.col = tri.col;

        // The inside point is valid, so keep that...
        out_tri.p[0] = inside_points[0];
        out_tri.t[0] = inside_tex[0];

        // but the two new points are at locations where the
        // original sides of the triangle (lines) intersect with the plane
        let mut t = 0.0;
        out_tri.p[1] = intersect_plane(
            &plane_p,
            &plane_n,
            &inside_points[0],
            &outside_points[0],
            &mut t,
        );
        out_tri.t[1].u = t * (outside_tex[0].u - inside_tex[0].u) + inside_tex[0].u;
        out_tri.t[1].v = t * (outside_tex[0].v - inside_tex[0].v) + inside_tex[0].v;
        out_tri.t[1].w = t * (outside_tex[0].w - inside_tex[0].w) + inside_tex[0].w;

        out_tri.p[2] = intersect_plane(
            &plane_p,
            &plane_n,
            &inside_points[0],
            &outside_points[1],
            &mut t,
        );

        out_tri.t[2].u = t * (outside_tex[1].u - inside_tex[0].u) + inside_tex[0].u;
        out_tri.t[2].v = t * (outside_tex[1].v - inside_tex[0].v) + inside_tex[0].v;
        out_tri.t[2].w = t * (outside_tex[1].w - inside_tex[0].w) + inside_tex[0].w;

        (1, [out_tri, Triangle::empty()])
    } else {
        // Triangle should be clipped. As two points lie inside the plane,
        // the clipped triangle becomes a "quad". Fortunately, we can
        // represent a quad with two triangles

        // Copy appearence info to new triangles
        let mut out_tri1 = Triangle::empty();
        let mut out_tri2 = Triangle::empty();
        out_tri1.col = tri.col;
        out_tri2.col = tri.col;

        // The first triangle consists of the two inside points and a new
        // point determined by the location where one side of the triangle
        // intersects with the plane
        out_tri1.p[0] = inside_points[0];
        out_tri1.p[1] = inside_points[1];
        out_tri1.t[0] = inside_tex[0];
        out_tri1.t[1] = inside_tex[1];

        let mut t = 0.0;
        out_tri1.p[2] = intersect_plane(
            &plane_p,
            &plane_n,
            &inside_points[0],
            &outside_points[0],
            &mut t,
        );
        out_tri1.t[2].u = t * (outside_tex[0].u - inside_tex[0].u) + inside_tex[0].u;
        out_tri1.t[2].v = t * (outside_tex[0].v - inside_tex[0].v) + inside_tex[0].v;
        out_tri1.t[2].w = t * (outside_tex[0].w - inside_tex[0].w) + inside_tex[0].w;

        // The second triangle is composed of one of the inside points, a
        // new point determined by the intersection of the other side of the
        // triangle and the plane, and the newly created point above
        out_tri2.p[0] = inside_points[1];
        out_tri2.t[0] = inside_tex[1];
        out_tri2.p[1] = out_tri1.p[2];
        out_tri2.t[1] = out_tri1.t[2];
        out_tri2.p[2] = intersect_plane(
            &plane_p,
            &plane_n,
            &inside_points[1],
            &outside_points[0],
            &mut t,
        );
        out_tri2.t[2].u = t * (outside_tex[0].u - inside_tex[1].u) + inside_tex[1].u;
        out_tri2.t[2].v = t * (outside_tex[0].v - inside_tex[1].v) + inside_tex[1].v;
        out_tri2.t[2].w = t * (outside_tex[0].w - inside_tex[1].w) + inside_tex[1].w;

        (2, [out_tri1, out_tri2]) // Return two newly formed triangles which form a quad
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

impl std::ops::Mul<f64> for &Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 0.0,
        }
    }
}

impl std::ops::Div<f64> for &Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 0.0,
        }
    }
}
