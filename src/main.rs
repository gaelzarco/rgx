/*
 * =============================================================================
 * Project: Tiny Renderer Built in Rust
 * Author: Gael Zarco
 * Date: December 1st, 2024
 * License: IDK
 * Description:
 * Built using the following guide: https://github.com/ssloy/tinyrenderer/wiki
 * =============================================================================
*/

pub mod geometry;

use minifb::{Window, WindowOptions};

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;

/// RGB color
///
/// Takes in r, g, and b values as 8-bit and spits out a 32-bit color integer
///
/// Utilizes bit-wise operations to amalgamate a final color value
fn u8_rgb_color(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

/// Bresenham Line
///
/// Takes in x0, y0, x1, and y1 values respectively.
/// These values are used draw a line of a specified color on a canvas buffer
///
/// The canvas buffer consists of a vector of pixels which length equals the total sum of the width x height
///
/// The width and height vars are the width and height of the canvas
fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    canvas_buf: &mut Vec<u32>,
    width: usize,
    height: usize,
    color: u32,
) {
    // Flip Y-axix to match screen space
    y0 = height as i32 - y0;
    y1 = height as i32 - y1;

    let mut steep = false;

    // If the line in steep, we transpose the image
    if (x0 - x1).abs() < (y0 - y1).abs() {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }

    // Make line left to right
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let mut x = x0; // Start point
    let mut y = y0; // Start point
    let dx = x1 - x0; // Total deviation of x
    let dy = y1 - y0; // Total deviation of y
    let derror2 = (dy.abs()) * 2; // Change in Y incremental integer operation. Multiplies dy by 2 to avoid float operations
    let mut error2 = 0; // Initialize error accumulator

    // Loop until x is greater than x1
    while x <= x1 {
        // Convert coordinates to buffer index based on if line is transposed
        let (draw_x, draw_y) = if steep {
            (y as usize, x as usize)
        } else {
            (x as usize, y as usize)
        };

        // Ensure coordinates are within bounds and draw point
        if draw_x < width && draw_y < height {
            let idx = draw_y * width + draw_x;
            canvas_buf[idx] = color;
        } else {
            println!(
                "Coordinates (x:{}, y:{}) are not within canvas bounds",
                draw_x, draw_y
            );
        }

        error2 += derror2; // Increment accumulator by dy incremental integer
                           // If accumulator is greater than change in x, find closest Y coordinate
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            // Reset accumulator
            error2 -= dx * 2;
        }

        // Continue loop
        x += 1;
    }
}

fn main() {
    let mut window = match Window::new(
        "rtgx",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("unable to create window {}", err);
            return;
        }
    };

    window.set_target_fps(120);
    window.topmost(true);

    let mut canvas_buf = vec![0; WIDTH * HEIGHT];

    use geometry::triangle;
    use geometry::Coordinate;

    triangle(
        Coordinate(10, 70),
        Coordinate(50, 160),
        Coordinate(70, 80),
        &mut canvas_buf,
        WIDTH,
        HEIGHT,
        u8_rgb_color(255, 255, 255),
    );
    triangle(
        Coordinate(180, 50),
        Coordinate(150, 1),
        Coordinate(70, 180),
        &mut canvas_buf,
        WIDTH,
        HEIGHT,
        u8_rgb_color(255, 255, 255),
    );
    triangle(
        Coordinate(180, 150),
        Coordinate(120, 160),
        Coordinate(130, 180),
        &mut canvas_buf,
        WIDTH,
        HEIGHT,
        u8_rgb_color(255, 255, 255),
    );

    // let ( vertices, faces ) = geometry::load_obj("obj/african_head.obj");
    //
    //// Loop over faces matrix
    // for face in faces.iter() {
    //     for j in 0..3 {
    //         // Get index of current face vertex
    //         let v0 = &vertices[face[j]];
    //         // Get index of next face vertex, looping back to 0 after last vertex
    //         let v1 = &vertices[face[(j + 1) % 3]];
    //
    //         let (x0, y0) = geometry::three_to_canvas(v0, WIDTH, HEIGHT);
    //         let (x1, y1) = geometry::three_to_canvas(v1, WIDTH, HEIGHT);
    //
    //         line(x0, y0, x1, y1, &mut canvas_buf, WIDTH, HEIGHT, u8_rgb_color(255, 255, 255));
    //     }
    // }

    while window.is_open() {
        window
            .update_with_buffer(&canvas_buf, WIDTH, HEIGHT)
            .unwrap();
    }
}
