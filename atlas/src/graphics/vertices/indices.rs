use glad_gl::gl;

pub trait IndexGeometry {
    const GEOMETRY: u32;
}

#[derive(Clone, Debug)]
pub struct TriangleGeometry(pub [u32; 3]);

impl IndexGeometry for TriangleGeometry {
    const GEOMETRY: u32 = gl::TRIANGLES;
}
