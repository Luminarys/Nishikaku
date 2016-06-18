pub mod path;
pub mod pattern;

use std::rc::Rc;

use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::PhysicsInteraction;
use ncollide::shape::{Cuboid, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use nalgebra::Vector2;

use game::object::Object;
use game::object::player::Player;

/// Top level game controller
pub struct Level {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Level {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).build();
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Level(Level {
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned Level!");
                self.ev.create_entity(Box::new(move |engine| ScreenArea::new(engine)));
                self.ev.create_entity(Box::new(move |engine| Player::new(engine)));
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}

pub struct ScreenArea {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    phys: PhysicsComp,
}

impl ScreenArea {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).with_alias(String::from("screen_area")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        let scaler = engine.scene.physics.scaler;
        let p = PhysicsComp::new(w.id,
                                 0,
                                 Vector2::new(0.0, 0.0),
                                 ShapeHandle2::new(Cuboid::new(Vector2::new(scaler, scaler))),
                                 PhysicsInteraction::Interactive,
                                 GeometricQueryType::Proximity(0.5),
                                 &engine.scene);
        Object::ScreenArea(ScreenArea {
            ev: e,
            world: w,
            phys: p,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned Level!");
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
