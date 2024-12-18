use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores 3 vertex values (x, y, z) 
#[derive(Debug)]
pub struct Vertex(f32, f32, f32);

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

                println!("{:?}", Vertex(x, y, z));
                vertices.push(Vertex(x, y, z));
            }
            Some("f") => {
                // Vertex indices start from 1 so sub 1
                // Retrieves the vertex coordinate of the first value in each vert idx, texture coor idx, and normal idx pair of a triangle face 
                let face: Vec<usize> = parts
                    .map(|part| part.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                    .collect();

                println!("{:?}", face);
                faces.push(face);
            }
            _ => {}
        }
    }

    (vertices, faces)
}
