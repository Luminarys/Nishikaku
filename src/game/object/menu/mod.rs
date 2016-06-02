use std::rc::Rc;

use game::object::Object;
use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};

pub struct MainMenu {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl MainMenu {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        Object::MainMenu(MainMenu {
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
            
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
