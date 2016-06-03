use std::rc::Rc;

use game::object::Object;
use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;

pub struct Mouse {
    world: WorldComp<Object>,
    ev: EventComp<Object>,
    pos: (f32, f32),
}

impl Mouse {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Mouse(Mouse {
            world: w,
            ev: e,
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
                self.pos = pos;
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
