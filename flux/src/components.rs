mod shape_renderer;
mod transform;

use atlas::allocator::ComponentAllocator;
use atlas::components::ComponentType;
use macros::declare_components;
use self::{shape_renderer::ShapeRenderer, transform::Transform};
use atlas::allocator::GenericComponentAllocator;

declare_components!((ShapeRenderer, Transform));
