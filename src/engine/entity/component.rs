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
    screen_locked: bool,
    half_widths: (f32, f32),
}

impl PGComp {
    pub fn new(graphics: GraphicsComp,
               physics: Vec<PhysicsComp>,
               world: Rc<PhysicsWorld>)
               -> PGComp {
        PGComp {
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            graphics: graphics,
            physics: physics,
            scaler: world.scaler.clone(),
            world: world,
            screen_locked: false,
            half_widths: (0.0, 0.0),
        }
    }

    pub fn screen_lock(&mut self, half_widths: (f32, f32)) {
        self.screen_locked = true;
        self.half_widths = (half_widths.0 / self.scaler, half_widths.1 / self.scaler);
    }

    pub fn get_render_info(&self) -> RenderInfo {
        self.graphics.get_render_info()
    }

    pub fn translate(&mut self, delta: Vector2<f32>) {
        self.graphics.translate(delta.x / self.scaler, delta.y / self.scaler);
        for comp in self.physics.iter() {
            comp.translate(delta);
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let (x, y) = self.graphics.get_pos();
        (x * self.scaler, y * self.scaler)
    }

    pub fn get_gfx_pos(&self) -> (f32, f32) {
        self.graphics.get_pos()
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        let (new_x, new_y) = (pos.0 / self.scaler, pos.1 / self.scaler);
        let (old_x, old_y) = self.get_pos();
        let (delta_x, delta_y) = ((new_x - old_x) * self.scaler, (new_y - old_y) * self.scaler);
        let delta = Vector2::new(delta_x, delta_y);
        self.graphics.set_pos(new_x, new_y);
        for comp in self.physics.iter() {
            comp.translate(delta);
        }
    }

    pub fn set_gfx_pos(&mut self, pos: (f32, f32)) {
        let converted_pos = (pos.0 * self.scaler, pos.1 * self.scaler);
        self.set_pos(converted_pos);
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let delta = self.velocity * dt;
        self.translate(delta);

        if self.screen_locked {
            let new_pos = self.get_gfx_pos();
            let mut actual_pos = new_pos;
            if new_pos.0 > 1.0 - self.half_widths.0 {
                actual_pos.0 = 1.0 - self.half_widths.0;
            } else if new_pos.0 < -1.0 + self.half_widths.0 {
                actual_pos.0 = -1.0 + self.half_widths.0;
            }

            if new_pos.1 > 1.0 - self.half_widths.1 {
                actual_pos.1 = 1.0 - self.half_widths.1;
            } else if new_pos.1 < -1.0 + self.half_widths.1 {
                actual_pos.1 = -1.0 + self.half_widths.1;
            }
            self.set_gfx_pos(actual_pos);
        }
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

pub struct Timer {
    pub id: usize,
    pub repeat: bool,
    amount: f32,
    left: f32,
}

impl Timer {
    pub fn new(id: usize, amount: f32, repeat: bool) -> Timer {
        Timer {
            id: id,
            repeat: repeat,
            left: amount,
            amount: amount,
        }
    }

    pub fn update(&mut self, time: f32) -> bool {
        self.left -= time;
        if self.repeat && self.left <= 0.0 {
            self.left = self.amount + self.left;
            true
        } else {
            self.left <= 0.0
        }
    }
}

pub struct EventComp<E: Entity> {
    id: usize,
    handler: Rc<RefCell<EventHandler<E>>>,
    timers: Vec<Timer>,
}

impl<E: Entity> EventComp<E> {
    pub fn new(id: usize, handler: Rc<RefCell<EventHandler<E>>>) -> EventComp<E> {
        EventComp {
            id: id,
            handler: handler,
            timers: vec![],
        }
    }

    pub fn update(&mut self, time: f32) {
        let mut expired = vec![];
        for (i, timer) in self.timers.iter_mut().enumerate() {
            if timer.update(time) {
                // self.dispatch_to(self.id, Event::Timer(timer.id));
                self.handler.deref().borrow_mut().enqueue_specific(self.id, Event::Timer(timer.id));
                if !timer.repeat {
                    expired.push(i);
                }
            }
        }
        for i in expired {
            self.timers.remove(i);
        }
    }

    pub fn set_timer(&mut self, id: usize, amount: f32) {
        self.timers.push(Timer::new(id, amount, false));
    }

    pub fn set_repeating_timer(&mut self, id: usize, amount: f32) {
        self.timers.push(Timer::new(id, amount, true));
    }

    pub fn remove_timer(&mut self, id: usize) {
        match self.timers.iter().position(|timer| timer.id == id) {
            Some(pos) => { self.timers.remove(pos); },
            None => { }
        }
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
