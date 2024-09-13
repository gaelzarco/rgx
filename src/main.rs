use minifb::{Window, WindowOptions, MouseMode};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

// Creates 32-bit RGB value using rgb input
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g ,b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let mut mx = 0.0;
    let mut my = 0.0;

    let mut window = match Window::new("RTGX", WIDTH, HEIGHT, WindowOptions {
        resize: true,
        ..WindowOptions::default()
    }) {
        Ok(win) => win,
        Err(err) => {
            println!("unable to create window {}", err);
            return;
        }
    };

    let blue: u32 = from_u8_rgb(0, 124, 210);  
    let buf: Vec<u32> = vec![blue; WIDTH * HEIGHT];

    window.update();
    window.set_target_fps(120);

    while window.is_open() {
        window.update();
        window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
            if &mx != &mouse.0 || &my != &mouse.1 {
                println!("x {} y {}", mouse.0, mouse.1);
                
                mx = mouse.0;
                my = mouse.1;
            }
        });
    }
}
