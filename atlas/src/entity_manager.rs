use std::any::Any;

use crate::{components::transform::Transform, game_entities::GameEntity};

pub type ComponentIterator<T> = Box<dyn Iterator<Item = T>>;

pub enum ComponentEvent<T> {
    AddComponent(T),
    RemoveComponent(usize),
}

pub trait ComponentIteratorGenerator<'a, T> {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = T> + 'a>;
}

pub trait ComponentMutIteratorGenerator<'a, T> {
    fn get_mut_view(&'a mut self) -> Box<dyn Iterator<Item = T> + 'a>;
}

pub trait EntityManagerTrait<T> {
    fn add_entity(&mut self, entity: T) -> usize;
    fn remove_entity(&mut self, entity: usize);
}

pub struct EntityManager {
    entities: Vec<Box<dyn Any>>,

    global_entity_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            global_entity_id: 0,
            entities: vec![],
        }
    }

    pub fn add<T: 'static>(&mut self, entity: T) -> usize {
        self.add_at(entity, Transform::new())
    }

    pub fn add_at<T: 'static>(&mut self, entity: T, transform: Transform) -> usize {
        let id = self.global_entity_id;
        let new_entity = GameEntity::new(entity, transform, &mut self.global_entity_id);

        if let Some(entity_container) = self
            .entities
            .iter_mut()
            .find_map(|container| container.downcast_mut::<Vec<GameEntity<T>>>())
        {
            entity_container.push(new_entity);
        } else {
            self.entities.push(Box::new(vec![new_entity]));
        }
        id
    }

    pub fn remove<T: 'static>(&mut self, id: usize) {
        self.entities.iter_mut().find_map(|container| {
            container
                .downcast_mut::<Vec<GameEntity<T>>>()
                .map(|container| {
                    container
                        .iter()
                        .position(|e| e.id == id)
                        .map(|pos| container.remove(pos));
                })
        });
    }

    pub fn get<T: 'static>(&self, id: usize) -> Option<&GameEntity<T>> {
        self.entities.iter().find_map(|container| {
            let container = container.downcast_ref::<Vec<GameEntity<T>>>()?;
            container.iter().find(|e| e.id == id)
        })
    }

    pub fn get_mut<T: 'static>(&mut self, id: usize) -> Option<&mut GameEntity<T>> {
        self.entities.iter_mut().find_map(|container| {
            let container = container.downcast_mut::<Vec<GameEntity<T>>>()?;
            container.iter_mut().find(|e| e.id == id)
        })
    }

    pub fn iter<T: 'static>(&self) -> impl Iterator<Item = &GameEntity<T>> {
        self.entities
            .iter()
            .find_map(|container| Some(container.downcast_ref::<Vec<GameEntity<T>>>()?.iter()))
            .unwrap_or(std::slice::Iter::default())
    }

    pub fn iter_mut<T: 'static>(&mut self) -> impl Iterator<Item = &mut GameEntity<T>> {
        self.entities
            .iter_mut()
            .find_map(|container| Some(container.downcast_mut::<Vec<GameEntity<T>>>()?.iter_mut()))
            .unwrap_or(std::slice::IterMut::default())
    }
}
