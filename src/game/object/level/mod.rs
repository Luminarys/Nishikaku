pub mod path;
pub mod pattern;
pub mod action;
pub mod spawn;
pub mod screen;

use std::rc::Rc;
use std::collections::HashMap;

use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::Registry;
use nalgebra::Vector2;

use game::object::Object;
use game::object::player::Player;
use self::screen::ScreenArea;
use self::spawn::Spawn;

const EVENT_LIM: usize = 1000;
const ACTION_LIM: usize = 2000;
const PATTERN_LIM: usize = 3000;

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

pub enum Point {
    Fixed(Vector2<f32>),
    Player(Vector2<f32>),
    Current(Vector2<f32>),
}

impl Point {
    pub fn eval(&self, current: &Vector2<f32>, player: &Vector2<f32>) -> Vector2<f32> {
        match self {
            &Point::Fixed(ref p) => *p,
            &Point::Current(ref p) => *p + *current,
            &Point::Player(ref p) => *p + *player,
        }
    }
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

    pub fn event_finished(&mut self, id: String) {
        if let Some(events) =  self.events.remove(&id) {
            for e in events {
                if e.delay > 0.001 {
                    let wid = self.ev_reg.get_id();
                    self.ev.set_timer(wid + EVENT_LIM, e.delay);
                    self.waiting_events.insert(wid, e);
                } else {
                    self.handle_level_event(e);
                }
            }
        }
    }

    pub fn handle_level_event(&mut self, evt: LevelEvent) {
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned Level!");
                self.ev.create_entity(Box::new(move |engine| ScreenArea::new(engine)));
                self.ev.create_entity(Box::new(move |engine| Player::new(engine)));
            }
            Event::Timer(id) => {
                if id >= EVENT_LIM && id < ACTION_LIM {
                    self.ev_reg.return_id(id - EVENT_LIM);
                    if let Some(event) = self.waiting_events.remove(&(id - EVENT_LIM)) {
                        self.handle_level_event(event);
                    }
                }
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
