use glad_gl::gl;
use std::ptr;

use super::{
    primitive::Primitive,
    vertices::{
        buffer::{Buffer, BufferTarget},
        indices::IndexGeometry,
        layouts::BufferElement,
    },
};

#[derive(Clone)]
pub struct InstancedPrimitive<
    InstancedData: BufferElement,
    Vertex: BufferElement,
    Index: IndexGeometry,
> {
    primitive: Primitive<Vertex, Index>,
    instanced_buffer: Buffer<InstancedData>,
}

impl<Instance: BufferElement, Vertex: BufferElement, Index: IndexGeometry>
    InstancedPrimitive<Instance, Vertex, Index>
{
    pub fn new(vertices: &[Vertex], indices: &[Index], data: &[Instance]) -> Self {
        let primitive = Primitive::new(vertices, indices);
        let instanced_buffer = Buffer::build(data, BufferTarget::Vertex);
        let start_attrib = Vertex::layout().len();

        primitive.use_vao(|| {
            instanced_buffer.bind();
            Primitive::<Vertex, Index>::declare_layout(&Instance::layout(), start_attrib, 1);
        });

        Self {
            primitive,
            instanced_buffer,
        }
    }

    pub fn render(&self) {
        unsafe {
            self.primitive.use_vao(|| {
                //println!("Rendering {} {}", self.primitive.index_count(), self.instanced_buffer.count());
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
