use std::rc::Rc;

use Engine;
use entity::Entity;
use scene::{EntityAccessor, World};
use util::HashSet;

pub struct WorldCompBuilder<E: Entity> {
    world : Rc<World<E>>,
    alias: Option<String>,
    tags: Vec<String>,
    id: usize,
}

impl<E: Entity> WorldCompBuilder<E> {
    pub fn new(engine: &Engine<E>) -> WorldCompBuilder<E> {
        let id = engine.scene.world.registry.borrow_mut().get_id();
        let world = engine.scene.world.clone();
        WorldCompBuilder {
            world: world,
            id: id,
            tags: Vec::new(),
            alias: None,
        }
    }

    pub fn with_id(engine: &Engine<E>, id: usize) -> WorldCompBuilder<E> {
        let world = engine.scene.world.clone();
        WorldCompBuilder {
            world: world,
            id: id,
            tags: Vec::new(),
            alias: None,
        }
    }

    pub fn with_alias(mut self, alias: String) -> WorldCompBuilder<E> {
        if !self.alias.is_none() {
            panic!("Entities should not be aliased more than once! Please use tags in this case.");
        }
        self.world.registry.borrow_mut().create_alias(alias.clone(), self.id);
        self.alias = Some(alias);
        self
    }

    pub fn with_tag(mut self, tag: String) -> WorldCompBuilder<E> {
        self.tags.push(tag.clone());
        self.world.registry.borrow_mut().tag_id(self.id, tag);
        self
    }

    pub fn with_tags(mut self, mut tags: Vec<String>) -> WorldCompBuilder<E> {
        for tag in tags.iter() {
            self.world.registry.borrow_mut().tag_id(self.id, tag.clone());
        }
        self.tags.append(&mut tags);
        self
    }

    pub fn build(self) -> WorldComp<E> {
        WorldComp {
            id: self.id,
            world: self.world,
            alias: self.alias,
            tags: self.tags,
        }
    }
}

pub struct WorldComp<E: Entity> {
    pub id: usize,
    world: Rc<World<E>>,
    alias: Option<String>,
    tags: Vec<String>,
}

use std::clone::Clone;

impl<E: Entity> Clone for WorldComp<E> {
    fn clone(&self) -> WorldComp<E> {
        let id = self.world.registry.borrow_mut().get_id();
        WorldComp {
            world: self.world.clone(),
            alias: self.alias.clone(),
            tags: self.tags.clone(),
            id: id
        }
    }
}


impl<E: Entity> WorldComp<E> {
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

    pub fn get_tagged(&self, tag: &String)  -> Option<HashSet<usize>> {
        match self.world.registry.borrow_mut().get_tagged(tag) {
            Some(tags) => Some(tags.clone()),
            None => None,
        }
    }

    pub fn remove_tagged(&mut self, tag: &String)  -> Option<HashSet<usize>> {
        self.world.registry.borrow_mut().remove_tagged(tag).clone()
    }
}

impl<E: Entity> Drop for WorldComp<E> {
    fn drop(&mut self) {
        self.world.registry.borrow_mut().return_id(self.id);
        for tag in self.tags.iter() {
            self.world.registry.borrow_mut().untag_id(&self.id, tag);
        }
        if let Some(ref alias) = self.alias {
            self.world.registry.borrow_mut().remove_alias(&alias);
        }
    }
}
