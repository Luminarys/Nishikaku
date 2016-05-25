use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use nalgebra::{Vector2, Isometry2, Translation};
use ncollide::shape::ShapeHandle2;
use ncollide::world::GeometricQueryType;

use engine::Engine;
use engine::scene;
use engine::scene::{EntityAccessor, Scene, PhysicsWorld, PhysicsInteraction};
use engine::entity::{Entity, RenderInfo};
use engine::event::{Event, Handler as EventHandler, SysEvent};
use engine::graphics::SpriteAttrs;

/// Combined physics and graphics component,
/// synchronizes their motion
pub struct PGComp {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    physics: Vec<PhysicsComp>,
    graphics: GraphicsComp,
    world: Rc<PhysicsWorld>,
    scaler: f32,
}

impl PGComp {
    pub fn new(graphics: GraphicsComp, physics: Vec<PhysicsComp>, world: Rc<PhysicsWorld>) -> PGComp {
        PGComp {
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            graphics: graphics,
            physics: physics,
            scaler: world.scaler.clone(),
            world: world,
        }
    }

    pub fn get_render_info(&self) -> RenderInfo {
        self.graphics.get_render_info()
    }

    pub fn translate(&mut self, delta: Vector2<f32>) {
        for comp in self.physics.iter() {
            comp.translate(delta);
        }
        self.graphics.translate(delta.x/self.scaler, delta.y/self.scaler);
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let (x, y) = self.graphics.get_pos();
        (x * self.scaler, y * self.scaler)
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        let (new_x, new_y) = (pos.0/self.scaler, pos.1/self.scaler);
        let (old_x, old_y) = self.get_pos();
        let (delta_x, delta_y) = ((new_x - old_x) * self.scaler, (new_y - old_y) * self.scaler);
        let delta = Vector2::new(delta_x, delta_y);
        for comp in self.physics.iter() {
            comp.translate(delta);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let new_vel = self.velocity * dt;
        self.translate(new_vel);
    }
}

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
                          query: GeometricQueryType<f32>,
                          scene: &Scene<E>)
                          -> PhysicsComp {
        let id = scene.physics.deref().add(position,
                                           shape,
                                           interactivity,
                                           query,
                                           Rc::new(PhysicsData::new(entity_id, tag)));
        println!("Creating new physics entity with id: {}", id);
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
        println!("Removing physics entity with id: {}", self.id);
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

impl<E: Entity> Drop for EventComp<E> {
    fn drop(&mut self) {
        self.handler.deref().borrow_mut().unsubscribe_all(self.id);
    }
}
