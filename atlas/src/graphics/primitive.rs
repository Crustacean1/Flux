use core::slice;
use std::mem::{self, size_of};

use glad_gl::gl;

use super::vertices::{
    base_vertices::{TriangleIndex, Vertex3PT},
    Shapely,
};

pub enum MeshIndices {
    Points(Vec<u32>),
    Lines(Vec<u32>),
    Triangles(Vec<u32>),
}

impl MeshIndices {
    pub fn to_buffer(&self) -> &[u32] {
        match self {
            MeshIndices::Points(buffer) => buffer,
            MeshIndices::Lines(buffer) => buffer,
            MeshIndices::Triangles(buffer) => buffer,
        }
    }

    pub fn to_gl(&self) -> u32 {
        match self {
            MeshIndices::Points(_) => gl::POINTS,
            MeshIndices::Lines(_) => gl::LINES,
            MeshIndices::Triangles(_) => gl::TRIANGLES,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Primitive {
    pub vao: u32,
    vbo: u32,
    ebo: u32,
    index_count: u32,
    mode: u32,
}

impl Default for Primitive {
    fn default() -> Self {
        Self::sphere(1.0, 10)
    }
}

impl Primitive {
    pub fn new(vertices: &[f32], shape: &[usize], indices: &mut MeshIndices) -> Self {
        let (vao, vbo, ebo) = Self::create_buffers(shape);

        let mut mesh = Self {
            vao,
            vbo,
            ebo,
            mode: indices.to_gl(),
            index_count: 0,
        };

        mesh.load(&vertices, indices.to_buffer());
        mesh
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn load(&mut self, vertices: &[f32], indices: &[u32]) {
        self.index_count = indices.len() as u32;
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize,
                mem::transmute(vertices.as_ptr()),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as isize,
                mem::transmute(indices.as_ptr()),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn primitive_type(&self) -> u32 {
        self.mode
    }

    pub fn count(&self) -> u32 {
        self.index_count * 4
    }

    fn create_buffers(attributes: &[usize]) -> (u32, u32, u32) {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            let stride = size_of::<f32>() * attributes.iter().sum::<usize>();

            attributes.iter().enumerate().fold(0, |offset, (i, &size)| {
                gl::VertexAttribPointer(
                    i as u32,
                    size as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const std::ffi::c_void,
                );

                gl::EnableVertexAttribArray(i as u32);

                offset + size * size_of::<f32>()
            });
        }
        (vao, vbo, ebo)
    }
}

impl Primitive {
    pub fn quad(width: f32, height: f32) -> Self {
        todo!()
    }

    pub fn skybox(side: f32) -> Self {
        let vertices = Vertex3PT::skybox(side);
        let vertices: Vec<_> = vertices
            .iter()
            .map(|v| [v.pos[0], v.pos[1], v.pos[2], v.tex[0], v.tex[1]])
            .flatten()
            .collect();

        let indices = TriangleIndex::skybox(side);
        let indices: Vec<_> = indices
            .iter()
            .map(|TriangleIndex { triangle }| *triangle)
            .flatten()
            .collect();

        Self::new(&vertices, &[3, 2], &mut MeshIndices::Triangles(indices))
    }

    pub fn sphere(radius: f32, detail: u32) -> Self {
        let vertices = Vertex3PT::sphere(radius, detail);
        let vertices: Vec<_> = vertices
            .iter()
            .map(|v| [v.pos[0], v.pos[1], v.pos[2], v.tex[0], v.tex[1]])
            .flatten()
            .collect();

        let indices = TriangleIndex::sphere(radius, detail);
        let indices: Vec<_> = indices
            .iter()
            .map(|TriangleIndex { triangle }| *triangle)
            .flatten()
            .collect();

        Self::new(&vertices, &[3, 2], &mut MeshIndices::Triangles(indices))
    }
}
