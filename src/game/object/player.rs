use nalgebra::{Isometry2, Vector2};
use ncollide::shape::{Ball, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use nalgebra;

use engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::scene::{PhysicsInteraction};
use engine::entity::RenderInfo;
use game::object::Object;

pub struct Player {
    gfx: GraphicsComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Player {
    pub fn new(engine: &engine::Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let g = GraphicsComp::new(1);
        let e = EventComp::new(1, engine.events.clone());
        Object::Player(Player {
            gfx: g,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::Spawn => {
                self.ev.subscribe(Event::KeyInput(InputState::Pressed, 0));
            }
            Event::Update(t) => {}
            Event::KeyInput(InputState::Pressed, 111) => {
                self.gfx.translate(0.0, 0.01);
            }
            Event::KeyInput(InputState::Pressed, 113) => {
                self.gfx.translate(-0.01, 0.0);
            }
            Event::KeyInput(InputState::Pressed, 114) => {
                self.gfx.translate(0.01, 0.0);
            }
            Event::KeyInput(InputState::Pressed, 116) => {
                self.gfx.translate(0.0, -0.01);
            }
            Event::KeyInput(InputState::Pressed, 52) => {
                // Shoot bullet
                let pos = self.gfx.get_pos();
                self.ev.create_entity(Box::new(move |engine| Bullet::new_at_pos(engine, pos)));
            }
            _ => {}
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.gfx.get_render_info())
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
    pub fn new_at_pos(engine: &engine::Engine<Object>, pos: (f32, f32)) -> Object {
        let mut g = GraphicsComp::new(2);
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        let scaler = engine.scene.physics.scaler;
        let p = PhysicsComp::new(w.id,
                                 String::from("bullet"),
                                 Vector2::new(pos.0 * scaler, pos.1 * scaler),
                                 ShapeHandle2::new(Ball::new(50.0)),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Contacts(0.0),
                                 &engine.scene);
        g.translate(pos.0, pos.1);
        let pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        Object::PlayerBullet(Bullet {
            pg: pg,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::Spawn => {}
            Event::Update(t) => {
                self.pg.translate(Vector2::new(0.0, 2.0));
            }
            Event::Exiting => {
                self.ev.destroy_self();
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
