use super::layouts::BufferElement;

pub struct Buffer<Q: BufferElement> {
    size: usize,
    buffer_handle: u32,
    buffer_target: u32,
    attribute: Q,
}

impl<T: BufferElement> Buffer<T> {
    pub fn new(data: &[T]) -> Self {
        Self {
            size: 0,
            buffer_handle: 0,
            buffer_target: 0,
            attribute: T::new(),
        }
    }
}

impl<T: BufferElement> From<&[T::ElementType]> for Buffer<T> {
    fn from(value: &[T::ElementType]) -> Self {
        let stride = T::layout().iter().sum();
        let end = (value.len() / stride) * stride;
        Self::new(value[0..end])
    }
}
