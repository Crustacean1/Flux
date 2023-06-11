use glad_gl::gl;

use super::vertices::{
    buffer::Buffer,
    layouts::{BufferElement, IndexGeometry},
    Shapely,
};

#[derive(Clone)]
pub struct Primitive<Vertex: BufferElement, Index: IndexGeometry> {
    vao: u32,
    vertices: Buffer<Vertex>,
    indices: Buffer<Index>,
}

impl<Vertex: BufferElement, Geometry: IndexGeometry> Default for Primitive<Vertex, Geometry> {
    fn default() -> Self {
        Self::sphere(1.0, 10)
    }
}

impl<Vertex: BufferElement, Geometry: IndexGeometry> Primitive<Vertex, Geometry> {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Geometry>) -> Self {
        let vao = Self::create_vao();
        let vertices = Buffer::build(vertices);
        let indices = Buffer::build(indices);

        Self {
            vao,
            vertices,
            indices,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    /*pub fn reload_vertices(&mut self, vertices: &[f32]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let mut buffer_size: i32 = 0;
            gl::GetBufferParameteriv(gl::ARRAY_BUFFER, gl::BUFFER_SIZE, &mut buffer_size);

            if vertices.len() <= buffer_size as usize {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (vertices.len() * size_of::<f32>()) as isize,
                    mem::transmute(vertices.as_ptr()),
                );
            } else {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    vertices.len() as isize * size_of::<f32>() as isize,
                    mem::transmute(vertices.as_ptr()),
                    gl::STATIC_DRAW,
                );
            }
            let attrib_size: usize = self.layout.iter().sum();
            self.vertex_count = (vertices.len() / attrib_size) as u32;
        }
    }

    pub fn reload_indices(&mut self, indices: &[u32]) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            let mut buffer_size: i32 = 0;
            gl::GetBufferParameteriv(gl::ELEMENT_ARRAY_BUFFER, gl::BUFFER_SIZE, &mut buffer_size);

            if indices.len() <= buffer_size as usize {
                gl::BufferSubData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    0,
                    (indices.len() * size_of::<u32>()) as isize,
                    mem::transmute(indices.as_ptr()),
                );
            } else {
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    indices.len() as isize * size_of::<u32>() as isize,
                    mem::transmute(indices.as_ptr()),
                    gl::STATIC_DRAW,
                );
            }

            self.index_count = (indices.len() / 3) as u32;
        }
    }*/

    /*pub fn load(&mut self, vertices: &[f32], indices: &[u32]) {
        self.index_count = indices.len() as u32;
        self.vertex_count = vertices.len() as u32;
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
    }*/

    fn create_vao() -> u32 {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            vao
        }
    }

    /*fn create_buffers(attributes: &[usize]) -> (u32, u32, u32) {
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
    }*/
}

/*impl<T, Q> Primitive<T, Q> {
    pub fn quad(width: f32, height: f32) -> Self {
        let vertices = Vertex2PT::quad(width, height);
        let vertices: Vec<_> = vertices
            .iter()
            .map(|v| [v.pos[0], v.pos[1], v.tex[0], v.tex[1]])
            .flatten()
            .collect();
        println!("Veritces: {:?}", vertices);

        let indices = TriangleIndex::quad(width, height);
        let indices: Vec<_> = indices
            .iter()
            .map(|TriangleIndex { triangle }| *triangle)
            .flatten()
            .collect();

        Self::new(&vertices, &[2, 2], &mut MeshIndices::Triangles(indices))
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

        Self::new(&vertices, &[3, 2], &mut MeshIndices::Points(indices))
    }
}*/
