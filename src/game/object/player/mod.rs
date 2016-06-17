use nalgebra::Vector2;
use nalgebra::angle_between;
use ncollide::shape::{Ball, Cuboid, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use ncollide::query::Proximity;
use glium::glutin::VirtualKeyCode;
use std::rc::Rc;

use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::scene::PhysicsInteraction;
use engine::entity::RenderInfo;
use game::object::Object;
use game::object::level::pattern::{Angle, Pattern, PatternBuilder};

pub struct Player {
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    pat: Pattern,
    slowdown: f32,
}

impl Player {
    pub fn new(engine: &Engine<Object>) -> Object {
        use ncollide::procedural::bezier_curve;
        use nalgebra::Point2;
        let c = bezier_curve(&[Point2::new(0.0, 0.0), Point2::new(0.5, 1.0), Point2::new(1.0, 0.0)], 10);
        println!("BC coords: {:?}", c.coords());

        let v1 = Vector2::new(1.0f32, 0.0);
        let v2 = Vector2::new(0.0, -1.0);
        let pat = PatternBuilder::new(Angle::Fixed(180.0), Angle::Fixed(360.0), 0.0, 0.5, 20, 1, 100.0).build(&v1, &v2);
        let w = WorldCompBuilder::new(engine).build();
        let g = GraphicsComp::new(engine.graphics.clone(), 1);
        let e = EventComp::new(w.id, engine.events.clone());

        let p = PhysicsComp::new(w.id,
                                 0,
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
            pat: pat,
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

    fn shoot_bullet(&mut self) {
        let pos = self.pg.get_pos();
        let np = match self.pat.next() {
            Some((_, np)) => np,
            None => Vector2::new(0.0, -200.0),
        };
        self.ev.create_entity(Box::new(move |engine| Bullet::new_at_pos(engine, pos, Some(np))));
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
    pub fn new_at_pos(engine: &Engine<Object>, pos: (f32, f32), vel: Option<Vector2<f32>>) -> Object {
        let mut g = GraphicsComp::new(engine.graphics.clone(), 2);
        let w = WorldCompBuilder::new(engine).build();
        let e = EventComp::new(w.id, engine.events.clone());
        let scaler = engine.scene.physics.scaler;
        let p = PhysicsComp::new(w.id,
                                 0,
                                 Vector2::new(pos.0, pos.1),
                                 ShapeHandle2::new(Ball::new(5.0)),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Contacts(0.0),
                                 &engine.scene);
        g.translate(pos.0 / scaler, pos.1 / scaler);
        let mut pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        pg.velocity = match vel {
            Some(v) => v,
            None => Vector2::new(0.0, -200.0),
        };
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
            Event::Proximity(id, ref data) => {
                if let Some(s) = self.world.find_aliased_entity_alias(&id) {
                    match (&s[..], data.proximity) {
                        ("screen_area", Proximity::Disjoint) => {
                            self.ev.destroy_self();
                        }
                        _ => { }
                    }
                }
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
