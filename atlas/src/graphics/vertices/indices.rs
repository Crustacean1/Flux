use glad_gl::gl;

pub trait IndexGeometry {
    const GEOMETRY: u32;
}

#[derive(Clone, Debug)]
pub struct TriangleGeometry(pub [u32; 3]);

#[derive(Clone, Debug)]
pub struct TriangleStripGeometry(pub [u32; 3]);

#[derive(Clone, Debug)]
pub struct LineGeometry(pub [u32; 2]);

#[derive(Clone, Debug)]
pub struct PointGeometry(pub [u32; 1]);

impl IndexGeometry for TriangleGeometry {
    const GEOMETRY: u32 = gl::TRIANGLES;
}

impl IndexGeometry for LineGeometry {
    const GEOMETRY: u32 = gl::LINES;
}

impl IndexGeometry for PointGeometry {
    const GEOMETRY: u32 = gl::POINTS;
}

impl IndexGeometry for TriangleStripGeometry {
    const GEOMETRY: u32 = gl::TRIANGLE_STRIP;
}
