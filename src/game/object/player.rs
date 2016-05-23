use engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::entity::{RenderInfo};
use game::object::Object;

pub struct Player {
    gfx: GraphicsComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Player {
    pub fn new(engine: &engine::Engine<Object>) -> Object {
        let w = WorldComp::new(&engine.scene);
        let g = GraphicsComp::new(1);
        let e = EventComp::new(1, engine.events.clone());
        Object::Player(Player {
            gfx: g,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::Spawn => {
                self.ev.subscribe(Event::KeyInput(InputState::Pressed, 0));
            }
            Event::Update(t) => {
            }
            Event::KeyInput(InputState::Pressed, 111) => {
                self.gfx.translate(0.0, 0.01);
            }
            Event::KeyInput(InputState::Pressed, 113) => {
                self.gfx.translate(-0.01, 0.0);
            }
            Event::KeyInput(InputState::Pressed, 114) => {
                self.gfx.translate(0.01, 0.0);
            }
            Event::KeyInput(InputState::Pressed, 116) => {
                self.gfx.translate(0.0, -0.01);
            }
            Event::KeyInput(InputState::Pressed, 52) => {
                // Shoot bullet
                let pos = self.gfx.get_pos();
                self.ev.create_entity(Box::new(move |engine| {
                    Bullet::new_at_pos(engine, pos)
                }));
            }
            _ => {
            }
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.gfx.get_render_info())
    }
    
    pub fn id(&self) -> usize {
        self.world.id
    }
}

pub struct Bullet {
    gfx: GraphicsComp,
    ev: EventComp<Object>,
    world: WorldComp<Object>,
}

impl Bullet {
    pub fn new_at_pos(engine: &engine::Engine<Object>, pos: (f32, f32)) -> Object {
        let mut g = GraphicsComp::new(2);
        let e = EventComp::new(2, engine.events.clone());
        let w = WorldComp::new(&engine.scene);
        g.set_pos(pos.0, pos.1);
        Object::PlayerBullet(Bullet {
            gfx: g,
            ev: e,
            world: w,
        })
    }

    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::Spawn => {
            }
            Event::Update(t) => {
                self.gfx.translate(0.0, 0.1);
            }
            _ => {
            }
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.gfx.get_render_info())
    }
    
    pub fn id(&self) -> usize {
        self.world.id
    }
}
