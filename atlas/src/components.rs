pub mod shape_renderer;

pub trait Component {
    fn entity_id(&self) -> usize;
}
