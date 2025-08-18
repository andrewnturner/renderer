use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use nalgebra::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vertex(Vector3<f32>);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex(Vector3::new(x, y, z))
    }

    pub fn as_vector3(&self) -> &Vector3<f32> {
        &self.0
    }
}

// We use anti-clockwise vertex ordering for the outside.
pub struct Face {
    pub i0: usize,
    pub i1: usize,
    pub i2: usize,
    pub vt0: Option<usize>,
    pub vt1: Option<usize>,
    pub vt2: Option<usize>,
}

impl Face {
    pub fn new(
        i0: usize,
        i1: usize,
        i2: usize,
        vt0: Option<usize>,
        vt1: Option<usize>,
        vt2: Option<usize>,
    ) -> Self {
        Face {
            i0: i0,
            i1: i1,
            i2: i2,
            vt0: vt0,
            vt1: vt1,
            vt2: vt2,
        }
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
            let parts: Vec<&str> = unwrapped_line.split(" ").collect();

            let line_type = parts[0];
            match line_type {
                "v" => {
                    let x: f32 = parts[1].parse().unwrap();
                    let y: f32 = parts[2].parse().unwrap();
                    let z: f32 = parts[3].parse().unwrap();

                    vertices.push(Vertex::new(x, y, z));
                }
                "f" => match parts.len() {
                    4 => {
                        let (i0, vt0) = parse_face_vertex(parts[1]);
                        let (i1, vt1) = parse_face_vertex(parts[2]);
                        let (i2, vt2) = parse_face_vertex(parts[3]);

                        faces.push(Face::new(i0, i1, i2, vt0, vt1, vt2));
                    }
                    5 => {
                        let (i0, vt0) = parse_face_vertex(parts[1]);
                        let (i1, vt1) = parse_face_vertex(parts[2]);
                        let (i2, vt2) = parse_face_vertex(parts[3]);
                        let (i3, vt3) = parse_face_vertex(parts[4]);

                        faces.push(Face::new(i0, i1, i2, vt0, vt1, vt2));
                        faces.push(Face::new(i0, i2, i3, vt0, vt2, vt3));
                    }

                    num_parts => {
                        panic!("Unsupported number of vertices: {}", num_parts - 1)
                    }
                },
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

fn parse_face_vertex(vertex_str: &str) -> (usize, Option<usize>) {
    let mut parts = vertex_str.split("/");

    // The file uses 1-indices, but we use 0-indices.
    let vertex_index = parts.next().unwrap().parse::<usize>().unwrap() - 1;
    let vertex_texture_index = parts.next().map(|x| x.parse::<usize>().unwrap() - 1);

    (vertex_index, vertex_texture_index)
}
