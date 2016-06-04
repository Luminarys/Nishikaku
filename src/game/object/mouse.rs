use std::rc::Rc;
use nalgebra;
use nalgebra::{Vector2, Isometry2};
use ncollide::shape::{Cuboid, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use ncollide::query::Proximity;


use game::object::Object;
use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::PhysicsInteraction;

pub struct Mouse {
    world: WorldComp<Object>,
    ev: EventComp<Object>,
    phys: PhysicsComp,
    pos: (f32, f32),
}

impl Mouse {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        let p = PhysicsComp::new(w.id,
                                 0,
                                 Vector2::new(0.0, 0.0),
                                 ShapeHandle2::new(Cuboid::new(Vector2::new(0.2, 0.2))),
                                 PhysicsInteraction::Interactive,
                                 GeometricQueryType::Proximity(0.05),
                                 &engine.scene);
        Object::Mouse(Mouse {
            world: w,
            ev: e,
            phys: p,
            pos: (0.0, 0.0),
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Mouse spawned!");
                self.ev.subscribe(Event::MouseMove((0.0, 0.0)));
            }
            Event::MouseMove(pos) => {
                self.phys.set_pos(Isometry2::new(Vector2::new(pos.0, pos.1), nalgebra::Vector1::new(0.0)));
                self.pos = pos;
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
