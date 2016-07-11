mod pg;

use std::rc::Rc;

use engine::{BULLET_COUNT, Engine};
use engine::entity::component::{EventComp, GraphicsComp, WorldComp, WorldCompBuilder};
use self::pg::{PGComp, PhysicsComp};
use engine::event::Event;
use engine::scene::PhysicsInteraction;
use engine::util::{ToCartesian, ToPolar};
use ncollide::world::GeometricQueryType;
use nalgebra::Vector2;

use game::object::Object;
use game::object::level::bullet::{Bullet as BulletInfo, Behavior};

pub struct Bullet {
    pub damage: usize,
    pg: PGComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    behavior: Behavior,
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
                                 GeometricQueryType::Contacts(0.5),
                                 &engine.scene);
        g.translate(pos.x / scaler, pos.y / scaler);
        let mut pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        pg.velocity = vel;
        match info.behavior {
            Behavior::Deaccelerate(_, ref accel) => {
                let angle = vel.to_polar().y;
                pg.acceleration = -1.0 * Vector2::new(*accel, angle).to_cartesian();
            }
            _ => { }
        }
        Object::Bullet(Bullet {
            behavior: info.behavior,
            damage: info.damage,
            pg: pg,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                unsafe { BULLET_COUNT += 1 };
                self.ev.set_repeating_timer(1, 1.0);
                self.ev.set_repeating_timer(2, 0.3);
            }
            Event::Timer(1) => {
                if !self.pg.in_screen() {
                    unsafe { BULLET_COUNT -= 1 };
                    self.ev.destroy_self();
                }
            }
            Event::Update(t) => {
                self.ev.update(t);
                self.pg.update(t);
                match self.behavior {
                    Behavior::Deaccelerate(ref mut time, _) => {
                        *time -= t;
                        if *time <= 0.0 {
                            self.pg.acceleration = Vector2::new(0.0, 0.0);
                        }
                    }
                    _ => { }
                }
            }
            Event::Collision(id, ref data) => {
                if let Some(s) = self.world.find_aliased_entity_alias(&id) {
                    match &s[..] {
                        "player" => {
                            self.ev.destroy_self();
                        }
                        _ => { }
                    }
                }
                //pub struct CollisionData {
                //    pub contact: Contact<Point2<f32>>,
                //    pub this_object: Rc<PhysicsData>,
                //    pub other_object: Rc<PhysicsData>,
                //}
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
