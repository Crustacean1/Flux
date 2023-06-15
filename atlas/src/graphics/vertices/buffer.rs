use std::marker::PhantomData;

use super::layouts::BufferElement;

#[derive(Clone)]
pub struct Buffer<T: BufferElement> {
    layout: Vec<usize>,
    size: usize,
    buffer_handle: u32,
    buffer_target: u32,
    data_type: PhantomData<T>,
}

impl<T: BufferElement> Buffer<T> {
    pub fn build(data: &[T]) -> Self {
        Self {
            layout: T::layout(),
            size: 0,
            buffer_handle: 0,
            buffer_target: 0,
            data_type: PhantomData::default(),
        }
    }

    pub fn load(&mut self, data: &[T]) {
        unsafe {
            //gl::
            //
        }
    }

    pub fn count(&self) -> u32 {
        todo!();
    }
}
