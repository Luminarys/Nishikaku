use std::rc::Rc;

use engine::entity::Entity;
use engine::scene::{Scene, EntityAccessor, World};

pub struct WorldComp<E: Entity> {
    world: Rc<World<E>>,
    pub id: usize,
}

impl<E: Entity> WorldComp<E> {
    pub fn new(scene: &Scene<E>) -> WorldComp<E> {
        let id = scene.world.registry.borrow_mut().get_id();
        WorldComp {
            id: id,
            world: scene.world.clone(),
        }
    }

    pub fn with_alias(scene: &Scene<E>, alias: String) -> WorldComp<E> {
        let id = scene.world.registry.borrow_mut().get_id();
        scene.world.registry.borrow_mut().create_alias(alias, id.clone());
        WorldComp {
            id: id,
            world: scene.world.clone(),
        }
    }

    pub fn find_aliased_entity_id(&self, alias: &String) -> Option<usize> {
        match self.world.registry.borrow_mut().get_aliased_id(alias) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn find_aliased_entity_alias(&self, id: &usize) -> Option<String> {
        match self.world.registry.borrow_mut().get_aliased_string(id) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    pub fn get_entity(&self, id: &usize) -> EntityAccessor<E> {
        self.world.get_entity(id)
    }
}

impl<E: Entity> Drop for WorldComp<E> {
    fn drop(&mut self) {
        self.world.registry.borrow_mut().return_id(self.id);
    }
}
