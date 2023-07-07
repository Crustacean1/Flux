use core::ffi;
use std::{mem, ops::Range, ptr};

use glad_gl::gl;

use crate::{game_root::GameError, resource_manager::ResourceLoader};

use super::{
    context::GenericMesh,
    vertices::{
        buffer::{Buffer, BufferTarget},
        indices::{IndexGeometry, TriangleGeometry},
        layouts::{Attribute, BufferElement, PTNVertex},
    },
};

#[derive(Clone)]
pub struct Mesh<Vertex: BufferElement, Index: IndexGeometry> {
    vao: u32,
    pub vertices: Buffer<Vertex>,
    pub indices: Buffer<Index>,
}

impl<Vertex: BufferElement, Index: IndexGeometry> Mesh<Vertex, Index> {
    pub fn new(vertices: &[Vertex], indices: &[Index]) -> Self {
        let primitive = Self {
            vao: Self::create_vao(),
            vertices: Buffer::build(vertices, BufferTarget::Vertex),
            indices: Buffer::build(indices, BufferTarget::Index),
        };

        primitive.use_vao(|| {
            primitive.vertices.bind();
            primitive.indices.bind();
            Self::declare_layout(&Vertex::layout(), 0, 0);
        });

        primitive
    }

    pub fn load_vertices(&mut self, vertices: &[Vertex]) {
        self.vertices.reload(vertices);
    }

    pub fn load_indices(&mut self, indices: &[Index]) {
        self.indices.reload(indices);
    }

    pub fn render(&self) {
        unsafe {
            self.use_vao(|| {
                gl::DrawElements(
                    //gl::POINTS,
                    Index::GEOMETRY,
                    self.index_count(),
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            })
        }
    }

    pub fn declare_layout(layout: &[Attribute], attrib_start: usize, divisor: u32) {
        let stride = layout.iter().map(|a| a.size()).sum::<usize>();
        let offsets = layout.iter().scan(0, |offset, attribute| {
            let position = *offset;
            *offset += attribute.size();
            Some(position)
        });

        unsafe {
            layout
                .iter()
                .zip(offsets)
                .enumerate()
                .for_each(|(i, (attrib, offset))| {
                    let attrib_index = (attrib_start + i) as u32;
                    attrib.declare(attrib_start + i, attrib.count(), offset, stride);
                    gl::VertexAttribDivisor(attrib_index, divisor);
                })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn use_vao(&self, action: impl FnOnce()) {
        unsafe {
            gl::BindVertexArray(self.vao);
            action();
            gl::BindVertexArray(0)
        }
    }

    pub fn index_count(&self) -> i32 {
        (self.indices.count() * (mem::size_of::<Index>() / mem::size_of::<u32>()) as u32) as i32
    }

    fn create_vao() -> u32 {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            vao
        }
    }
}

impl<Vertex: BufferElement, Index: IndexGeometry> GenericMesh for Mesh<Vertex, Index> {
    fn vao(&self) -> u32 {
        self.vao
    }
}

impl ResourceLoader for Mesh<PTNVertex, TriangleGeometry> {
    type Resource = Mesh<PTNVertex, TriangleGeometry>;
    fn is_resource(path: &std::path::PathBuf) -> bool {
        path.extension().map_or(false, |path| path == "mesh")
    }

    fn load_resource(contents: &[std::path::PathBuf]) -> Result<Self::Resource, GameError> {
        let file = contents
            .iter()
            .find(|entry| entry.is_file() && entry.extension().map_or(false, |ext| ext == "glb"))
            .ok_or(GameError::new(&format!(
                "No file with matching .glb extension found",
            )))?;

        Ok(Mesh::new(&vec![], &vec![]))
    }
}
