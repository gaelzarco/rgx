/*
 * =============================================================================
 * Project: Tiny Renderer Built in Rust
 * Author: Gael Zarco
 * Date: December 1st, 2024
 * Description:
 * Built using the tiny renderer guide: https://github.com/ssloy/tinyrenderer/wiki
 * =============================================================================
*/

pub mod geometry;
use geometry::Point;
use minifb::{Window, WindowOptions};
use rand::Rng;

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

// Function to generate a random color (0xRRGGBB format)
fn random_color() -> u32 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    (r << 16) | (g << 8) | b // Combine into 0xRRGGBB format
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

    let (vertices, faces) = geometry::load_obj("obj/african_head.obj");

    // Loop over faces matrix
    for face in faces.iter() {
        // Extract the 3 vertices of the current triangle
        let p0 = &vertices[face[0]];
        let p1 = &vertices[face[1]];
        let p2 = &vertices[face[2]];

        // Convert 3D coordinates to 2D screen space
        let (x0, y0) = geometry::three_to_canvas(p0, WIDTH, HEIGHT);
        let (x1, y1) = geometry::three_to_canvas(p1, WIDTH, HEIGHT);
        let (x2, y2) = geometry::three_to_canvas(p2, WIDTH, HEIGHT);

        // Prepare the vertices in 2D space for the triangle function
        let triangle_pts = [
            Point::new(x0, y0, 0.0),
            Point::new(x1, y1, 0.0),
            Point::new(x2, y2, 0.0),
        ];

        // Assign a random color for the triangle
        let color = random_color();

        // Call the triangle function to render the triangle
        geometry::triangle(&triangle_pts, &mut canvas_buf, WIDTH, HEIGHT, color);
    }

    while window.is_open() {
        window
            .update_with_buffer(&canvas_buf, WIDTH, HEIGHT)
            .unwrap();
    }
}
