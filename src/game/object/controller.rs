use std::rc::Rc;

use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};

use game::object::Object;
use game::object::mouse::Mouse;

/// Top level game controller
pub struct Controller {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Controller {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Controller(Controller {
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned controller!");
                self.ev.create_entity(Box::new(move |engine| Mouse::new(engine)));
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}