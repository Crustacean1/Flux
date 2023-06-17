use std::{marker::PhantomData, mem};

use glad_gl::gl;

#[derive(Clone)]
pub struct Buffer<T> {
    count: u32,
    buffer_handle: u32,
    buffer_target: BufferTarget,
    data_type: PhantomData<T>,
}

#[derive(Clone)]
pub enum BufferTarget {
    Vertex,
    Index,
}

impl BufferTarget {
    pub fn to_gl(&self) -> u32 {
        match self {
            BufferTarget::Vertex => gl::ARRAY_BUFFER,
            BufferTarget::Index => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl<T> Buffer<T> {
    pub fn build(data: &[T], buffer_target: BufferTarget) -> Self {
        unsafe {
            let mut buffer_handle: u32 = 0;
            let size = (data.len() * mem::size_of::<T>()) as u32;

            gl::GenBuffers(1, &mut buffer_handle);
            gl::BindBuffer(buffer_target.to_gl(), buffer_handle);
            gl::BufferData(
                buffer_target.to_gl(),
                size as isize,
                mem::transmute(data.as_ptr()),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(buffer_target.to_gl(), 0);

            Self {
                count: data.len() as u32,
                buffer_handle,
                buffer_target,
                data_type: PhantomData::default(),
            }
        }
    }

    pub fn reload(&mut self, data: &[T]) {
        unsafe {
            let buffer_size = self.get_buffer_size();

            let memory = mem::transmute(data.as_ptr());
            let target = self.buffer_target.to_gl();
            let size = (data.len() * mem::size_of::<T>()) as isize;

            gl::BindBuffer(target, self.buffer_handle);
            if data.len() as u32 > buffer_size {
                gl::BufferData(target, size, memory, gl::STATIC_DRAW);
            } else {
                gl::BufferSubData(target, 0, size, memory);
            }
            gl::BindBuffer(target, 0);

            self.count = data.len() as u32;
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_target.to_gl(), self.buffer_handle);
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    fn get_buffer_size(&self) -> u32 {
        unsafe {
            let mut buffer_size: u32 = 0;
            gl::GetBufferParameteriv(
                self.buffer_target.to_gl(),
                gl::BUFFER_SIZE,
                mem::transmute(&mut buffer_size),
            );
            buffer_size
        }
    }
}
