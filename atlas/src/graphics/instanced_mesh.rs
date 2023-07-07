use glad_gl::gl;
use std::ptr;

use super::{
    mesh::Mesh,
    vertices::{
        buffer::{Buffer, BufferTarget},
        indices::IndexGeometry,
        layouts::BufferElement,
    },
};

#[derive(Clone)]
pub struct InstancedMesh<InstancedData: BufferElement, Vertex: BufferElement, Index: IndexGeometry>
{
    primitive: Mesh<Vertex, Index>,
    instanced_buffer: Buffer<InstancedData>,
}

impl<Instance: BufferElement, Vertex: BufferElement, Index: IndexGeometry>
    InstancedMesh<Instance, Vertex, Index>
{
    pub fn new(vertices: &[Vertex], indices: &[Index], data: &[Instance]) -> Self {
        let primitive = Mesh::new(vertices, indices);
        let instanced_buffer = Buffer::build(data, BufferTarget::Vertex);
        let start_attrib = Vertex::layout().len();

        primitive.use_vao(|| {
            instanced_buffer.bind();
            Mesh::<Vertex, Index>::declare_layout(&Instance::layout(), start_attrib, 1);
        });

        Self {
            primitive,
            instanced_buffer,
        }
    }

    pub fn render(&self) {
        unsafe {
            self.primitive.use_vao(|| {
                gl::DrawElementsInstanced(
                    Index::GEOMETRY,
                    self.primitive.index_count(),
                    gl::UNSIGNED_INT,
                    ptr::null(),
                    self.instanced_buffer.count() as i32,
                )
            })
        }
    }

    pub fn load_instances(&mut self, instances: &[Instance]) {
        self.instanced_buffer.reload(instances);
    }
}
