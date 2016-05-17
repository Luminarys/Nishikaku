use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use nalgebra::{Vector2, Isometry2};
use ncollide::shape::ShapeHandle2;

use engine::scene;
use engine::scene::{EntityAccessor, Scene, PhysicsWorld, PhysicsInteraction};
use engine::entity::Entity;
use engine::event::{Event, Handler as EventHandler};

pub struct PhysicsComp {
    id: usize,
    world: Rc<PhysicsWorld>,
}

impl PhysicsComp {
    pub fn new<E: Entity>(id: usize,
                          position: Vector2<f32>,
                          shape: ShapeHandle2<f32>,
                          interactivity: PhysicsInteraction,
                          scene: &Scene<E>)
                          -> PhysicsComp {
        scene.physics.deref().add(id, position, shape, interactivity, Default::default());
        PhysicsComp {
            id: id,
            world: scene.physics.clone(),
        }
    }

    pub fn get_pos(&self) -> Option<Isometry2<f32>> {
        self.world.deref().get_pos(self.id)
    }

    pub fn set_pos(&self, pos: Isometry2<f32>) {
        self.world.deref().set_pos(self.id, pos);
    }

    // pub fn flush_flags(&self) -> Vec<PhysicsFlags> {
    //    // mem::replace(&mut self.data.deref().borrow_mut().flags, Default::default())
    // }
}

impl Drop for PhysicsComp {
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

pub struct GraphicsComp {
    texture: String,
}

impl GraphicsComp {
    pub fn new(texture: String) -> GraphicsComp {
        GraphicsComp { texture: texture }
    }
}

pub struct WorldComp<E: Entity> {
    world: Rc<scene::World<E>>,
}

impl<E: Entity> WorldComp<E> {
    fn new(scene: &Scene<E>) -> WorldComp<E> {
        WorldComp { world: scene.world.clone() }
    }

    fn get_entity(&self, id: &usize) -> EntityAccessor<E> {
        self.world.deref().get_entity(id)
    }
}

pub struct EventComp {
    id: usize,
    handler: Rc<RefCell<EventHandler>>,
}

impl EventComp {
    pub fn new(id: usize, handler: Rc<RefCell<EventHandler>>) -> EventComp {
        EventComp {
            id: id,
            handler: handler
        }
    }

    pub fn destroy_self(&self) {
        self.handler.deref().borrow_mut().enqueue_specific(self.id, Event::Destroy);
    }
}
