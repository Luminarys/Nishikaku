pub mod path;
pub mod pattern;
pub mod action;
pub mod spawn;
pub mod enemy;
pub mod bullet;
pub mod point;

pub use self::point::Point;

use std::rc::Rc;

use engine::util;
use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::Registry;
use engine::util::HashMap;

use game::object::Object;
use game::object::player::Player;
use game::object::enemy::Enemy;
use game::event::Event as CEvent;
use self::spawn::{Spawn, SpawnType};

/// Top level game controller
pub struct Level {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    events: HashMap<String, Vec<LevelEvent>>,
    waiting_events: HashMap<usize, LevelEvent>,
    waiting_spawns: HashMap<usize, Spawn>,
    active_spawns: Vec<Spawn>,
    ev_reg: Registry,
}

#[derive(Clone, Debug)]
pub struct LevelEvent {
    pub name: String,
    pub id: usize,
    pub delay: f32,
    pub spawns: Vec<Spawn>,
}

impl Level {
    pub fn new(engine: &Engine<Object>, level: HashMap<String, Vec<LevelEvent>>) -> Object {
        let w = WorldCompBuilder::new(engine).with_tag(String::from("level")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Level(Level {
            ev: e,
            ev_reg: Registry::new(),
            events: level,
            waiting_events: util::hashmap(),
            waiting_spawns: util::hashmap(),
            active_spawns: Vec::new(),
            world: w,
        })
    }

    fn event_finished(&mut self, id: String) {
        if let Some(events) = self.events.remove(&id) {
            for e in events {
                if e.delay > 0.001 {
                    self.ev.set_timer_with_class(e.id, e.delay, 1);
                    self.waiting_events.insert(e.id, e);
                } else {
                    self.handle_level_event(e);
                }
            }
        }
    }

    fn handle_level_event(&mut self, evt: LevelEvent) {
        println!("Level event {} triggered", evt.name);
        for spawn in evt.spawns {
            if spawn.repeat > 0 {
                let wid = self.ev_reg.get_id();
                self.ev.set_repeating_timer_with_class(wid, spawn.repeat_delay, 2);
                self.waiting_spawns.insert(wid, spawn.clone());
            }
            self.active_spawns.push(spawn.clone());
        }
        self.event_finished(evt.name);
    }

    fn handle_update(&mut self, t: f32) {
        self.ev.update(t);
        let mut done_pats = Vec::new();
        for (i, ref mut spawn) in self.active_spawns.iter_mut().enumerate() {
            let ref mut pat = spawn.pattern;
            let spawns = pat.next(t);
            for &(pos, _vel) in spawns.iter() {
                match spawn.spawn_type {
                    SpawnType::Enemy(ref e_info) => {
                        let info = e_info.clone();
                        let pos = pos + spawn.location;
                        let paths = spawn.paths.clone();
                        self.ev.create_entity(Box::new(move |engine| Enemy::new(engine, info, pos, paths.clone())));
                    }
                    SpawnType::Player => {
                        // Spawn the palyer
                        self.ev.create_entity(Box::new(|engine| Player::new(engine)));
                    }
                }
            }
            if spawns.len() == 0 && pat.finished() {
                done_pats.push(i);
            }
        }

        let mut i = 0;
        self.active_spawns = self.active_spawns.clone().into_iter().filter(|_| {
            let ret = !done_pats.contains(&i);
            i += 1;
            ret
        }).collect();

    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned Level!");
                self.event_finished(String::from("start"));
            }
            Event::Update(t) => {
                self.handle_update(t);
            }
            Event::CTimer(1, id) => {
                // Event timer delay
                match self.waiting_events.remove(&id) {
                    Some(event) => {
                        self.handle_level_event(event);
                    }
                    None => println!("Nonexistent level event {:?}, referenced", id)
                }
            }
            Event::CTimer(2, id) => {
                // Repeated spawns
                let mut repeat = 0;
                if let Some(ref mut spawn) = self.waiting_spawns.get_mut(&id) {
                    spawn.repeat -= 1;
                    repeat = spawn.repeat;
                    self.active_spawns.push(spawn.clone());
                }

                if repeat <= 0 {
                    self.ev.remove_timer_with_class(id, 2);
                }
            }
            Event::Custom(ref cev) => {
                self.handle_cevent(cev.downcast_ref::<CEvent>().unwrap());
            }
            _ => {}
        };
    }

    fn handle_cevent(&mut self, e: &CEvent) {
        match *e {
            _ => { }
        }
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
