pub mod shape_renderer;
pub mod transform;

use self::{shape_renderer::ShapeRenderer, transform::Transform};
use atlas::allocator::ComponentAllocator;
use atlas::allocator::GenericComponentAllocator;
use atlas::components::{Component, ComponentType};
use macros::declare_components;

declare_components!((ShapeRenderer, Transform));
