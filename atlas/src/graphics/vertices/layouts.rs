use vertex_macro_derive::BufferLayout;

use super::{
    attributes::{Attribute, AttributeTrait, Normal, Position, Position2D, TexCoords, Triangle},
    Shapely,
};

pub trait IndexGeometry: BufferElement {
    fn geometry() -> u32;
}

pub trait BufferElement {
    type ElementType;
    fn layout() -> Vec<usize>;
}

pub struct PVertex(Attribute<Position>);
pub struct PNVertex(Attribute<Position>, Attribute<Normal>);
pub struct PTNVertex(Attribute<Position>, Attribute<TexCoords>, Attribute<Normal>);
pub struct PTVertex(Attribute<Position>, Attribute<TexCoords>);
pub struct P2TVertex(Attribute<Position2D>, Attribute<TexCoords>);
pub struct TriangleGeometry(Attribute<Triangle>);

impl BufferElement for P2TVertex {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![Position2D::COUNT, TexCoords::COUNT]
    }
}

impl BufferElement for PTVertex {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![Position::COUNT, TexCoords::COUNT]
    }
}

impl BufferElement for PTNVertex {
    type ElementType = f32;

    fn layout() -> Vec<usize> {
        vec![Position::COUNT, TexCoords::COUNT, Normal::COUNT]
    }
}

impl IndexGeometry for TriangleGeometry {
    fn geometry() -> u32 {}
}

impl BufferElement for TriangleGeometry {
    type ElementType = u32;

    fn layout() -> Vec<usize> {
        todo!()
    }
}
