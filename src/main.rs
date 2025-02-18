/*
 * =============================================================================
 * Project: Software Rasterizer
 * Author: Gael Zarco
 * Date: December 1st, 2024
 * Description: Translates pixel and triangle face coordinates to 2D image
 *              implementing complex mathematical algorithms and perforamnce
 *              optimizations such as back-face culling
 * Project Based On: https://github.com/ssloy/tinyrenderer/wiki
 * =============================================================================
*/

pub mod color;
pub mod geometry;
pub mod obj;

use minifb::{Window, WindowOptions};

// CANVAS CONSTANTS
const WIDTH: usize = 720;
const HEIGHT: usize = 720;

fn main() {
    /* ======SCENE=============================================================== */
    // Create window with default options
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
    // Additional window options
    window.set_target_fps(120);
    window.topmost(true);

    // Canvas buffer for rasterization
    let mut canvas_buf = vec![0; WIDTH * HEIGHT];
    // Establish light direction
    let light_dir: [f32; 3] = [0.0, 0.0, -1.0];

    /* ======PROCESSING========================================================== */
    // Load in face and vertex data
    let (vertices, faces) = obj::load_obj("obj/african_head.obj");
    // Loop over faces matrix
    for face in faces.iter() {
        // Extract the 3 vertices of the current triangle
        let p0 = vertices[face[0]];
        let p1 = vertices[face[1]];
        let p2 = vertices[face[2]];

        // Normalize cross product value of triangle sides
        let normal = (p2 - p0).cross(p1 - p0).normalize();

        // Back-face culling; skip triangles that are behind the object
        if normal.z > 0.0 {
            continue;
        }

        // Convert 3D coordinates to 2D screen coordinates
        let t0 = geometry::three_to_canvas(&p0, WIDTH, HEIGHT);
        let t1 = geometry::three_to_canvas(&p1, WIDTH, HEIGHT);
        let t2 = geometry::three_to_canvas(&p2, WIDTH, HEIGHT);

        // Triangle coordinates
        let triangle_pts = [t0, t1, t2];

        // Determine light intensity by dot product of light dir and triangle
        let intensity = normal.dot(&light_dir);

        // Set color based on light intensity
        let gray = (intensity * 255.0) as u8;
        let color = color::u8_rgb_color(gray, gray, gray);

        // Draw triangle
        geometry::triangle(&triangle_pts, &mut canvas_buf, WIDTH, HEIGHT, color);
    }

    /* ======OUPUT=============================================================== */
    while window.is_open() {
        window
            .update_with_buffer(&canvas_buf, WIDTH, HEIGHT)
            .unwrap();
    }
}
