pub mod button_handler;
pub mod button_trigger;
pub mod camera;
pub mod controller;
pub mod mesh_renderer;
pub mod particle_emitter;
pub mod physical_body;
pub mod sprite_renderer;
pub mod skybox_renderer;
pub mod text_renderer;
pub mod transform;
pub mod particle_renderer;
pub mod collider;

pub trait ComponentType {
    fn component_type_id() -> usize;
}

pub struct Component<T> {
    pub entity_id: usize,
    pub component: T,
}

impl<T> Component<T> {
    pub fn new(entity_id: usize, component: T) -> Self {
        Component::<T> {
            entity_id,
            component,
        }
    }
}
