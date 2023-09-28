use std::{
    time::{Duration, Instant},
    vec,
};

use engine_3d::{
    draw_triangle, get_color,
    mat4x4::{
        make_projection, make_rotation_x, make_rotation_z, make_translation, multiply_matrix,
        multiply_vector, Mat4x4,
    },
    mesh::Mesh,
    triangle::Triangle,
    vec3d::{cross_product, dot_product, Vec3D},
};
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
        // let mesh_cube = Mesh::new(vec![
        //     // SOUTH
        //     [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0],
        //     [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0],
        //     // EAST
        //     [1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
        //     [1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0],
        //     // NORTH
        //     [1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0],
        //     [1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
        //     // WEST
        //     [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        //     [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        //     // TOP
        //     [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        //     [0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0],
        //     // BOTTOM
        //     [1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        //     [1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        // ]);

        let mesh_cube = Mesh::from_file("teapot.obj");

        let mat_proj = make_projection(90.0, HEIGHT as f32 / WIDTH as f32, 0.1, 1000.0);

        let camera = Vec3D::empty();

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

        let mat_rot_z = make_rotation_z(self.theta * 0.5);
        let mat_rot_x = make_rotation_x(self.theta);

        let mat_trans = make_translation(0.0, 0.0, 5.0);

        let mut mat_world = multiply_matrix(&mat_rot_z, &mat_rot_x);
        mat_world = multiply_matrix(&mat_world, &mat_trans);

        let mut triangles_to_raster = vec![];

        // Draw Triangles
        for tri in &self.mesh_cube.tris {
            let mut tri_transformed = Triangle::new(
                multiply_vector(&mat_world, &tri.p[0]),
                multiply_vector(&mat_world, &tri.p[1]),
                multiply_vector(&mat_world, &tri.p[2]),
            );

            // Calculate triangle normal

            // Get lines on either side of triangle
            let line1 = &tri_transformed.p[1] - &tri_transformed.p[0];
            let line2 = &tri_transformed.p[2] - &tri_transformed.p[0];

            // Take cross product of lines to get normal to triangle surface
            let mut normal = cross_product(&line1, &line2);

            // Normalize
            normal = normal.normalise();

            // Get ray from triangle to camera
            let camera_ray = &tri_transformed.p[0] - &self.camera;

            // If ray is aligned with normal, then triangle is visible
            if dot_product(&normal, &camera_ray) < 0.0 {
                // Illumination
                let mut light_direction = Vec3D::new(0.0, 1.0, -1.0);
                light_direction = light_direction.normalise();

                // How "aligned" are light direction and triangle surface normal?
                let dp = dot_product(&light_direction, &normal).max(0.1);

                // Choose colors
                let c = get_color(dp);
                tri_transformed.col = c;

                // Project triangles from 3D --> 2D
                let mut tri_projected = Triangle::new(
                    multiply_vector(&self.mat_proj, &tri_transformed.p[0]),
                    multiply_vector(&self.mat_proj, &tri_transformed.p[1]),
                    multiply_vector(&self.mat_proj, &tri_transformed.p[2]),
                );
                tri_projected.col = tri_transformed.col;

                tri_projected.p[0] = &tri_projected.p[0] / tri_projected.p[0].w;
                tri_projected.p[1] = &tri_projected.p[1] / tri_projected.p[1].w;
                tri_projected.p[2] = &tri_projected.p[2] / tri_projected.p[2].w;

                // Offset verts into visible normalised space
                let offset_view = Vec3D::new(1.0, 1.0, 0.0);
                tri_projected.p[0] = &tri_projected.p[0] + &offset_view;
                tri_projected.p[1] = &tri_projected.p[1] + &offset_view;
                tri_projected.p[2] = &tri_projected.p[2] + &offset_view;

                tri_projected.p[0].x *= 0.5 * WIDTH as f32;
                tri_projected.p[0].y *= 0.5 * HEIGHT as f32;
                tri_projected.p[1].x *= 0.5 * WIDTH as f32;
                tri_projected.p[1].y *= 0.5 * HEIGHT as f32;
                tri_projected.p[2].x *= 0.5 * WIDTH as f32;
                tri_projected.p[2].y *= 0.5 * HEIGHT as f32;

                // Store triangles for sorting
                triangles_to_raster.push(tri_projected);
            }
        }

        // Sort triangles from back to front
        triangles_to_raster.sort_by(|t1, t2| {
            let z1 = (t1.p[0].z + t1.p[1].z + t1.p[2].z) / 3.0;
            let z2 = (t2.p[0].z + t2.p[1].z + t2.p[2].z) / 3.0;
            z2.partial_cmp(&z1).unwrap()
        });

        for tri_projected in triangles_to_raster {
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
