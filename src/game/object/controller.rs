use std::rc::Rc;

use engine::Engine;
use engine::entity::component::*;
use engine::entity::EntityBuilder;
use engine::event::Event;
use game::event::Event as CEvent;

use game::asset::Assets;
use game::object::Object;
use game::object::mouse::Mouse;
use game::object::menu::MainMenu;
use game::object::level::Level;

/// Top level game controller
pub struct Controller {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    assets: Assets,
}

impl Controller {
    pub fn new(engine: &Engine<Object>, assets: Assets) -> Object {
        let w = WorldCompBuilder::new(engine).with_alias(String::from("controller")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Controller(Controller {
            ev: e,
            world: w,
            assets: assets,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned controller!");
                self.ev.create_entity(Box::new(|engine| Mouse::new(engine)));
                self.ev.create_entity(Box::new(|engine| MainMenu::new(engine)));
                let level = self.assets.levels.pop().unwrap();
                // self.ev.create_entity(Box::new(move |engine| Level::new(engine, level.clone())));
            }
            Event::Custom(ref cev) => {
                self.handle_cevent(cev.downcast_ref::<CEvent>().unwrap());
            }
            _ => {}
        };
    }

   fn handle_cevent(&mut self, e: &CEvent) {
       match *e {
           CEvent::LevelStart => {
               let level = self.assets.levels.pop().unwrap();
                self.ev.create_entity(Box::new(move |engine| Level::new(engine, level.clone())));
           }
           _ => { }
       }
   }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
