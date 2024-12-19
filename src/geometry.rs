use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores 3 vertex values (x, y, z)
#[derive(Debug)]
pub struct Vertex(pub f32, pub f32, pub f32);

// Stores 2D coordinates for simple line drawing
#[derive(Debug)]
pub struct Coordinate(pub i32, pub i32);

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
                // Retrieves the vertex coordinate of the first value in each vert idx, texture coor idx, and normal idx pair of a triangle face
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
///
/// Y-axis flipped to transform y axis to screen space
pub fn three_to_canvas(v: &Vertex, width: usize, height: usize) -> (i32, i32) {
    let x = ((v.0 + 1.0) * width as f32 / 2.0) as i32;
    let y = (height as f32 - (v.1 + 1.0) * height as f32 / 2.0) as i32; // Flip Y-axis
    (x, y)
}

pub fn triangle(
    v1: Coordinate,
    v2: Coordinate,
    v3: Coordinate,
    canvas: &mut Vec<u32>,
    width: usize,
    height: usize,
    color: u32,
) {
    crate::line(v1.0, v1.1, v2.0, v2.1, canvas, width, height, color);
    crate::line(v2.0, v2.1, v3.0, v3.1, canvas, width, height, color);
    crate::line(v3.0, v3.1, v1.0, v1.1, canvas, width, height, color);
}
