use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use engine_3d::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 240;
const SCALE: i32 = 4;

struct Engine3D {
    elapsed_time: Duration,
    theta: f32,

    mesh_cube: Mesh,
    mat_proj: Mat4x4,

    camera: Vec3D,
}

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(WIDTH * SCALE, HEIGHT * SCALE);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut engine = Engine3D::new();

    let mut last_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                control_flow.set_exit();
            }

            engine.update();
            engine.draw(pixels.frame_mut());

            if let Err(e) = pixels.render() {
                println!("{}", e);
                control_flow.set_exit();
            }

            engine.elapsed_time = last_frame_time.elapsed();
            last_frame_time = Instant::now();

            let fps = 1.0 / engine.elapsed_time.as_secs_f32();
            window.set_title(&format!("Engine 3D - FPS: {:.0}", fps));
        }
    })
}

impl Engine3D {
    fn new() -> Self {
        let mesh_cube = Mesh::new(vec![
            // SOUTH
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0],
            // EAST
            [1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0],
            // NORTH
            [1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
            // WEST
            [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            // TOP
            [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0],
            // BOTTOM
            [1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        ]);

        let near = 0.1;
        let far = 1000.0;
        let fov = 90.0;
        let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * PI).tan();

        let mut mat_proj = Mat4x4::new();
        mat_proj.m[0][0] = aspect_ratio * fov_rad;
        mat_proj.m[1][1] = fov_rad;
        mat_proj.m[2][2] = far / (far - near);
        mat_proj.m[3][2] = (-far * near) / (far - near);
        mat_proj.m[2][3] = 1.0;
        mat_proj.m[3][3] = 0.0;

        let camera = Vec3D::new();

        Self {
            elapsed_time: Duration::new(0, 0),
            theta: 0.0,
            mesh_cube,
            mat_proj,
            camera,
        }
    }

    fn update(&mut self) {
        self.theta += 1.0 * self.elapsed_time.as_secs_f32();
    }

    fn draw(&self, frame: &mut [u8]) {
        frame.fill(0x00);

        let theta = self.theta;
        let half_theta = theta * 0.5;

        let mut mat_rot_z = Mat4x4::new();
        let mut mat_rot_x = Mat4x4::new();

        // Rotation Z
        mat_rot_z.m[0][0] = theta.cos();
        mat_rot_z.m[0][1] = theta.sin();
        mat_rot_z.m[1][0] = -(theta.sin());
        mat_rot_z.m[1][1] = theta.cos();
        mat_rot_z.m[2][2] = 1.0;
        mat_rot_z.m[3][3] = 1.0;

        // Rotation X
        mat_rot_x.m[0][0] = 1.0;
        mat_rot_x.m[1][1] = half_theta.cos();
        mat_rot_x.m[1][2] = half_theta.sin();
        mat_rot_x.m[2][1] = -(half_theta.sin());
        mat_rot_x.m[2][2] = half_theta.cos();
        mat_rot_x.m[3][3] = 1.0;

        // Draw Triangles
        for tri in &self.mesh_cube.tris {
            // Rotate in Z-Axis
            let tri_rotated_z = Triangle {
                p: [
                    multiply_matrix_vector(&tri.p[0], &mat_rot_z),
                    multiply_matrix_vector(&tri.p[1], &mat_rot_z),
                    multiply_matrix_vector(&tri.p[2], &mat_rot_z),
                ],
                col: [0xff, 0xff, 0xff, 0xff],
            };

            // Rotate in X-Axis
            let tri_rotated_zx = Triangle {
                p: [
                    multiply_matrix_vector(&tri_rotated_z.p[0], &mat_rot_x),
                    multiply_matrix_vector(&tri_rotated_z.p[1], &mat_rot_x),
                    multiply_matrix_vector(&tri_rotated_z.p[2], &mat_rot_x),
                ],
                col: [0xff, 0xff, 0xff, 0xff],
            };

            // Offset into the screen
            let mut tri_translated = tri_rotated_zx.clone();
            tri_translated.p[0].z = tri_rotated_zx.p[0].z + 3.0;
            tri_translated.p[1].z = tri_rotated_zx.p[1].z + 3.0;
            tri_translated.p[2].z = tri_rotated_zx.p[2].z + 3.0;

            let line1 = Vec3D {
                x: tri_translated.p[1].x - tri_translated.p[0].x,
                y: tri_translated.p[1].y - tri_translated.p[0].y,
                z: tri_translated.p[1].z - tri_translated.p[0].z,
            };

            let line2 = Vec3D {
                x: tri_translated.p[2].x - tri_translated.p[0].x,
                y: tri_translated.p[2].y - tri_translated.p[0].y,
                z: tri_translated.p[2].z - tri_translated.p[0].z,
            };

            let mut normal = Vec3D {
                x: line1.y * line2.z - line1.z * line2.y,
                y: line1.z * line2.x - line1.x * line2.z,
                z: line1.x * line2.y - line1.y * line2.x,
            };

            let l = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            normal.x /= l;
            normal.y /= l;
            normal.z /= l;

            if normal.x * (tri_translated.p[0].x - self.camera.x)
                + normal.y * (tri_translated.p[0].y - self.camera.y)
                + normal.z * (tri_translated.p[0].z - self.camera.z)
                < 0.0
            {
                // Illumination
                let mut light_direction = Vec3D {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                };
                let l = (light_direction.x * light_direction.x
                    + light_direction.y * light_direction.y
                    + light_direction.z * light_direction.z)
                    .sqrt();
                light_direction.x /= l;
                light_direction.y /= l;
                light_direction.z /= l;

                let dp = normal.x * light_direction.x
                    + normal.y * light_direction.y
                    + normal.z * light_direction.z;

                let c = get_color(dp);
                tri_translated.col = c;

                // Project triangles from 3D --> 2D
                let mut tri_projected = Triangle {
                    p: [
                        multiply_matrix_vector(&tri_translated.p[0], &self.mat_proj),
                        multiply_matrix_vector(&tri_translated.p[1], &self.mat_proj),
                        multiply_matrix_vector(&tri_translated.p[2], &self.mat_proj),
                    ],
                    col: tri_translated.col,
                };

                // Scale into view
                tri_projected.p[0].x += 1.0;
                tri_projected.p[0].y += 1.0;
                tri_projected.p[1].x += 1.0;
                tri_projected.p[1].y += 1.0;
                tri_projected.p[2].x += 1.0;
                tri_projected.p[2].y += 1.0;

                tri_projected.p[0].x *= 0.5 * WIDTH as f32;
                tri_projected.p[0].y *= 0.5 * HEIGHT as f32;
                tri_projected.p[1].x *= 0.5 * WIDTH as f32;
                tri_projected.p[1].y *= 0.5 * HEIGHT as f32;
                tri_projected.p[2].x *= 0.5 * WIDTH as f32;
                tri_projected.p[2].y *= 0.5 * HEIGHT as f32;

                // Rasterize triangles

                draw_triangle(
                    frame,
                    WIDTH,
                    tri_projected.p[0].x as i32,
                    tri_projected.p[0].y as i32,
                    tri_projected.p[1].x as i32,
                    tri_projected.p[1].y as i32,
                    tri_projected.p[2].x as i32,
                    tri_projected.p[2].y as i32,
                    &tri_projected.col,
                );

                // pixels_primitives::triangle(
                //     frame,
                //     WIDTH,
                //     tri_projected.p[0].x as i32,
                //     tri_projected.p[0].y as i32,
                //     tri_projected.p[1].x as i32,
                //     tri_projected.p[1].y as i32,
                //     tri_projected.p[2].x as i32,
                //     tri_projected.p[2].y as i32,
                //     &[0, 0, 0, 0xff],
                // );
            }
        }
    }
}
