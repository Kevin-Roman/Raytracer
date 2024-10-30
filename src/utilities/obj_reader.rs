use crate::core::{vector::Vector, vertex::Vertex};
use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct Triangle {
    pub vertex_indices: [usize; 3],
    pub vertex_normal_indices: [usize; 3],
    pub face_normal: Vector,
}

impl Triangle {
    pub fn new(
        vertex_indices: [usize; 3],
        vertex_normal_indices: [usize; 3],
        vertices: &Vec<Vertex>,
    ) -> Self {
        // Calculate face normal through cross product of two of the triangle's edges.
        let vert0 = vertices[vertex_indices[0]];
        let vert1 = vertices[vertex_indices[1]];
        let vert2 = vertices[vertex_indices[2]];

        let edge1 = vert1.vector - vert0.vector;
        let edge2 = vert2.vector - vert0.vector;

        let face_normal = edge1.cross(&edge2).normalise();

        Self {
            vertex_indices,
            vertex_normal_indices,
            face_normal,
        }
    }
}

#[derive(Clone, Copy)]
struct VertexAndNormalIndices {
    /// Vertex index;
    pub vertex_index: usize,
    /// Vertex normal index.
    pub vertex_normal_index: usize,
}

pub struct ObjReader {
    vertices: Vec<Vertex>,
    vertex_normals: Vec<Vertex>,
    faces: Vec<Vec<VertexAndNormalIndices>>,
}

impl ObjReader {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut vertex_normals: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Vec<VertexAndNormalIndices>> = Vec::new();

        let lines = Self::read_lines(file_path)?;
        for line in lines.flatten() {
            if line.starts_with("v ") {
                vertices.push(Self::process_vertex_line(&line));
            } else if line.starts_with("vn ") {
                vertex_normals.push(Self::process_vertex_line(&line));
            } else if line.starts_with("f ") {
                faces.push(Self::process_face_line(&line));
            }
        }

        Ok(Self {
            vertices,
            vertex_normals,
            faces,
        })
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn vertex_normals(&self) -> Vec<Vertex> {
        self.vertex_normals.clone()
    }

    pub fn triangles(&self) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = Vec::new();

        for face in &self.faces {
            triangles.extend(self.convert_face_to_triangles(face));
        }

        triangles
    }

    fn convert_face_to_triangles(&self, face: &Vec<VertexAndNormalIndices>) -> Vec<Triangle> {
        assert!(face.len() >= 3);

        let mut triangles: Vec<Triangle> = Vec::new();

        for i in 1..face.len() - 1 {
            triangles.push(Triangle::new(
                [
                    face[0].vertex_index,
                    face[i].vertex_index,
                    face[i + 1].vertex_index,
                ],
                [
                    face[0].vertex_normal_index,
                    face[i].vertex_normal_index,
                    face[i + 1].vertex_normal_index,
                ],
                &self.vertices,
            ));
        }

        triangles
    }

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
        let vertex_indices: Vec<usize> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.split("/").next().unwrap().parse::<usize>().unwrap() - 1)
            .collect();

        let vertex_normals_indices: Vec<usize> = line
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

        assert!(vertex_indices.len() >= 3 && vertex_normals_indices.len() >= 3);

        let mut faces: Vec<VertexAndNormalIndices> = Vec::new();
        for (vertex_index, vertex_normal_index) in vertex_indices
            .into_iter()
            .zip(vertex_normals_indices.into_iter())
        {
            faces.push(VertexAndNormalIndices {
                vertex_index,
                vertex_normal_index,
            });
        }

        faces
    }
}
