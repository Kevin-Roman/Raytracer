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

fn process_face_line(line: &str) -> Vec<VertexAndNormalIndices> {
    // Wavefront .obj files are 1-indexed.
    // Filter just the vertex index out.
    let vertices_indices: Vec<usize> = line
        .split_whitespace()
        .skip(1)
        .map(|s| s.split("/").next().unwrap().parse::<usize>().unwrap() - 1)
        .collect();

    let vertices_normals_indices: Vec<usize> = line
        .split_whitespace()
        .skip(1)
        .map(|s| {
            s.split("/")
                .skip(2)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
                - 1
        })
        .collect();

    assert!(vertices_indices.len() >= 3 && vertices_normals_indices.len() >= 3);

    let mut faces: Vec<VertexAndNormalIndices> = Vec::new();
    for (v_i, vn_i) in vertices_indices
        .into_iter()
        .zip(vertices_normals_indices.into_iter())
    {
        faces.push(VertexAndNormalIndices { v_i, vn_i });
    }

    faces
}

#[derive(Clone, Copy)]
pub struct VertexAndNormalIndices {
    /// Vertex index;
    pub v_i: usize,
    /// Vertex normal index.
    pub vn_i: usize,
}

pub struct ObjReader {
    vertices: Vec<Vertex>,
    vertices_normals: Vec<Vertex>,
    faces: Vec<Vec<VertexAndNormalIndices>>,
}

impl ObjReader {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut vertices_normals: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Vec<VertexAndNormalIndices>> = Vec::new();

        let lines = read_lines(file_path)?;
        for line in lines.flatten() {
            if line.starts_with("v ") {
                vertices.push(process_vertex_line(&line));
            } else if line.starts_with("vn ") {
                vertices_normals.push(process_vertex_line(&line));
            } else if line.starts_with("f ") {
                faces.push(process_face_line(&line));
            }
        }

        Ok(Self {
            vertices,
            vertices_normals,
            faces,
        })
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn vertices_normals(&self) -> Vec<Vertex> {
        self.vertices_normals.clone()
    }

    pub fn triangles(&self) -> Vec<[VertexAndNormalIndices; 3]> {
        let mut triangles: Vec<[VertexAndNormalIndices; 3]> = Vec::new();

        for face in &self.faces {
            triangles.extend(Self::convert_face_to_triangles(face));
        }

        triangles
    }

    fn convert_face_to_triangles(
        face: &Vec<VertexAndNormalIndices>,
    ) -> Vec<[VertexAndNormalIndices; 3]> {
        assert!(face.len() >= 3);

        let mut triangles: Vec<[VertexAndNormalIndices; 3]> = Vec::new();

        for i in 1..face.len() - 1 {
            triangles.push([face[0], face[i], face[i + 1]]);
        }

        triangles
    }
}
