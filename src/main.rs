use std::{
    time::{Duration, Instant},
    vec,
};

use engine_3d::{
    get_color,
    mat4x4::{
        make_projection, make_rotation_x, make_rotation_y, make_rotation_z, make_translation,
        multiply_matrix, multiply_vector, point_at, quick_inverse, Mat4x4,
    },
    mesh::Mesh,
    textured_triangle,
    triangle::Triangle,
    vec3d::{clip_against_plane, cross_product, dot_product, Vec3D},
};
use image::{io::Reader, DynamicImage};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 240;
const SCALE: i32 = 4;

const SPEED: f64 = 16.0;

struct Engine3D {
    elapsed_time: Duration,
    theta: f64,

    mesh_cube: Mesh,
    mat_proj: Mat4x4,

    camera: Vec3D,
    look_dir: Vec3D,

    yaw: f64,

    spr_tex: DynamicImage,
}

impl Engine3D {
    fn new() -> Self {
        let mesh_cube = Mesh::from_file("models/spyro_level.obj", true);
        let spr_tex = Reader::open("textures/spyro_high.png")
            .unwrap()
            .decode()
            .unwrap();

        let mat_proj = make_projection(90.0, HEIGHT as f64 / WIDTH as f64, 0.1, 1000.0);

        Self {
            elapsed_time: Duration::new(0, 0),
            theta: 0.0,
            mesh_cube,
            mat_proj,
            camera: Vec3D::empty(),
            look_dir: Vec3D::empty(),
            yaw: 0.0,
            spr_tex,
        }
    }

    fn update(&mut self, input: &WinitInputHelper) -> Vec<Triangle> {
        let elapsed_time = self.elapsed_time.as_secs_f64();

        if input.key_held(VirtualKeyCode::Up) || input.key_held(VirtualKeyCode::Space) {
            self.camera.y += SPEED * elapsed_time;
        }
        if input.key_held(VirtualKeyCode::Down) || input.held_shift() {
            self.camera.y -= SPEED * elapsed_time;
        }

        // Removed b/c this makes no sense in first person cam
        // if input.key_held(VirtualKeyCode::Left) {
        //     self.camera.x += SPEED * elapsed_time;
        // }
        // if input.key_held(VirtualKeyCode::Right) {
        //     self.camera.x -= SPEED * elapsed_time;
        // }

        let forward = &self.look_dir * (SPEED * elapsed_time);

        if input.key_held(VirtualKeyCode::W) {
            self.camera = &self.camera + &forward;
        }
        if input.key_held(VirtualKeyCode::S) {
            self.camera = &self.camera - &forward;
        }

        if input.key_held(VirtualKeyCode::A) {
            self.yaw -= 2.0 * elapsed_time;
        }
        if input.key_held(VirtualKeyCode::D) {
            self.yaw += 2.0 * elapsed_time;
        }

        // self.theta += 1.0 * self.elapsed_time.as_secs_f64();
        let mat_rot_z = make_rotation_z(self.theta * 0.5);
        let mat_rot_x = make_rotation_x(self.theta);

        let mat_trans = make_translation(0.0, 0.0, 5.0);

        let mut mat_world = multiply_matrix(&mat_rot_z, &mat_rot_x);
        mat_world = multiply_matrix(&mat_world, &mat_trans);

        let up = Vec3D::new(0.0, 1.0, 0.0);
        let mut target = Vec3D::new(0.0, 0.0, 1.0);
        let mat_camera_rot = make_rotation_y(self.yaw);
        self.look_dir = multiply_vector(&mat_camera_rot, &target);
        target = &self.camera + &self.look_dir;

        let mat_camera = point_at(&self.camera, &target, &up);

        // Make view matrix from camera
        let mat_view = quick_inverse(&mat_camera);

        // Store triangles for rastering later
        let mut tris_to_raster = vec![];

        // Draw Triangles
        for tri in &self.mesh_cube.tris {
            let tri_transformed = Triangle::new_uv(
                multiply_vector(&mat_world, &tri.p[0]),
                multiply_vector(&mat_world, &tri.p[1]),
                multiply_vector(&mat_world, &tri.p[2]),
                tri.t[0],
                tri.t[1],
                tri.t[2],
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

                // Convert world space --> view space
                let mut tri_viewed = Triangle::new_uv(
                    multiply_vector(&mat_view, &tri_transformed.p[0]),
                    multiply_vector(&mat_view, &tri_transformed.p[1]),
                    multiply_vector(&mat_view, &tri_transformed.p[2]),
                    tri_transformed.t[0],
                    tri_transformed.t[1],
                    tri_transformed.t[2],
                );
                tri_viewed.col = c;

                // Clip viewed triangle against near plane, this could form two additional
                // triangles
                let (clipped_triangles, clipped) = clip_against_plane(
                    Vec3D::new(0.0, 0.0, 0.1),
                    Vec3D::new(0.0, 0.0, 1.0),
                    &tri_viewed,
                );

                for n in 0..clipped_triangles {
                    // Project triangles from 3D --> 2D
                    let mut tri_projected = Triangle::new_uv(
                        multiply_vector(&self.mat_proj, &clipped[n].p[0]),
                        multiply_vector(&self.mat_proj, &clipped[n].p[1]),
                        multiply_vector(&self.mat_proj, &clipped[n].p[2]),
                        clipped[n].t[0],
                        clipped[n].t[1],
                        clipped[n].t[2],
                    );
                    tri_projected.col = clipped[n].col;

                    tri_projected.t[0].u /= tri_projected.p[0].w;
                    tri_projected.t[1].u /= tri_projected.p[1].w;
                    tri_projected.t[2].u /= tri_projected.p[2].w;

                    tri_projected.t[0].v /= tri_projected.p[0].w;
                    tri_projected.t[1].v /= tri_projected.p[1].w;
                    tri_projected.t[2].v /= tri_projected.p[2].w;

                    tri_projected.t[0].w = 1.0 / tri_projected.p[0].w;
                    tri_projected.t[1].w = 1.0 / tri_projected.p[1].w;
                    tri_projected.t[2].w = 1.0 / tri_projected.p[2].w;

                    tri_projected.p[0] = &tri_projected.p[0] / tri_projected.p[0].w;
                    tri_projected.p[1] = &tri_projected.p[1] / tri_projected.p[1].w;
                    tri_projected.p[2] = &tri_projected.p[2] / tri_projected.p[2].w;

                    // X/Y are inverted so put them back
                    tri_projected.p[0].x *= -1.0;
                    tri_projected.p[1].x *= -1.0;
                    tri_projected.p[2].x *= -1.0;
                    tri_projected.p[0].y *= -1.0;
                    tri_projected.p[1].y *= -1.0;
                    tri_projected.p[2].y *= -1.0;

                    // Offset verts into visible normalised space
                    let offset_view = Vec3D::new(1.0, 1.0, 0.0);
                    tri_projected.p[0] = &tri_projected.p[0] + &offset_view;
                    tri_projected.p[1] = &tri_projected.p[1] + &offset_view;
                    tri_projected.p[2] = &tri_projected.p[2] + &offset_view;
                    tri_projected.p[0].x *= 0.5 * WIDTH as f64;
                    tri_projected.p[0].y *= 0.5 * HEIGHT as f64;
                    tri_projected.p[1].x *= 0.5 * WIDTH as f64;
                    tri_projected.p[1].y *= 0.5 * HEIGHT as f64;
                    tri_projected.p[2].x *= 0.5 * WIDTH as f64;
                    tri_projected.p[2].y *= 0.5 * HEIGHT as f64;

                    // Store triangles for sorting
                    tris_to_raster.push(tri_projected);
                }
            }
        }

        // Sort triangles from back to front
        // tris_to_raster.sort_by(|t1, t2| {
        //     let z1 = (t1.p[0].z + t1.p[1].z + t1.p[2].z) / 3.0;
        //     let z2 = (t2.p[0].z + t2.p[1].z + t2.p[2].z) / 3.0;
        //     z2.partial_cmp(&z1).unwrap()
        // });

        tris_to_raster
    }

