use std::{marker::PhantomData, mem};

use glad_gl::gl;

use super::vertices::{Index, PrimitiveType, Shapely, Vertex};

pub struct Mesh<
    VertexType: Vertex + Shapely<Attribute = VertexType>,
    IndexType: Index + Shapely<Attribute = IndexType>,
> {
    vao: u32,
    vbo: u32,
    ebo: u32,
    phantom_vertex: PhantomData<VertexType>,
    phantom_index: PhantomData<IndexType>,
}

impl<
        VertexType: Vertex + Shapely<Attribute = VertexType>,
        IndexType: Index + Shapely<Attribute = IndexType>,
    > Mesh<VertexType, IndexType>
{
    pub fn new(vertices: &[VertexType], indices: &[IndexType]) -> Self {
        let (vao, vbo, ebo) = Self::create_buffers();

        let mut mesh = Self {
            vao,
            vbo,
            ebo,
            phantom_index: Default::default(),
            phantom_vertex: Default::default(),
        };

        mesh.load(vertices, indices);

        mesh
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn load(&mut self, vertices: &[VertexType], indices: &[IndexType]) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vertices.len() as isize,
                mem::transmute(vertices.as_ptr()),
                gl::STATIC_DRAW,
            );

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                indices.len() as isize,
                mem::transmute(indices.as_ptr()),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn primitive_type(&self) -> u32 {
        IndexType::primitive_type().into()
    }

    pub fn count(&self) -> i32 {
        0
    }

    fn create_buffers() -> (u32, u32, u32) {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            VertexType::declare_layout();
        }
        (vao, vbo, ebo)
    }
}

impl<
        VertexType: Vertex + Shapely<Attribute = VertexType>,
        IndexType: Index + Shapely<Attribute = IndexType>,
    > Mesh<VertexType, IndexType>
{
    pub fn gen_quad(width: f32, height: f32) -> Self {
        let vertices = VertexType::gen_quad(width, height);
        let indices = IndexType::gen_quad(width, height);
        Self::new(&vertices, &indices)
    }
}
