use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use nalgebra::{Vector2, Isometry2};
use ncollide::shape::ShapeHandle2;

use engine::Engine;
use engine::scene;
use engine::scene::{EntityAccessor, Scene, PhysicsWorld, PhysicsInteraction};
use engine::entity::{Entity, RenderInfo};
use engine::event::{Event, Handler as EventHandler, SysEvent};
use engine::graphics::SpriteAttrs;

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
}

impl Drop for PhysicsComp {
    fn drop(&mut self) {
        self.world.deref().remove(self.id);
    }
}

#[derive(Default)]
pub struct PhysicsData {
}

pub struct GraphicsComp {
    sprite: usize,
    data: SpriteAttrs,
}

impl GraphicsComp {
    pub fn new (sprite: usize) -> GraphicsComp {
        GraphicsComp {
            sprite: sprite,
            data: Default::default(),
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.data.translate(dx, dy);
    }

    pub fn set_rot(&mut self, t: f32) {
        self.data.set_rot(t);
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.data.set_pos(x, y);
    }

    pub fn get_data(&self) -> &SpriteAttrs {
        &self.data
    }

    pub fn get_render_info(&self) -> RenderInfo {
        RenderInfo {
            sprite: self.sprite,
            attrs: self.data,
        }
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

pub struct EventComp<E: Entity> {
    id: usize,
    handler: Rc<RefCell<EventHandler<E>>>,
}

impl<E: Entity> EventComp<E> {
    pub fn new(id: usize, handler: Rc<RefCell<EventHandler<E>>>) -> EventComp<E> {
        EventComp {
            id: id,
            handler: handler,
        }
    }

    pub fn update(time: u64) {
        // TODO: Update internal timers etc.
    }

    pub fn subscribe(&self, event: Event) {
        self.handler.deref().borrow_mut().subscribe(self.id.clone(), event);
    }

    pub fn unsubscribe(&self, event: Event) {
        self.handler.deref().borrow_mut().unsubscribe(self.id.clone(), event);
    }

    pub fn destroy_self(&self) {
        self.handler.deref().borrow_mut().enqueue_sys(SysEvent::Destroy(self.id));
    }

    pub fn create_entity(&self, f: fn(&Engine<E>) -> E) {
        self.handler.deref().borrow_mut().enqueue_sys(SysEvent::Create(f));
    }
}
