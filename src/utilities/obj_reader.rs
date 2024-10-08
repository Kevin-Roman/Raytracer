use crate::core::vertex::Vertex;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_vertex_line(line: &str) -> Vertex {
    let coords: Vec<f32> = line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<f32>().unwrap())
        .collect();

    assert_eq!(coords.len(), 3);

    Vertex::new(coords[0], coords[1], coords[2], 1.0)
}

fn process_face_line(line: &str) -> Vec<usize> {
    // Wavefront .obj files are 1-indexed.
    // Filter just the vertex index out.
    let vertices_indices: Vec<usize> = line
        .split_whitespace()
        .skip(1)
        .map(|s| s.split("/").next().unwrap().parse::<usize>().unwrap() - 1)
        .collect();

    assert!(vertices_indices.len() >= 3);

    vertices_indices
}

pub struct ObjReader {
    vertices: Vec<Vertex>,
    faces: Vec<Vec<usize>>,
}

impl ObjReader {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Vec<usize>> = Vec::new();

        let lines = read_lines(file_path)?;
        for line in lines.flatten() {
            if line.starts_with("v ") {
                // Vertices.
                vertices.push(process_vertex_line(&line));
            } else if line.starts_with("f ") {
                // Faces.
                faces.push(process_face_line(&line));
            }
        }

        Ok(Self { vertices, faces })
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn triangles(&self) -> Vec<[usize; 3]> {
        let mut triangles: Vec<[usize; 3]> = Vec::new();

        for face in &self.faces {
            triangles.extend(Self::convert_face_to_triangles(face));
        }

        triangles
    }

    fn convert_face_to_triangles(face: &Vec<usize>) -> Vec<[usize; 3]> {
        assert!(face.len() >= 3);

        let mut triangles: Vec<[usize; 3]> = Vec::new();

        for i in 1..face.len() - 1 {
            triangles.push([face[0], face[i], face[i + 1]]);
        }

        return triangles;
    }
}
