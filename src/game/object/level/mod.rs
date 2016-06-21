pub mod path;
pub mod pattern;
pub mod action;
pub mod spawn;
pub mod screen;
pub mod enemy;
pub mod bullet;
pub mod point;

pub use self::point::Point;

use std::rc::Rc;
use std::collections::HashMap;

use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::Registry;
use nalgebra::Vector2;

use game::object::Object;
use game::object::player::Player;
use game::event::Event as CEvent;
use self::screen::ScreenArea;
use self::spawn::Spawn;

/// Top level game controller
pub struct Level {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    events: HashMap<String, Vec<LevelEvent>>,
    waiting_events: HashMap<usize, LevelEvent>,
    ev_reg: Registry,
}

pub struct LevelEvent {
    name: String,
    delay: f32,
    spawns: Vec<Spawn>,
}

impl Level {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).with_tag(String::from("level")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Level(Level {
            ev: e,
            ev_reg: Registry::new(),
            events: HashMap::new(),
            waiting_events: HashMap::new(),
            world: w,
        })
    }

    fn event_finished(&mut self, id: String) {
        if let Some(events) =  self.events.remove(&id) {
            for e in events {
                if e.delay > 0.001 {
                    let wid = self.ev_reg.get_id();
                    self.ev.set_timer_manual(wid, e.delay, false, Event::Custom(Box::new(CEvent::Level(wid))));
                    self.waiting_events.insert(wid, e);
                } else {
                    self.handle_level_event(e);
                }
            }
        }
    }

    fn handle_level_event(&mut self, evt: LevelEvent) {
        println!("Level event {} triggered", evt.name);

    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned Level!");
                self.ev.create_entity(Box::new(|engine| ScreenArea::new(engine)));
                self.ev.create_entity(Box::new(|engine| Player::new(engine)));
            }
            Event::Update(dt) => {
                self.ev.update(dt);
            }
            Event::Custom(ref cev) => {
                self.handle_cevent(cev.downcast_ref::<CEvent>().unwrap());
            }
            _ => {}
        };

    }

    fn handle_cevent(&mut self, e: &CEvent) {
        match *e {
            CEvent::Level(id) => {
                match self.waiting_events.remove(&id) {
                    Some(e) => self.handle_level_event(e),
                    None => println!("Nonexistent level event {:?}, referenced", id)
                }
            }
            _ => { }
        }
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
