use core::ffi;
use std::{mem, ops::Range, ptr};

use glad_gl::gl;

use super::vertices::{
    buffer::{Buffer, BufferTarget},
    indices::IndexGeometry,
    layouts::BufferElement,
};

#[derive(Clone)]
pub struct Primitive<Vertex: BufferElement, Index: IndexGeometry> {
    vao: u32,
    pub vertices: Buffer<Vertex>,
    pub indices: Buffer<Index>,
}

impl<Vertex: BufferElement, Index: IndexGeometry> Primitive<Vertex, Index> {
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
                    Index::GEOMETRY,
                    self.index_count(),
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            })
        }
    }

    pub fn render_sub(&self, range: Range<u32>) {
        let count = range.clone().count();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                Index::GEOMETRY,
                count as i32,
                gl::UNSIGNED_INT,
                mem::transmute(range.start as usize * mem::size_of::<u32>()),
            );
        }
    }

    pub fn declare_layout(layout: &[usize], attrib_start: usize, divisor: u32) {
        let stride = layout.iter().sum::<usize>() * mem::size_of::<f32>();
        let offsets = layout.iter().scan(0, |offset, &size| {
            let position = *offset;
            *offset += size * mem::size_of::<f32>();
            Some(position)
        });

        unsafe {
            layout
                .iter()
                .zip(offsets)
                .enumerate()
                .for_each(|(i, (attrib, offset))| {
                    let attrib_index = (attrib_start + i) as u32;
                    gl::EnableVertexAttribArray(attrib_index);
                    gl::VertexAttribPointer(
                        attrib_index,
                        *attrib as i32,
                        gl::FLOAT,
                        gl::FALSE,
                        stride as i32,
                        offset as *const ffi::c_void,
                    );
                    gl::VertexAttribDivisor(attrib_index, divisor);
                })
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
