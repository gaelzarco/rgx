use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores 3 vertex values (x, y, z)
#[derive(Debug)]
pub struct Point {
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
}

impl IntoIterator for Point {
    type Item = f32;
    type IntoIter = PointIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PointIntoIterator {
            point: self,
            idx: 0
        }
    }
}

#[derive(Debug)]
pub struct PointIntoIterator {
    pub point: Point,
    pub idx: usize
}

impl Iterator for PointIntoIterator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let result = match self.idx {
            0 => self.point.x,
            1 => self.point.y,
            2 => self.point.z,
            _ => return None
        };
        
        self.idx += 1;
        Some(result)
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn product(one: Point, two: Point) -> Point {
        one.into_iter()
            .map(|item| two
                .into_iter()
                .collect()
            )
            .flatten()
            .collect()
    }
}

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
pub fn three_to_canvas(v: &Point, width: usize, height: usize) -> (i32, i32) {
    let x = ((v.x + 1.0) * width as f32 / 2.0) as i32;
    let y = ((v.y + 1.0) * height as f32 / 2.0) as i32;
    (x, y)
}

/// Draws a 2D triangle onto the screen
/// 
/// Utilizes old-school line-sweeping method
// pub fn triangle(
//     mut v0: (i32, i32),
//     mut v1: (i32, i32),
//     mut v2: (i32, i32),
//     canvas: &mut Vec<u32>,
//     width: usize,
//     height: usize,
//     color: u32,
// ) {
//     // Bubblesort coordinates by y-axis (ascending)
//     if v0.1 > v1.1 {
//         (v0, v1) = (v1, v0);
//     }
//     if v0.1 > v2.1 {
//         (v0, v2) = (v2, v0);
//     }
//     if v1.1 > v2.1 {
//         (v1, v2) = (v2, v1)
//     }
// 
//     // Total height of bottom half
//     let total_height = v2.1 - v0.1;
// 
//     // Draw bottom half
//     for y in v0.1..=v1.1 {
//         let segment_height = v1.1 - v0.1;
//         if segment_height <= 0 {
//             panic!(
//                 "Segment height cannot be less than 0. v1.y = {}, v0.y = {}",
//                 v1.1, v0.1
//             );
//         }
//         let alpha = (y - v0.1) as f32 / total_height as f32;
//         let beta = (y - v0.1) as f32 / segment_height as f32;
// 
//         let mut a = (v0.0 + ((v2.0 - v0.0) as f32 * alpha) as i32, y);
//         let mut b = (v0.0 + ((v1.0 - v0.0) as f32 * beta) as i32, y);
// 
//         if a.0 > b.0 {
//             (a.0, a.1, b.0, b.1) = (b.0, b.1, a.0, a.1);
//         }
// 
//         crate::line(a.0, a.1, b.0, b.1, canvas, width, height, color);
//     }
// 
//     // Draw top half
//     for y in v1.1..=v2.1 {
//         let segment_height = v2.1 - v1.1;
//         if segment_height <= 0 {
//             panic!(
//                 "Segment height cannot be less than 0. v2.y = {}, v1.y = {}",
//                 v2.1, v1.1
//             );
//         }
//         let alpha = (y - v0.1) as f32 / total_height as f32;
//         let beta = (y - v1.1) as f32 / segment_height as f32;
// 
//         let mut a = (v0.0 + ((v2.0 - v0.0) as f32 * alpha) as i32, y);
//         let mut b = (v1.0 + ((v2.0 - v1.0) as f32 * beta) as i32, y);
// 
//         if a.0 > b.0 {
//             (a.0, a.1, b.0, b.1) = (b.0, b.1, a.0, a.1);
//         }
// 
//         crate::line(a.0, a.1, b.0, b.1, canvas, width, height, color);
//     }
// }
//
//
pub fn barycentric_coord(pts: (Point, Point), p: (f32, f32)) {
    let u: (f32, f32, f32) = (pts.1.x - pts.0.x, pts.1.x - pts.0.x, pts.0.x - p.0);

}

pub fn triangle() {

}
