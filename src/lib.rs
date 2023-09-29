use std::mem;

use image::{DynamicImage, GenericImageView};
use triangle::Triangle;

pub mod mat4x4;
pub mod mesh;
pub mod triangle;
pub mod vec2d;
pub mod vec3d;

pub fn get_color(lum: f64) -> [u8; 4] {
    let r = (lum * 255.0) as u8;
    let g = (lum * 255.0) as u8;
    let b = (lum * 255.0) as u8;
    [r, g, b, 0xff]
}

pub fn textured_triangle(frame: &mut [u8], canvas_width: i32, tri: &Triangle, tex: &DynamicImage, depth_buffer: &mut [f64]) {
    let mut x1 = tri.p[0].x as i32;
    let mut y1 = tri.p[0].y as i32;
    let mut x2 = tri.p[1].x as i32;
    let mut y2 = tri.p[1].y as i32;
    let mut x3 = tri.p[2].x as i32;
    let mut y3 = tri.p[2].y as i32;

    let mut u1 = tri.t[0].u;
    let mut v1 = tri.t[0].v;
    let mut u2 = tri.t[1].u;
    let mut v2 = tri.t[1].v;
    let mut u3 = tri.t[2].u;
    let mut v3 = tri.t[2].v;

    let mut w1 = tri.t[0].w;
    let mut w2 = tri.t[1].w;
    let mut w3 = tri.t[2].w;

    let tex_width = (tex.width() - 1) as f64;
    let tex_height = (tex.height() - 1) as f64;

    let canvas_height = frame.len() as i32 / 4 / canvas_width;

    if y1 > y2 {
        mem::swap(&mut y1, &mut y2);
        mem::swap(&mut x1, &mut x2);
        mem::swap(&mut u1, &mut u2);
        mem::swap(&mut v1, &mut v2);
        mem::swap(&mut w1, &mut w2);
    }
    if y1 > y3 {
        mem::swap(&mut y1, &mut y3);
        mem::swap(&mut x1, &mut x3);
        mem::swap(&mut u1, &mut u3);
        mem::swap(&mut v1, &mut v3);
        mem::swap(&mut w1, &mut w3);
    }
    if y2 > y3 {
        mem::swap(&mut y2, &mut y3);
        mem::swap(&mut x2, &mut x3);
        mem::swap(&mut u2, &mut u3);
        mem::swap(&mut v2, &mut v3);
        mem::swap(&mut w2, &mut w3);
    }

    let mut dy1 = y2 - y1;
    let mut dx1 = x2 - x1;
    let mut dv1 = v2 - v1;
    let mut du1 = u2 - u1;
    let mut dw1 = w2 - w1;

    let dy2 = y3 - y1;
    let dx2 = x3 - x1;
    let dv2 = v3 - v1;
    let du2 = u3 - u1;
    let dw2 = w3 - w1;

    // let mut tex_u;
    // let mut tex_v;

    let mut dax_step = 0.0;
    let mut dbx_step = 0.0;
    let mut du1_step = 0.0;
    let mut dv1_step = 0.0;
    let mut du2_step = 0.0;
    let mut dv2_step = 0.0;
    let mut dw1_step = 0.0;
    let mut dw2_step = 0.0;

    if dy1 != 0 {
        dax_step = dx1 as f64 / dy1.abs() as f64;
    }
    if dy2 != 0 {
        dbx_step = dx2 as f64 / dy2.abs() as f64;
    }

    if dy1 != 0 {
        du1_step = du1 / dy1.abs() as f64;
    }
    if dy1 != 0 {
        dv1_step = dv1 / dy1.abs() as f64;
    }
    if dy1 != 0 {
        dw1_step = dw1 / dy1.abs() as f64;
    }

    if dy2 != 0 {
        du2_step = du2 / dy2.abs() as f64;
    }
    if dy2 != 0 {
        dv2_step = dv2 / dy2.abs() as f64;
    }
    if dy2 != 0 {
        dw2_step = dw2 / dy2.abs() as f64;
    }

    if dy1 != 0 {
        for i in y1..=y2 {
            let mut ax = (x1 as f64 + (i - y1) as f64 * dax_step) as i32;
            let mut bx = (x1 as f64 + (i - y1) as f64 * dbx_step) as i32;

            let mut tex_su = u1 + (i - y1) as f64 * du1_step;
            let mut tex_sv = v1 + (i - y1) as f64 * dv1_step;
            let mut tex_sw = w1 + (i - y1) as f64 * dw1_step;

            let mut tex_eu = u1 + (i - y1) as f64 * du2_step;
            let mut tex_ev = v1 + (i - y1) as f64 * dv2_step;
            let mut tex_ew = w1 + (i - y1) as f64 * dw2_step;

            if ax > bx {
                mem::swap(&mut ax, &mut bx);
                mem::swap(&mut tex_su, &mut tex_eu);
                mem::swap(&mut tex_sv, &mut tex_ev);
                mem::swap(&mut tex_sw, &mut tex_ew);
            }

            // tex_u = tex_su;
            // tex_v = tex_sv;

            let t_step = 1.0 / (bx - ax) as f64;
            let mut t = 0.0;

            for j in ax..bx {
                let tex_u = (1.0 - t) * tex_su + t * tex_eu;
                let tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                let tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                if tex_w > depth_buffer[(i * canvas_width + j) as usize] {
                    let rgba = tex
                        .get_pixel(
                            (tex_u / tex_w * tex_width) as u32,
                            (tex_v / tex_w * tex_height) as u32 as u32,
                        )
                        .0;
                    color_position(j, i, canvas_width, canvas_height, frame, &rgba);
                    depth_buffer[(i * canvas_width + j) as usize] = tex_w;
                }
                
                t += t_step;
            }
        }
    }

    dy1 = y3 - y2;
    dx1 = x3 - x2;
    dv1 = v3 - v2;
    du1 = u3 - u2;
    dw1 = w3 - w2;

    if dy1 != 0 {
        dax_step = dx1 as f64 / dy1.abs() as f64;
    }
    if dy2 != 0 {
        dbx_step = dx2 as f64 / dy2.abs() as f64;
    }

    if dy1 != 0 {
        du1_step = du1 / dy1.abs() as f64;
    }
    if dy1 != 0 {
        dv1_step = dv1 / dy1.abs() as f64;
    }
    if dy1 != 0 {
        dw1_step = dw1 / dy1.abs() as f64;
    }

    if dy1 != 0 {
        for i in y2..=y3 {
            let mut ax = (x2 as f64 + (i - y2) as f64 * dax_step) as i32;
            let mut bx = (x1 as f64 + (i - y1) as f64 * dbx_step) as i32;

            let mut tex_su = u2 + (i - y2) as f64 * du1_step;
            let mut tex_sv = v2 + (i - y2) as f64 * dv1_step;
            let mut tex_sw = w2 + (i - y2) as f64 * dw1_step;

            let mut tex_eu = u1 + (i - y1) as f64 * du2_step;
            let mut tex_ev = v1 + (i - y1) as f64 * dv2_step;
            let mut tex_ew = w1 + (i - y1) as f64 * dw2_step;

            if ax > bx {
                mem::swap(&mut ax, &mut bx);
                mem::swap(&mut tex_su, &mut tex_eu);
                mem::swap(&mut tex_sv, &mut tex_ev);
                mem::swap(&mut tex_sw, &mut tex_ew);
            }

            // tex_u = tex_su;
            // tex_v = tex_sv;

            let t_step = 1.0 / (bx - ax) as f64;
            let mut t = 0.0;

            for j in ax..bx {
                let tex_u = (1.0 - t) * tex_su + t * tex_eu;
                let tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                let tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                if tex_w > depth_buffer[(i * canvas_width + j) as usize] {
                    let rgba = tex
                        .get_pixel(
                            (tex_u / tex_w * tex_width) as u32,
                            (tex_v / tex_w * tex_height) as u32,
                        )
                        .0;
                    color_position(j, i, canvas_width, canvas_height, frame, &rgba);
                    depth_buffer[(i * canvas_width + j) as usize] = tex_w;
                }

                t += t_step;
            }
        }
    }
}

pub fn draw_triangle(frame: &mut [u8], canvas_width: i32, tri: &Triangle, col: &[u8; 4]) {
    pixels_primitives::triangle(
        frame,
        canvas_width,
        tri.p[0].x as i32,
        tri.p[0].y as i32,
        tri.p[1].x as i32,
        tri.p[1].y as i32,
        tri.p[2].x as i32,
        tri.p[2].y as i32,
        col,
    );
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
