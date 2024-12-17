use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Vertex(f32, f32, f32);

pub fn load_obj(file_path: &str) {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => panic!("ERROR: Could not find/open specified file")
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_parts = line.split_whitespace();
    }
}
