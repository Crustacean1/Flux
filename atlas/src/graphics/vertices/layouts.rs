use std::mem;

use glad_gl::gl;

pub enum Attribute {
    Float(usize),
    UnsignedInt(usize),
}

impl Attribute {
    pub fn to_gl(&self) -> u32 {
        match self {
            Attribute::Float(_) => gl::FLOAT,
            Attribute::UnsignedInt(_) => gl::UNSIGNED_INT,
        }
    }

    pub fn count(&self) -> usize {
        match *self {
            Attribute::Float(size) => size,
            Attribute::UnsignedInt(size) => size,
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            Attribute::Float(count) => count * mem::size_of::<f32>(),
            Attribute::UnsignedInt(count) => count * mem::size_of::<u32>(),
        }
    }
}

pub trait BufferElement {
    fn layout() -> Vec<Attribute>;
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
#[derive(Clone, Debug)]
pub struct PTVertex(pub [f32; 3], pub [f32; 2]);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct P2TVertex(pub [f32; 2], pub [f32; 2]);

impl BufferElement for P2TVertex {
    fn layout() -> Vec<Attribute> {
        vec![Attribute::Float(2), Attribute::Float(2)]
    }
}

impl BufferElement for PTVertex {
    fn layout() -> Vec<Attribute> {
        vec![Attribute::Float(3), Attribute::Float(2)]
    }
}

impl BufferElement for PTNVertex {
    fn layout() -> Vec<Attribute> {
        vec![
            Attribute::Float(3),
            Attribute::Float(2),
            Attribute::Float(3),
        ]
    }
}
