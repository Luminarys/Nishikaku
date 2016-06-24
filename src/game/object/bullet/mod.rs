use std::collections::HashMap;
use std::rc::Rc;

use engine::Engine;
use engine::entity::component::*;
use engine::event::Event;
use engine::scene::Registry;
use engine::scene::PhysicsInteraction;
use ncollide::query::Proximity;
use ncollide::world::GeometricQueryType;
use nalgebra::Vector2;

use game::object::Object;
use game::event::Event as CEvent;
use game::object::level::bullet::Bullet as BulletInfo;

pub struct Bullet {
    pub damage: usize,
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Bullet {
    pub fn new(engine: &Engine<Object>, info: BulletInfo, pos: Vector2<f32>, vel: Vector2<f32>) -> Object {
        let mut g = GraphicsComp::new(engine.graphics.clone(), info.sprite);
        let w = WorldCompBuilder::new(engine).build();
        let e = EventComp::new(w.id, engine.events.clone());
        let scaler = engine.scene.physics.scaler;
        let p = PhysicsComp::new(w.id,
                                 0,
                                 pos,
                                 engine.graphics.borrow().get_sprite_shape(&info.sprite).unwrap(),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Contacts(0.0),
                                 &engine.scene);
        g.translate(pos.x / scaler, pos.y / scaler);
        let mut pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        pg.velocity = vel;
        Object::Bullet(Bullet {
            damage: info.damage,
            pg: pg,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {}
            Event::Update(t) => {
                self.pg.update(t);
            }
            Event::Proximity(id, ref data) => {
                if let Some(s) = self.world.find_aliased_entity_alias(&id) {
                    match (&s[..], data.proximity) {
                        ("screen_area", Proximity::Disjoint) => {
                            self.ev.destroy_self();
                        }
                        ("player", Proximity::Intersecting) => {
                            self.ev.destroy_self();
                        }
                        _ => { }
                    }
                }
            }
            Event::Render => {
                self.pg.render();
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
