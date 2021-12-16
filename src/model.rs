use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use crate::point::Point3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { x: x, y: y, z: z }
    }

    pub fn to_point3(self) -> Point3<f32> {
        Point3::new(self.x, self.y, self.z)
    }
}

pub struct Face {
    pub i0: usize,
    pub i1: usize,
    pub i2: usize,
}

impl Face {
    pub fn new(i0: usize, i1: usize, i2: usize) -> Self {
        Face {
            i0: i0,
            i1: i1,
            i2: i2,
        }
    }

    pub fn lines(&self) -> Vec<(usize, usize)> {
        vec![(self.i0, self.i1), (self.i1, self.i2), (self.i2, self.i0)]
    }
}

pub struct Model {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Model {
    pub fn new_from_obj(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for line in reader.lines() {
            let unwrapped_line = line.unwrap();
            let mut parts = unwrapped_line.split(" ");

            let line_type = parts.next().unwrap();
            match line_type {
                "v" => {
                    let x: f32 = parts.next().unwrap().parse().unwrap();
                    let y: f32 = parts.next().unwrap().parse().unwrap();
                    let z: f32 = parts.next().unwrap().parse().unwrap();

                    vertices.push(Vertex::new(x, y, z));
                }
                "f" => {
                    let i0: usize = parts
                        .next()
                        .unwrap()
                        .split("/")
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let i1: usize = parts
                        .next()
                        .unwrap()
                        .split("/")
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let i2: usize = parts
                        .next()
                        .unwrap()
                        .split("/")
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();

                    // THe file uses 1-indices, but we use 0-indices.
                    faces.push(Face::new(i0 - 1, i1 - 1, i2 - 1));
                }
                _ => {}
            }
        }

        Ok(Model {
            vertices: vertices,
            faces: faces,
        })
    }

    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    pub fn vertex(&self, index: usize) -> &Vertex {
        &self.vertices[index]
    }
}
