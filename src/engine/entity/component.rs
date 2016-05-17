use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use nalgebra::Vector2;
use ncollide::shape::ShapeHandle2;

use engine::scene;
use engine::scene::{EntityAccessor, Scene, PhysicsWorld, PhysicsInteraction};
use engine::entity::Entity;

pub struct Physics {
    id: usize,
    world: Rc<PhysicsWorld>,
}

impl Physics {
    pub fn new<E: Entity>(id: usize,
                          position: Vector2<f32>,
                          shape: ShapeHandle2<f32>,
                          interactivity: PhysicsInteraction,
                          scene: &Scene<E>)
                          -> Physics {
        scene.physics.deref().add(id, position, shape, interactivity, Default::default());
        Physics {
            id: id,
            world: scene.physics.clone(),
        }
    }

    // pub fn flush_flags(&self) -> Vec<PhysicsFlags> {
    //    // mem::replace(&mut self.data.deref().borrow_mut().flags, Default::default())
    // }
}

impl Drop for Physics {
    fn drop(&mut self) {
        self.world.deref().remove(self.id);
    }
}

#[derive(Default)]
pub struct PhysicsData {
    pub flags: Vec<PhysicsFlags>,
}

pub enum PhysicsFlags {
    Collision {
        id: usize,
    },
}

pub struct Graphics {
    texture: String,
}

impl Graphics {
    pub fn new(texture: String) -> Graphics {
        Graphics { texture: texture }
    }
}

pub struct World<E: Entity> {
    world: Rc<scene::World<E>>,
}

impl<E: Entity> World<E> {
    fn new(scene: &Scene<E>) -> World<E> {
        World { world: scene.world.clone() }
    }

    fn get_entity(&self, id: &usize) -> EntityAccessor<E> {
        self.world.deref().get_entity(id)
    }
}
