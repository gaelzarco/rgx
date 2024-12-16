use minifb::{Window, WindowOptions};

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;

// struct FrameBuf {
//     width: usize,
//     height: usize,
//     color: u32, // RGB Color
//     buf: Vec<u32>,
// }
//
// impl FrameBuf {
//     // Creates a new instance of Frame Buffer
//     pub fn new(width: usize, height: usize, color: u32) -> Self {
//         Self {
//             width,
//             height,
//             color,
//             buf: vec![color; width * height],
//         }
//     }
//
//     // Creates 32-bit RGB value using rgb input
//     pub fn u8_rgb_color(r: u8, g: u8, b: u8) -> u32 {
//         let (r, g, b) = (r as u32, g as u32, b as u32);
//         (r << 16) | (g << 8) | b
//     }
// }

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
    mut x0: f32,
    mut y0: f32,
    mut x1: f32,
    mut y1: f32,
    canvas_buf: &mut Vec<u32>,
    width: usize,
    height: usize,
    color: u32,
) {
    let mut steep = false;

    if (x0-x1).abs() < (y0-y1).abs() { // If the line in steep, we transpose the image
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }

    if x0 > x1 { // Make it left to right
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let mut x = x0;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = (dy/dx).abs();
    let mut error = 0.0;
    let mut y = y0;

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
            println!("ERROR: Coordinates are not within canvas bounds");
        }

        error = error + derror;
        if error > 0.5 {
            y = y + (if y1>y0 { 1.0 } else { -1.0 });
            error -= 1.0;
        }

        x += 1.0;
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

    line(
        0.0,
        0.0,
        300.0,
        200.0,
        &mut canvas_buf,
        WIDTH,
        HEIGHT,
        u8_rgb_color(0, 124, 210),
    );

    line(
        200.0,
        300.0,
        0.0,
        0.0,
        &mut canvas_buf,
        WIDTH,
        HEIGHT,
        u8_rgb_color(125, 124, 210),
    );
    
    while window.is_open() {
        window
            .update_with_buffer(&canvas_buf, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("Error drawing buffer {e}");
            });
    }
}
