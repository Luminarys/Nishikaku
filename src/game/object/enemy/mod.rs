use nalgebra::Vector2;
use ncollide::world::GeometricQueryType;
use std::rc::Rc;
use std::cell::Cell;
use std::mem;

use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::PhysicsInteraction;
use game::event::Event as CEvent;
use game::object::Object;
use game::object::level::path::{Path, PathBuilder};
use game::object::level::pattern::Pattern;
use game::object::level::enemy::Enemy as EnemyInfo;
use game::object::level::action::ActionType;
use game::object::level::bullet::Bullet as BulletInfo;
use game::object::bullet::Bullet;
use game::object::player::PLAYER_POSITION;

#[derive(Clone)]
pub struct PosFetcher {
    pos: Cell<Vector2<f32>>,
    world: WorldComp<Object>,
}

use std::fmt;

impl fmt::Debug for PosFetcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PosFetcher Object, derived from world comp with id: {}", self.world.id)
    }
}

impl PosFetcher {
    pub fn new(pos: Cell<Vector2<f32>>, world: WorldComp<Object>) -> PosFetcher {
        PosFetcher { pos: pos, world: world }
    }

    pub fn fetch(&self) -> (Vector2<f32>, Vector2<f32>) {
        // muh performance :')
        let ppos = unsafe { PLAYER_POSITION.clone() };
        (self.pos.get(), ppos)
    }
}

pub struct Enemy {
    health: usize,
    pub damage: usize,
    paths: Vec<PathBuilder>,
    cpath: Path,
    actions: Vec<ActionType>,
    patterns: Vec<(BulletInfo, Pattern)>,
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    pos: Cell<Vector2<f32>>,
}

impl Enemy {
    pub fn new(engine: &Engine<Object>,
               info: EnemyInfo,
               pos: Vector2<f32>,
               mut paths: Vec<PathBuilder>)
               -> Object {
        let mut g = GraphicsComp::new(engine.graphics.clone(), info.sprite);
        let w = WorldCompBuilder::new(engine).build();
        let mut e = EventComp::new(w.id, engine.events.clone());
        let scaler = engine.scene.physics.scaler;
        let p = PhysicsComp::new(w.id,
                                 0,
                                 Vector2::new(pos.x, pos.y),
                                 engine.graphics.borrow().get_sprite_shape(&info.sprite).unwrap(),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Contacts(0.0),
                                 &engine.scene);
        g.translate(pos.x / scaler, pos.y / scaler);
        let pg = PGComp::new(g, vec![p], engine.scene.physics.clone());

        let ppos = {
            let pid = w.find_aliased_entity_id(&String::from("player")).unwrap();
            let ea = w.get_entity(&pid);
            let p = match *ea.access().unwrap() {
                Object::Player(ref p) => p.get_pos(),
                _ => panic!("Non player object aliased to player!"),
            };
            p
        };
        // Makes it easy to pop from the back
        paths.reverse();
        let mut path = paths.pop().unwrap().build(&pos, &ppos);
        let mut actions = Vec::new();
        for (id, action) in path.actions().iter_mut().enumerate() {
            actions.push(mem::replace(&mut action.action_type, ActionType::None));
            e.set_timer_with_class(id, action.delay, 1);
        }
        // pg.velocity = vel;
        let cpos = pg.get_vpos();
        Object::Enemy(Enemy {
            health: info.health,
            paths: paths,
            cpath: path,
            damage: info.damage,
            actions: actions,
            patterns: Vec::new(),
            pg: pg,
            ev: e,
            world: w,
            pos: Cell::new(cpos),
        })
    }

    pub fn get_pos(&self) -> Vector2<f32> {
        self.pg.get_vpos()
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Update(t) => {
                self.ev.update(t);
                self.pg.update(t);
                match self.cpath.travel(t) {
                    Some(p) => self.pg.set_pos((p.x, p.y)),
                    None => {
                        if let Some(pb) = self.paths.pop() {
                            let ppos = self.get_player_pos();
                            self.cpath = pb.build(&self.pg.get_vpos(), &ppos);
                            let len = self.actions.len();
                            for (id, action) in self.cpath.actions().iter_mut().enumerate() {
                                if action.delay > 0.00001 {
                                    self.actions.push(mem::replace(&mut action.action_type, ActionType::None));
                                    self.ev.set_timer_with_class(id + len, action.delay, 1);
                                } else {
                                    self.handle_action(mem::replace(&mut action.action_type, ActionType::None));
                                }
                            }
                        } else {
                            self.ev.destroy_self();
                        }
                    }
                };
                let mut done_pats = Vec::new();
                for (i, &mut (ref bullet, ref mut pat)) in self.patterns.iter_mut().enumerate() {
                    let spawns = pat.next(t);
                    if spawns.len() == 0 && pat.finished() {
                        done_pats.push(i);
                        continue;
                    }
                    for (pos, vel) in spawns {
                        let b = bullet.clone();
                        let pos = pos + self.pg.get_vpos();
                        self.ev.create_entity(Box::new(move |engine| Bullet::new(engine, b, pos, vel)));
                    }
                }
                for i in done_pats {
                    self.patterns.remove(i);
                }
                self.pos.set(self.pg.get_vpos());
            }
            Event::CTimer(1, i) => {
                // Action timer trigger
                let action = mem::replace(&mut self.actions[i], ActionType::None);
                self.handle_action(action);
            }
            Event::Collision(id, ref _data) => {
                if let Some(s) = self.world.find_aliased_entity_alias(&id) {
                    if &s[..] == "player" {
                        // What do we do when we hit the player?
                        println!("Player coliision!");
                        self.ev.destroy_self();
                    }
                }
            }
            Event::Render => {
                self.pg.render();
            }
            Event::Custom(ref cev) => {
                self.handle_cevent(cev.downcast_ref::<CEvent>().unwrap());
            }
            _ => {}
        };
    }

    fn handle_action(&mut self, a: ActionType) {
        match a {
            ActionType::Bullets(bullet, pb) => {
                let ppos = self.get_player_pos();
                let mut pattern = pb.build(&self.pg.get_vpos(), &ppos);
                pattern.set_pos_fetcher(PosFetcher::new(self.pos.clone(), self.world.clone()));
                self.patterns.push((bullet.clone(), pattern.clone()));
            }
            ActionType::None => {}
        }
    }

    fn handle_cevent(&mut self, e: &CEvent) {
        match *e {
            _ => { }
        }
    }


    pub fn id(&self) -> usize {
        self.world.id
    }

    fn get_player_pos(&self) -> Vector2<f32> {
        let pid = self.world.find_aliased_entity_id(&String::from("player")).unwrap();
        let ea = self.world.get_entity(&pid);
        let p = match *ea.access().unwrap() {
            Object::Player(ref p) => p.get_pos(),
            _ => panic!("Non player object aliased to player!"),
        };
        p
    }
}
