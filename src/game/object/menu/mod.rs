use std::rc::Rc;
use nalgebra::Vector2;
use ncollide::shape::{Cuboid, ShapeHandle2};
use ncollide::world::GeometricQueryType;
use ncollide::query::Proximity;

use game::object::Object;
use engine::Engine;
use engine::scene::PhysicsInteraction;
use engine::entity::component::*;
use engine::event::{Event, InputState};

pub struct MainMenu {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl MainMenu {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        Object::MainMenu(MainMenu {
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                self.ev.create_entity(Box::new(move |engine| MainMenuBar::new(engine)));
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}

pub struct MainMenuBar {
    pg: PGComp,
    text: TextComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    selected: bool,
}

impl MainMenuBar {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let e = EventComp::new(w.id, engine.events.clone());
        let g = GraphicsComp::new(engine.graphics.clone(), 3);
        let p = PhysicsComp::new(w.id,
                                 0,
                                 Vector2::new(0.0, 0.0),
                                 ShapeHandle2::new(Cuboid::new(Vector2::new(120.0, 20.0))),
                                 PhysicsInteraction::SemiInteractive,
                                 GeometricQueryType::Proximity(0.1),
                                 &engine.scene);
        let pg = PGComp::new(g, vec![p], engine.scene.physics.clone());
        let text = TextCompBuilder::new_scaled(engine).with_font(1).with_pos((-20.0, -10.0)).with_text("Play").build();
        Object::MainMenuBar(MainMenuBar {
            ev: e,
            world: w,
            pg: pg,
            text: text,
            selected: false,
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                println!("Spawned menu bar!");
            }
            Event::Render => {
                self.pg.render();
                self.text.render();
            }
            Event::Proximity(id, ref data) => {
                println!("Proximity ev");
            }
            _ => {}
        };
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
