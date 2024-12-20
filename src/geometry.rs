use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores 3 vertex values (x, y, z)
#[derive(Debug)]
pub struct Vertex(pub f32, pub f32, pub f32);

/// Stores 2D coordinates, x and y in that order
/// #[derive(Debug)]
/// pub struct Coordinate(pub i32, pub i32);

/// Load Geometry into memory
///
/// Takes in a file path and returns a tuple of vectors
///
/// The first is a vector of geometric vertex coordinates
/// The second is a matrix of face element coordinate vectors
///
/// This fuction will panic if the file_path specified cannot be found or the file cannot be opened.
pub fn load_obj(file_path: &str) -> (Vec<Vertex>, Vec<Vec<usize>>) {
    let mut vertices: Vec<Vertex> = vec![];
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

                vertices.push(Vertex(x, y, z));
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
pub fn three_to_canvas(v: &Vertex, width: usize, height: usize) -> (i32, i32) {
    let x = ((v.0 + 1.0) * width as f32 / 2.0) as i32;
    let y = ((v.1 + 1.0) * height as f32 / 2.0) as i32;
    (x, y)
}

/// Draws a 2D triangle onto the screen
pub fn triangle(
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

        crate::line(a.0, a.1, b.0, b.1, canvas, width, height, color);
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

        crate::line(a.0, a.1, b.0, b.1, canvas, width, height, color);
    }

    //crate::line(
    //    v0.0,
    //    v0.1,
    //    v1.0,
    //    v1.1,
    //    canvas,
    //    width,
    //    height,
    //    crate::u8_rgb_color(0, 255, 0),
    //);
    //crate::line(
    //    v1.0,
    //    v1.1,
    //    v2.0,
    //    v2.1,
    //    canvas,
    //    width,
    //    height,
    //    crate::u8_rgb_color(0, 255, 0),
    //);
    //crate::line(
    //    v2.0,
    //    v2.1,
    //    v0.0,
    //    v0.1,
    //    canvas,
    //    width,
    //    height,
    //    crate::u8_rgb_color(255, 0, 0),
    //);
}
