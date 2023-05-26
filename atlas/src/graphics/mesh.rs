use std::{
    fmt,
    marker::PhantomData,
    mem::{self, size_of},
};

use glad_gl::gl;

use super::vertices::{Index, Shapely, Vertex};


#[derive(Clone, Copy)]
pub struct Primitive<
    VertexType: Vertex + Shapely<Attribute = VertexType> + fmt::Debug,
    IndexType: Index + Shapely<Attribute = IndexType> + fmt::Debug,
> {
    vao: u32,
    vbo: u32,
    ebo: u32,
    index_count: u32,
    phantom_vertex: PhantomData<VertexType>,
    phantom_index: PhantomData<IndexType>,
}

impl<
        VertexType: Vertex + Shapely<Attribute = VertexType> + fmt::Debug,
        IndexType: Index + Shapely<Attribute = IndexType> + fmt::Debug,
    > Primitive<VertexType, IndexType>
{
    pub fn new(vertices: &[VertexType], indices: &[IndexType]) -> Self {
        let (vao, vbo, ebo) = Self::create_buffers();

        let mut mesh = Self {
            vao,
            vbo,
            ebo,
            index_count: 0,
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
        self.index_count = IndexType::index_count(indices.len()) as u32;
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                VertexType::size(vertices.len()) as isize,
                mem::transmute(vertices.as_ptr()),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                IndexType::size(indices.len()) as isize,
                mem::transmute(indices.as_ptr()),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn primitive_type(&self) -> u32 {
        IndexType::primitive_type().into()
    }

    pub fn count(&self) -> u32 {
        self.index_count
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
        VertexType: Vertex + Shapely<Attribute = VertexType> + fmt::Debug,
        IndexType: Index + Shapely<Attribute = IndexType> + fmt::Debug,
    > Primitive<VertexType, IndexType>
{
    pub fn gen_quad(width: f32, height: f32) -> Self {
        let vertices = VertexType::gen_quad(width, height);
        let indices = IndexType::gen_quad(width, height);
        Self::new(&vertices, &indices)
    }

    pub fn skybox(side: f32) -> Self {
        let vertices = VertexType::skybox(side);
        let indices = IndexType::skybox(side);
        Self::new(&vertices, &indices)
    }

    pub fn sphere(radius: f32, detail: u32) -> Self {
        let vertices = VertexType::sphere(radius, detail);
        let indices = IndexType::sphere(radius, detail);
        Self::new(&vertices, &indices)
    }
}
