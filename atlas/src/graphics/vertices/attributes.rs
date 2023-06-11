use std::mem::size_of;

pub trait AttributeTrait {
    const COUNT: usize;
    type AttributeType;
}

pub struct Attribute<T: AttributeTrait>
where
    [T::AttributeType; T::COUNT]: Sized,
{
    pub data: [T::AttributeType; T::COUNT],
}

impl<T: AttributeTrait> Attribute<T>
where
    [T::AttributeType; T::COUNT]: Sized,
{
    pub fn count() -> usize {
        T::COUNT
    }

    pub fn size() -> usize {
        size_of::<[T::AttributeType; T::COUNT]>()
    }
}

pub struct Triangle;
impl AttributeTrait for Triangle {
    const COUNT: usize = 3;
    type AttributeType = u32;
}

pub struct Point;
impl AttributeTrait for Point {
    const COUNT: usize = 1;
    type AttributeType = u32;
}

pub struct Position;
impl AttributeTrait for Position {
    const COUNT: usize = 3;
    type AttributeType = f32;
}

pub struct Position2D;
impl AttributeTrait for Position2D {
    const COUNT: usize = 2;
    type AttributeType = f32;
}

pub struct TexCoords;
impl AttributeTrait for TexCoords {
    const COUNT: usize = 2;
    type AttributeType = f32;
}

pub struct Normal;
impl AttributeTrait for Normal {
    const COUNT: usize = 3;
    type AttributeType = f32;
}