    fn draw(&self, frame: &mut [u8], tris_to_raster: Vec<Triangle>) {
        // Clear screen
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[107, 229, 252, 0xff]);
        }

        let mut depth_buffer = [0.0; (WIDTH * HEIGHT) as usize];

        for tri_to_raster in tris_to_raster {
            // Clip triangles against all four screen edges, this could yield
            // a bunch of triangles

            // Add initial triangle
            let mut list_triangles = vec![tri_to_raster];
            let mut new_triangles = 1;

            for p in 0..4 {
                while new_triangles > 0 {
                    // Take triangles from front of queue
                    let test = list_triangles.remove(0);
                    new_triangles -= 1;

                    // Clip it against a plane. We only need to test each
                    // subsequent plane, against subsequent new triangles
                    // as all triangles after a plane clip are guaranteed
                    // to lie on the inside of the plane. I like how this
                    // comment is almost completely and utterly justified
                    let (tris_to_add, clipped) = match p {
                        0 => clip_against_plane(
                            Vec3D::new(0.0, 0.0, 0.0),
                            Vec3D::new(0.0, 1.0, 0.0),
                            &test,
                        ),
                        1 => clip_against_plane(
                            Vec3D::new(0.0, HEIGHT as f64 - 1.0, 0.0),
                            Vec3D::new(0.0, -1.0, 0.0),
                            &test,
                        ),
                        2 => clip_against_plane(
                            Vec3D::new(0.0, 0.0, 0.0),
                            Vec3D::new(1.0, 0.0, 0.0),
                            &test,
                        ),
                        3 => clip_against_plane(
                            Vec3D::new(WIDTH as f64 - 1.0, 0.0, 0.0),
                            Vec3D::new(-1.0, 0.0, 0.0),
                            &test,
                        ),
                        _ => (0, [Triangle::empty(); 2]),
                    };

                    // Clipping may yield a variable number of triangles, so
                    // add these new ones to the back of the queue for subsequent
                    // clipping against next planes
                    for w in 0..tris_to_add {
                        list_triangles.push(clipped[w]);
                    }
                }

                new_triangles = list_triangles.len();
            }

            // Draw triangles
            for t in list_triangles {
                // fill_triangle(frame, WIDTH, &t);
                textured_triangle(frame, WIDTH, &t, &self.spr_tex, &mut depth_buffer);
                // draw_triangle(frame, WIDTH, &t, &[255, 255, 255, 0xff]);
            }
        }

        // Draw depth buffer to screen
        // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //     let c = ((depth_buffer[i] * 4.0).tanh() * 255.0) as u8;
        //     pixel.copy_from_slice(&[c, c, c, 0xff]);
        // }
    }
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

            let tris_to_raster = engine.update(&input);
            engine.draw(pixels.frame_mut(), tris_to_raster);

            if let Err(e) = pixels.render() {
                println!("{}", e);
                control_flow.set_exit();
            }

            engine.elapsed_time = last_frame_time.elapsed();
            last_frame_time = Instant::now();

            let fps = 1.0 / engine.elapsed_time.as_secs_f64();
            window.set_title(&format!("Engine 3D - FPS: {:.0}", fps));
        }
    })
}
