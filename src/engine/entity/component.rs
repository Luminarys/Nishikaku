use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use nalgebra::{Vector2, Isometry2, Translation};
use ncollide::shape::ShapeHandle2;

use engine::Engine;
use engine::scene;
use engine::scene::{EntityAccessor, Scene, PhysicsWorld, PhysicsInteraction};
use engine::entity::{Entity, RenderInfo};
use engine::event::{Event, Handler as EventHandler, SysEvent};
use engine::graphics::SpriteAttrs;

pub struct PhysicsComp {
    id: usize,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    world: Rc<PhysicsWorld>,
}

impl PhysicsComp {
    pub fn new<E: Entity>(entity_id: usize,
                          tag: String,
                          position: Vector2<f32>,
                          shape: ShapeHandle2<f32>,
                          interactivity: PhysicsInteraction,
                          scene: &Scene<E>)
                          -> PhysicsComp {
        let id = scene.physics.deref().add(position,
                                           shape,
                                           interactivity,
                                           Rc::new(PhysicsData::new(entity_id, tag)));
        PhysicsComp {
            id: id,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            world: scene.physics.clone(),
        }
    }

    pub fn translate(&self, delta: Vector2<f32>) {
        let pos = self.get_pos().append_translation(&delta);
        self.set_pos(pos);
    }

    pub fn get_pos(&self) -> Isometry2<f32> {
        self.world.deref().get_pos(self.id).unwrap()
    }

    pub fn set_pos(&self, pos: Isometry2<f32>) {
        self.world.deref().set_pos(self.id, pos);
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.translate(self.velocity * dt);
    }
}

impl Drop for PhysicsComp {
    fn drop(&mut self) {
        self.world.deref().remove(self.id);
    }
}

#[derive(Clone, Default)]
pub struct PhysicsData {
    pub entity_id: usize,
    pub tag: String,
}

impl PhysicsData {
    pub fn new(entity_id: usize, tag: String) -> PhysicsData {
        PhysicsData {
            entity_id: entity_id,
            tag: tag,
        }
    }
}

pub struct GraphicsComp {
    sprite: usize,
    data: SpriteAttrs,
}

impl GraphicsComp {
    pub fn new(sprite: usize) -> GraphicsComp {
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

    pub fn get_pos(&self) -> (f32, f32) {
        self.data.get_pos()
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

    pub fn get_entity(&self, id: &usize) -> EntityAccessor<E> {
        self.world.deref().get_entity(id)
    }
}

impl<E: Entity> Drop for WorldComp<E> {
    fn drop(&mut self) {
        self.world.registry.borrow_mut().return_id(self.id);
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

    pub fn update(time: f32) {
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

    pub fn dispatch(&self, event: Event) {
        self.handler.deref().borrow_mut().enqueue_all(event);
    }

    pub fn dispatch_to(&self, id: usize, event: Event) {
        self.handler.deref().borrow_mut().enqueue_specific(id, event);
    }

    pub fn create_entity(&self, f: Box<Fn(&Engine<E>) -> E>) {
        self.handler.deref().borrow_mut().enqueue_sys(SysEvent::Create(f));
    }
}
