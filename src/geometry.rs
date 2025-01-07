use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores 3 vertex values (x, y, z)
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::BitXor for Point {
    type Output = Point;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Point::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

/// Load Geometry into memory
///
/// Takes in a file path and returns a tuple of vectors
///
/// The first is a vector of geometric vertex coordinates
/// The second is a matrix of face element coordinate vectors
///
/// This fuction will panic if the file_path specified cannot be found or the file cannot be opened.
pub fn load_obj(file_path: &str) -> (Vec<Point>, Vec<Vec<usize>>) {
    let mut vertices: Vec<Point> = vec![];
    let mut faces = Vec::new();

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => panic!("ERROR: Could not find/open specified file"),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("v") => {
                let x = parts.next().unwrap().parse::<f32>().unwrap();
                let y = parts.next().unwrap().parse::<f32>().unwrap();
                let z = parts.next().unwrap().parse::<f32>().unwrap();

                vertices.push(Point::new(x, y, z));
            }
            Some("f") => {
                // Face vertex indices start from 1 so sub 1
                // Retrieves the vertex coordinate of the first value in each vert y, texture coor y, and normal y pair of a triangle face
                let face: Vec<usize> = parts
                    .map(|part| part.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                    .collect();

                faces.push(face);
            }
            _ => {}
        }
    }

    (vertices, faces)
}

/// Transform 3D coordinates to 2D space
///
/// Takes in specified vertex and the canvas width/height to transform coordinates to 2D space
///
/// Translates normalized x and y vertex coordinates to match 2D origin and scales them to resolution
pub fn three_to_canvas(v: &Point, width: usize, height: usize) -> (f32, f32) {
    let x = (v.x + 1.0) * width as f32 / 2.0;
    let y = (v.y + 1.0) * height as f32 / 2.0;
    (x, y)
}

/// Bresenham Line
///
/// Takes in x0, y0, x1, and y1 values respectively.
/// These values are used draw a line of a specified color on a canvas buffer
///
/// The canvas buffer consists of a vector of pixels which length equals the total sum of the width x height
///
/// The width and height vars are the width and height of the canvas
pub fn line(
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
                "s (x:{}, y:{}) are not within canvas bounds",
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

/// Draws a 2D triangle onto the screen
///
/// Utilizes line-sweeping method
pub fn old_skool_triangle(
    mut v0: (i32, i32),
    mut v1: (i32, i32),
    mut v2: (i32, i32),
    canvas: &mut Vec<u32>,
    width: usize,
    height: usize,
    color: u32,
) {
    // Bubblesort coordinates by y-axis (ascending)
    if v0.1 > v1.1 {
        (v0, v1) = (v1, v0);
    }
    if v0.1 > v2.1 {
        (v0, v2) = (v2, v0);
    }
    if v1.1 > v2.1 {
        (v1, v2) = (v2, v1)
    }

    // Total height of bottom half
    let total_height = v2.1 - v0.1;

    // Draw bottom half
    for y in v0.1..=v1.1 {
        let segment_height = v1.1 - v0.1;
        if segment_height <= 0 {
            panic!(
                "Segment height cannot be less than 0. v1.y = {}, v0.y = {}",
                v1.1, v0.1
            );
        }
        let alpha = (y - v0.1) as f32 / total_height as f32;
        let beta = (y - v0.1) as f32 / segment_height as f32;

        let mut a = (v0.0 + ((v2.0 - v0.0) as f32 * alpha) as i32, y);
        let mut b = (v0.0 + ((v1.0 - v0.0) as f32 * beta) as i32, y);

        if a.0 > b.0 {
            (a.0, a.1, b.0, b.1) = (b.0, b.1, a.0, a.1);
        }

        line(a.0, a.1, b.0, b.1, canvas, width, height, color);
    }

    // Draw top half
    for y in v1.1..=v2.1 {
        let segment_height = v2.1 - v1.1;
        if segment_height <= 0 {
            panic!(
                "Segment height cannot be less than 0. v2.y = {}, v1.y = {}",
                v2.1, v1.1
            );
        }
        let alpha = (y - v0.1) as f32 / total_height as f32;
        let beta = (y - v1.1) as f32 / segment_height as f32;

        let mut a = (v0.0 + ((v2.0 - v0.0) as f32 * alpha) as i32, y);
        let mut b = (v1.0 + ((v2.0 - v1.0) as f32 * beta) as i32, y);

        if a.0 > b.0 {
            (a.0, a.1, b.0, b.1) = (b.0, b.1, a.0, a.1);
        }

        line(a.0, a.1, b.0, b.1, canvas, width, height, color);
    }
}

/// Caclulate Barycentric coordinates of triangle for triangle rasterization
pub fn barycentric(pts: &[Point; 3], p: Point) -> Point {
    let u = Point::new(pts[2].x - pts[0].x, pts[1].x - pts[0].x, pts[0].x - p.x)
        ^ Point::new(pts[2].y - pts[0].y, pts[1].y - pts[0].y, pts[0].y - p.y);

    if u.z.abs() < 1.0 {
        return Point::new(-1.0, 1.0, 1.0);
    }

    Point::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
}

/// Improved method for drawing triangles
pub fn triangle(
    pts: &[Point; 3],
    canvas_buf: &mut Vec<u32>,
    width: usize,
    height: usize,
    color: u32,
) {
    // Initialize bounding box
    let mut bbox_min = Point::new(width as f32 - 1.0, height as f32 - 1.0, 0.0);
    let mut bbox_max = Point::new(0.0, 0.0, 0.0);
    let clamp = Point::new(width as f32 - 1.0, height as f32 - 1.0, 0.0);

    // Calculate bounding box
    for i in 0..3 {
        // Invert Y-coordinate for screen space
        let screen_space_y = height as f32 - pts[i].y;

        bbox_min.x = f32::max(0.0, f32::min(bbox_min.x, pts[i].x));
        bbox_min.y = f32::max(0.0, f32::min(bbox_min.y, screen_space_y));
        bbox_max.x = f32::min(clamp.x, f32::max(bbox_max.x, pts[i].x));
        bbox_max.y = f32::min(clamp.y, f32::max(bbox_max.y, screen_space_y));
    }

    // Iterate over all pixels in the bounding box
    for px in bbox_min.x as usize..=bbox_max.x as usize {
        for py in bbox_min.y as usize..=bbox_max.y as usize {
            // Convert back to triangle space (invert Y for barycentric calculation)
            let p = Point::new(px as f32, height as f32 - py as f32, 0.0);

            // Compute barycentric coordinates
            let bc_screen = barycentric(pts, p);

            // Check if pixel is inside the triangle
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            // Calculate buffer index
            let idx = py * width + px;

            // Set pixel in buffer
            if idx < canvas_buf.len() {
                canvas_buf[idx] = color;
            }
        }
    }
}
