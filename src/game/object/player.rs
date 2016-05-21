use engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::entity::{Entity, RenderInfo};
use game::object::Object;

pub struct Player {
    gfx: GraphicsComp,
    ev: EventComp<Object>,
}

impl Player {
    pub fn new(engine: &engine::Engine<Object>) -> Object {
        let g = GraphicsComp::new(1);
        let e = EventComp::new(1, engine.events.clone());
        Object::Player(Player {
            gfx: g,
            ev: e,
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
            _ => {
            }
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.gfx.get_render_info())
    }
    
    pub fn id(&self) -> usize {
        1
    }
}

pub struct Bullet {
    gfx: GraphicsComp,
    ev: EventComp<Object>,
}

impl Bullet {
    pub fn new(engine: &engine::Engine<Object>) -> Object {
        let g = GraphicsComp::new(1);
        let e = EventComp::new(1, engine.events.clone());
        Object::PlayerBullet(Bullet {
            gfx: g,
            ev: e,
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
            _ => {
            }
        };
    }

    pub fn render(&self) -> Option<RenderInfo> {
        Some(self.gfx.get_render_info())
    }
    
    pub fn id(&self) -> usize {
        1
    }
}
