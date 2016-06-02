use nalgebra::Vector2;
use ncollide::shape::{Ball, Cuboid, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use glium::glutin::VirtualKeyCode;
use std::rc::Rc;

use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::scene::PhysicsInteraction;
use engine::entity::RenderInfo;
use game::object::Object;

pub struct Player {
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    slowdown: f32,
}

impl Player {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let g = GraphicsComp::new(engine.graphics.clone(), 1);
        let e = EventComp::new(w.id, engine.events.clone());

        let p = PhysicsComp::new(w.id,
                                 String::from("collision_box"),
                                 Vector2::new(0.0, 0.0),
                                 ShapeHandle2::new(Cuboid::new(Vector2::new(25.0, 50.0))),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Contacts(0.1),
                                 &engine.scene);
        let mut pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        pg.screen_lock((25.0, 50.0));
        Object::Player(Player {
            pg: pg,
            ev: e,
            world: w,
            slowdown: 1.0,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                self.ev.subscribe(Event::KeyInput(InputState::Pressed, VirtualKeyCode::A));
                let _ = self.world.get_entity(&100);
            }
            Event::Update(t) => {
                self.pg.update(t);
                self.ev.update(t);
            }
            Event::Render => {
                self.pg.render();
            }
            Event::KeyInput(InputState::Pressed, VirtualKeyCode::Up) |
            Event::KeyInput(InputState::Released, VirtualKeyCode::Down) => {
                self.pg.velocity += Vector2::new(0.0, 100.0) * self.slowdown;
            }
            Event::KeyInput(InputState::Pressed, VirtualKeyCode::Left) |
            Event::KeyInput(InputState::Released, VirtualKeyCode::Right) => {
                self.pg.velocity += Vector2::new(-100.0, 0.0) * self.slowdown;
            }
            Event::KeyInput(InputState::Pressed, VirtualKeyCode::Right) |
            Event::KeyInput(InputState::Released, VirtualKeyCode::Left) => {
                self.pg.velocity += Vector2::new(100.0, 0.0) * self.slowdown;
            }
            Event::KeyInput(InputState::Pressed, VirtualKeyCode::Down) |
            Event::KeyInput(InputState::Released, VirtualKeyCode::Up) => {
                self.pg.velocity += Vector2::new(0.0, -100.0) * self.slowdown;
            }
            Event::KeyInput(InputState::Pressed, VirtualKeyCode::LShift) => {
                self.slowdown = 0.5;
                self.pg.velocity *= self.slowdown;
            }
            Event::KeyInput(InputState::Released, VirtualKeyCode::LShift) => {
                self.pg.velocity *= 1.0 / self.slowdown;
                self.slowdown = 1.0;
            }
            Event::Timer(1) => {
                self.shoot_bullet();
            }
            Event::KeyInput(InputState::Pressed, VirtualKeyCode::Z) => {
                self.shoot_bullet();
                self.ev.set_repeating_timer(1, 0.08);
            }
            Event::KeyInput(InputState::Released, VirtualKeyCode::Z) => {
                self.ev.remove_timer(1);
            }
            _ => {}
        };
    }

    fn shoot_bullet(&self) {
        let pos = self.pg.get_pos();
        self.ev.create_entity(Box::new(move |engine| Bullet::new_at_pos(engine, pos)));
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.pg.get_render_info())
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}

pub struct Bullet {
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Bullet {
    pub fn new_at_pos(engine: &Engine<Object>, pos: (f32, f32)) -> Object {
        let mut g = GraphicsComp::new(engine.graphics.clone(), 2);
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        let scaler = engine.scene.physics.scaler;
        let p = PhysicsComp::new(w.id,
                                 String::from("bullet"),
                                 Vector2::new(pos.0, pos.1),
                                 ShapeHandle2::new(Ball::new(5.0)),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Contacts(0.0),
                                 &engine.scene);
        g.translate(pos.0 / scaler, pos.1 / scaler);
        let mut pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        pg.velocity = Vector2::new(0.0, 200.0);
        Object::PlayerBullet(Bullet {
            pg: pg,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {}
            Event::Update(t) => {
                self.pg.update(t);
            }
            Event::Exiting => {
                self.ev.destroy_self();
            }
            Event::Render => {
                self.pg.render();
            }
            _ => {}
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.pg.get_render_info())
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
