use glad_gl::gl;

pub mod base_vertices;

pub enum PrimitiveType {
    Points,
    Lines,
    Triangles,
    Quads,
}

impl From<PrimitiveType> for u32 {
    fn from(value: PrimitiveType) -> Self {
        match value {
            PrimitiveType::Points => gl::POINTS,
            PrimitiveType::Lines => gl::LINES,
            PrimitiveType::Triangles => gl::TRIANGLES,
            PrimitiveType::Quads => gl::QUADS,
        }
    }
}

pub trait Shapely {
    type Attribute;
    fn gen_quad(width: f32, height: f32) -> Vec<Self::Attribute>;
}

pub trait Vertex {
    type VertexType: Vertex;
    fn declare_layout();
    fn size(len: usize) -> usize;
}

pub trait Index {
    type IndexType: Index;
    fn primitive_type() -> PrimitiveType;
    fn index_count(poly_count: usize) -> usize;
    fn size(len: usize) -> usize;
}
