use nalgebra::Vector2;
use ncollide::world::GeometricQueryType;
use std::rc::Rc;
use std::mem;

use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::PhysicsInteraction;
use engine::entity::RenderInfo;
use game::object::Object;
use game::object::level::path::{Path, PathBuilder};
use game::object::level::enemy::Enemy as EnemyInfo;
use game::object::level::action::ActionType;

pub struct Enemy {
    health: usize,
    pub damage: usize,
    paths: Vec<PathBuilder>,
    cpath: Path,
    actions: Vec<ActionType>,
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
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
        let mut path = paths.pop().unwrap().build(&pos, &ppos);
        let mut actions = Vec::new();
        for (id, action) in path.actions().iter_mut().enumerate() {
            actions.push(mem::replace(&mut action.action_type, ActionType::None));
            e.set_timer(id, action.delay);
        }
        // pg.velocity = vel;
        Object::Enemy(Enemy {
            health: info.health,
            paths: paths,
            cpath: path,
            damage: info.damage,
            actions: actions,
            pg: pg,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Update(t) => {
                match self.cpath.travel(t) {
                    Some(p) => self.pg.set_pos((p.x, p.y)),
                    None => {
                        if let Some(pb) = self.paths.pop() {
                            let ppos = self.get_player_pos();
                            self.cpath = pb.build(&self.pg.get_vpos(), &ppos);
                            let len = self.actions.len();
                            for (id, action) in self.cpath.actions().iter_mut().enumerate() {
                                self.actions.push(mem::replace(&mut action.action_type, ActionType::None));
                                self.ev.set_timer(id + len, action.delay);
                            }
                        } else {
                            self.ev.destroy_self();
                        }
                    }
                };
                self.pg.update(t);
            }
            Event::Timer(i) => {
                match mem::replace(&mut self.actions[i], ActionType::None) {
                    ActionType::Bullets(bullet, pb) => {
                        let ppos = self.get_player_pos();
                        let pattern = pb.build(&self.pg.get_vpos(), &ppos);
                        // TODO: Spawn the bullets
                    }
                    ActionType::None => {}
                }
            }
            Event::Collision(id, ref _data) => {
                if let Some(s) = self.world.find_aliased_entity_alias(&id) {
                    if &s[..] == &String::from("player") {
                        // What do we do when we hit the player?
                        self.ev.destroy_self();
                    }
                }
            }
            Event::Render => {
                self.pg.render();
            }
            _ => {}
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.pg.get_render_info())
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
