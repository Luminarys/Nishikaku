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
use self::screen::ScreenArea;
use self::spawn::{Spawn, SpawnType};

/// Top level game controller
pub struct Level {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    events: HashMap<String, Vec<LevelEvent>>,
    waiting_events: HashMap<usize, LevelEvent>,
    waiting_spawns: HashMap<usize, Spawn>,
    active_spawns: HashMap<usize, Spawn>,
    ev_reg: Registry,
}

pub struct LevelEvent {
    pub name: String,
    pub id: usize,
    pub delay: f32,
    pub spawns: Vec<Spawn>,
}

impl Level {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).with_tag(String::from("level")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        Object::Level(Level {
            ev: e,
            ev_reg: Registry::new(),
            events: util::hashmap(),
            waiting_events: util::hashmap(),
            waiting_spawns: util::hashmap(),
            active_spawns: util::hashmap(),
            world: w,
        })
    }

    fn event_finished(&mut self, id: String) {
        if let Some(events) = self.events.remove(&id) {
            for e in events {
                if e.delay > 0.001 {
                    self.ev.set_repeating_timer_with_class(e.id, e.delay, 1);
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
            let id = self.ev_reg.get_id();
            if spawn.repeat > 0 {
                let wid = self.ev_reg.get_id();
                self.ev.set_repeating_timer_with_class(wid, spawn.repeat_delay, 2);
                self.waiting_spawns.insert(wid, spawn.clone());
            }
            self.ev.set_repeating_timer_with_class(id, spawn.pattern.time_int, 3);
            self.active_spawns.insert(id, spawn.clone());
            // pub struct Spawn {
            //     pub spawn_type: SpawnType,
            //     pub location: Vector2<f32>,
            //     pub paths: Vec<PathBuilder>,
            //     pub pattern: Pattern,
            //     pub repeat: usize,
            //     pub repeat_delay: f32,
            //     pub mirror_x: bool,
            //     pub mirror_y: bool,
            // }
            // Handle mirror x/y here!
            // pub fn set_repeating_timer_with_class(&mut self, id: usize, amount: f32, class: u8) {
        }
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
                        self.ev.set_repeating_timer_with_class(id, spawn.pattern.time_int, 3);
                        self.active_spawns.insert(id, spawn.clone());
                    }

                    if repeat <= 0 {
                        self.ev.remove_timer_with_class(id, 2);
                    }
                }
                Event::CTimer(3, id) => {
                    // event pattern spawns
                    let mut done = false;
                    if let Some(ref mut spawn) = self.active_spawns.get_mut(&id) {
                        if let Some((pos, _)) = spawn.pattern.next() {
                            match spawn.spawn_type {
                                SpawnType::Enemy(ref info) => {
                                    let i = info.clone();
                                    let pos = pos + spawn.pattern.center;
                                    let paths = spawn.paths.clone();
                                    self.ev.create_entity(Box::new(move |engine| Enemy::new(engine, i, pos, paths.clone())));
                                }
                                SpawnType::Player => {
                                    // Spawn the palyer
                                    self.ev.create_entity(Box::new(|engine| Player::new(engine)));
                                }
                            }
                        } else {
                            done = true;
                        }
                    }
                    if done {
                        self.active_spawns.remove(&id);
                        self.ev.remove_timer_with_class(id, 3);
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
