use atlas::components::{shape_renderer::ShapeRenderer, transform::Transform, Component};

pub trait ComponentManager<T> {
    fn components_mut(&mut self) -> std::slice::IterMut<Component<T>>;

    fn add_component(&mut self, entity: usize, component: T) -> Option<()>;
}

pub trait EntityManager {
    fn add_entity(&mut self) -> usize;
    fn remove_entity(&mut self, entity_id: usize) -> Option<()>;
}

pub trait ComponentIterator<'a, T> {
    fn use_components(&'a mut self, run: &dyn Fn(T) -> ());
}

pub struct ComponentAggregator {
    entities: Vec<usize>,
    entity_count: usize,

    transforms: Vec<Component<Transform>>,
    shape_renderers: Vec<Component<ShapeRenderer>>,

    // Iterators
    shape_and_transform: Vec<(usize, usize)>,
}

impl ComponentAggregator {
    pub fn new() -> Self {
        ComponentAggregator {
            entities: vec![],
            entity_count: 0,
            transforms: vec![],
            shape_renderers: vec![],
            shape_and_transform: vec![],
        }
    }
}

impl EntityManager for ComponentAggregator {
    fn add_entity(&mut self) -> usize {
        let entity_id = self.entity_count;
        self.entities.push(entity_id);
        self.entity_count += 1;
        entity_id
    }
    fn remove_entity(&mut self, entity_id: usize) -> Option<()> {
        None
    }
}

impl ComponentManager<Transform> for ComponentAggregator {
    fn components_mut(&mut self) -> std::slice::IterMut<Component<Transform>> {
        self.transforms.iter_mut()
    }

    fn add_component(&mut self, entity: usize, component: Transform) -> Option<()> {
        self.transforms.push(Component::new(entity, component));
        Some(())
    }
}

impl ComponentManager<ShapeRenderer> for ComponentAggregator {
    fn components_mut(&mut self) -> std::slice::IterMut<Component<ShapeRenderer>> {
        self.shape_renderers.iter_mut()
    }

    fn add_component(&mut self, entity: usize, component: ShapeRenderer) -> Option<()> {
        self.shape_renderers.push(Component::new(entity, component));
        Some(())
    }
}

impl<'a> ComponentIterator<'a, (&'a Component<ShapeRenderer>, &'a Component<Transform>)>
    for ComponentAggregator
{
    fn use_components(
        &'a mut self,
        run: &dyn Fn((&'a Component<ShapeRenderer>, &'a Component<Transform>)) -> (),
    ) {
        self.shape_and_transform
            .iter()
            .for_each(|&(shape_i, transform_i)| {
                let shape = &self.shape_renderers[shape_i];
                let transform = &self.transforms[transform_i];
                run((shape, transform));
            })
    }
}
