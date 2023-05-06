use super::{Index, PrimitiveType, Shapely, Vertex};
use macros::Vertex;

#[derive(Vertex)]
pub struct Vertex2P {
    pos: [f32; 2],
}

impl Shapely for Vertex2P {
    type Attribute = Vertex2P;
    fn gen_quad(width: f32, height: f32) -> Vec<Vertex2P> {
        let (width, height) = (width * 0.5, height * 0.5);
        vec![
            Vertex2P {
                pos: [-width, -height],
            },
            Vertex2P {
                pos: [width, -height],
            },
            Vertex2P {
                pos: [width, height],
            },
            Vertex2P {
                pos: [-width, height],
            },
        ]
    }
}

pub struct TriangleIndex {
    triangle: [u32; 3],
}

impl Index for TriangleIndex {
    type IndexType = TriangleIndex;

    fn primitive_type() -> PrimitiveType {
        PrimitiveType::Triangles
    }
}

impl Shapely for TriangleIndex {
    type Attribute = TriangleIndex;

    fn gen_quad(_width: f32, _height: f32) -> Vec<TriangleIndex> {
        vec![]
    }
}
