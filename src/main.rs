use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct FrameBuf {
    width: usize,
    height: usize,
    color: u32, // RGB Color
    buf: Vec<u32>,
}

impl FrameBuf {
    // Creates a new instance of Frame Buffer
    pub fn new(width: usize, height: usize, color: u32) -> Self {
        Self {
            width,
            height,
            color,
            buf: vec![color; width * height],
        }
    }

    // Creates 32-bit RGB value using rgb input
    pub fn u8_rgb_color(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
}

//
// fn bresenham_line(x0: f32, y0: f32, x1: f32, y1: f32, canvas_buf: Vec<u32>, line: FrameBuf) {
//     println!("Line: x0: {}, y0: {}, x1: {}, x2: {}", x0, y0, x1, y1);
//
//     let mut t = 0.0;
//
//     while t < 1.0 {
//         let x = x0 + (x1 - x0) * t;
//         let y = y0 + (y1 - y0) * t;
//         t = t + 0.01;
//     }
//
//     println!("Loop exit");
// }

fn bresenham_line(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    canvas_buf: &mut Vec<u32>,
    width: usize,
    color: u32,
) {
    let mut t = 0.0;

    while t < 1.0 {
        let x = (x0 + (x1 - x0) * t) as usize;
        let y = (y0 + (y1 - y0) * t) as usize;

        if x < width && y < canvas_buf.len() / width {
            let idx = y * width + x;
            canvas_buf[idx] = color;
        }

        t += 0.01;
    }
}

fn main() {
    let mut window = match Window::new(
        "rust renderer",
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

    // let line = FrameBuf::new(100, 1, FrameBuf::u8_rgb_color(0, 124, 210));
    let mut canvas_buf = vec![0; WIDTH * HEIGHT];

    // Draw the line at a specific position in the full buffer
    // w/o the implementation of a line drawing algorithm
    // for x in 0..line.width {
    //     if x < WIDTH {
    //         let idx = x + (HEIGHT / 2) * WIDTH; // Place the line in the middle vertically
    //         canvas_buf[idx] = line.color;
    //     }
    // }

    bresenham_line(
        0.0,
        0.0,
        300.0,
        200.0,
        &mut canvas_buf,
        WIDTH,
        FrameBuf::u8_rgb_color(0, 124, 210),
    );

    while window.is_open() {
        window
            .update_with_buffer(&canvas_buf, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("Error drawing buffer {e}");
            });
    }
}
