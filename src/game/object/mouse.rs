use std::rc::Rc;
use nalgebra;
use nalgebra::{Vector2, Isometry2};
use ncollide::shape::{Cuboid, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use ncollide::query::Proximity;
use glium::glutin::MouseButton;

use game::object::Object;
use game::event::Event as CEvent;
use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::scene::PhysicsInteraction;

pub struct Mouse {
    world: WorldComp<Object>,
    ev: EventComp<Object>,
    phys: PhysicsComp,
    moused_over: Vec<usize>,
    pos: (f32, f32),
}

impl Mouse {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).build();
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
            moused_over: Vec::new(),
            pos: (0.0, 0.0),
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Mouse spawned!");
                self.ev.subscribe(Event::MouseMove((0.0, 0.0)));
                self.ev.subscribe(Event::MouseInput(InputState::Released, MouseButton::Left));
            }
            Event::MouseMove(pos) => {
                self.phys.set_pos(Isometry2::new(Vector2::new(pos.0, pos.1), nalgebra::Vector1::new(0.0)));
                self.pos = pos;
            }
            Event::MouseInput(InputState::Pressed, MouseButton::Left) => {
                let len = self.moused_over.len();
                if len > 0 {
                    let id = self.moused_over[len-1];
                    let e = Event::Custom(Box::new(CEvent::MouseClickedOver));
                    self.ev.dispatch_to(id, e);
                }
            }
            Event::MouseInput(InputState::Released, MouseButton::Left) => {
                let len = self.moused_over.len();
                if len > 0 {
                    let id = self.moused_over[len-1];
                    let e = Event::Custom(Box::new(CEvent::MouseUnclickedOver));
                    self.ev.dispatch_to(id, e);
                }
            }
            Event::Proximity(id, ref data) => {
                match data.proximity {
                    Proximity::Intersecting => {
                        self.moused_over.push(id);
                        let e = Event::Custom(Box::new(CEvent::MouseOver));
                        self.ev.dispatch_to(id, e);
                    }
                    Proximity::Disjoint => {
                        // assert_eq!(Some(id), self.moused_over.pop());
                        let e = Event::Custom(Box::new(CEvent::MouseLeft));
                        self.ev.dispatch_to(id, e);
                    }
                    _ => { }
                }
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
