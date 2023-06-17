pub trait BufferElement {
    type ElementType;
    fn layout() -> Vec<usize>;
}

#[repr(C)]
#[derive(Clone)]
pub struct PVertex(pub [f32; 3]);

#[repr(C)]
#[derive(Clone)]
pub struct PNVertex(pub [f32; 3], pub [f32; 3]);

#[repr(C)]
#[derive(Clone)]
pub struct PTNVertex(pub [f32; 3], pub [f32; 2], pub [f32; 3]);

#[repr(C)]
#[derive(Clone)]
pub struct PTVertex(pub [f32; 3], pub [f32; 2]);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct P2TVertex(pub [f32; 2], pub [f32; 2]);

impl BufferElement for P2TVertex {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![2, 2]
    }
}

impl BufferElement for PTVertex {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![3, 2]
    }
}

impl BufferElement for PTNVertex {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![3, 2, 3]
    }
}
