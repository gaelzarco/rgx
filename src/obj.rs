/*
 * =============================================================================
 * Crate: Object
 * Author: Gael Zarco
 * Description: Includes methods for loading .obj files into memory and
 *              returning trianlge vertex and face cooridnate data.
 * =============================================================================
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::geometry::point::Point;

/// Load Geometry into memory
///
/// Takes in a file path and returns a tuple of vectors
///
/// The first is a vector of geometric vertex coordinates
/// The second is a matrix of face element coordinate vectors
///
/// Function panics if file cannot be found or opened.
pub fn load_obj(file_path: &str) -> (Vec<Point>, Vec<Vec<usize>>) {
    let mut vertices: Vec<Point> = vec![];
    let mut faces = Vec::new();

    // Open file
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => panic!("ERROR: Could not find/open specified file"),
    };
    // Read contents of file
    let reader = BufReader::new(file);

    // Iterate over contents in file
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        // Use parts that include vertex or face data
        match parts.next() {
            Some("v") => {
                let x = parts.next().unwrap().parse::<f32>().unwrap();
                let y = parts.next().unwrap().parse::<f32>().unwrap();
                let z = parts.next().unwrap().parse::<f32>().unwrap();

                vertices.push(Point::new(x, y, z));
            }
            Some("f") => {
                // Face vert indices start from 1 so sub 1
                // Retrieves vert coordinate of first value in each vert y,
                // texture coor y, and normal y pair of triangle face
                let face: Vec<usize> = parts
                    .map(|part| part.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                    .collect();

                faces.push(face);
            }
            _ => {}
        }
    }

    // Return tuple of vertex and face matrices
    (vertices, faces)
}
