use std::mem;

use triangle::Triangle;

pub mod mat4x4;
pub mod mesh;
pub mod triangle;
pub mod vec3d;
pub mod vec2d;

pub fn get_color(lum: f32) -> [u8; 4] {
    let r = (lum * 255.0) as u8;
    let g = (lum * 255.0) as u8;
    let b = (lum * 255.0) as u8;
    [r, g, b, 0xff]
}

pub fn fill_triangle(frame: &mut [u8], canvas_width: i32, tri: &Triangle) {
    let mut x1 = tri.p[0].x as i32;
    let mut y1 = tri.p[0].y as i32;
    let mut x2 = tri.p[1].x as i32;
    let mut y2 = tri.p[1].y as i32;
    let mut x3 = tri.p[2].x as i32;
    let mut y3 = tri.p[2].y as i32;

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
            let mut ax = (x1 as f32 + (i - y1) as f32 * dax_step) as i32;
            let mut bx = (x1 as f32 + (i - y1) as f32 * dbx_step) as i32;

            if ax > bx {
                mem::swap(&mut ax, &mut bx);
            }

            for j in ax..bx {
                color_position(j, i, canvas_width, canvas_height, frame, &tri.col)
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
            let mut ax = (x2 as f32 + (i - y2) as f32 * dax_step) as i32;
            let mut bx = (x1 as f32 + (i - y1) as f32 * dbx_step) as i32;

            if ax > bx {
                mem::swap(&mut ax, &mut bx);
            }

            for j in ax..bx {
                color_position(j, i, canvas_width, canvas_height, frame, &tri.col)
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
