pub trait IndexGeometry: BufferElement {
    fn geometry() -> u32;
}

pub trait BufferElement {
    type ElementType;
    fn layout() -> Vec<usize>;
}

#[derive(Clone)]
pub struct PVertex(pub [f32; 3]);
#[derive(Clone)]
pub struct PNVertex(pub [f32; 3], pub [f32; 3]);
#[derive(Clone)]
pub struct PTNVertex(pub [f32; 3], pub [f32; 2], pub [f32; 3]);
#[derive(Clone)]
pub struct PTVertex(pub [f32; 3], pub [f32; 2]);
#[derive(Clone)]
pub struct P2TVertex(pub [f32; 2], pub [f32; 2]);
#[derive(Clone)]
pub struct TriangleGeometry(pub [u32; 3]);

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

impl BufferElement for TriangleGeometry {
    type ElementType = u32;

    fn layout() -> Vec<usize> {
        vec![3]
    }
}
