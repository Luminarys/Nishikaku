#[macro_use]
extern crate glium;
extern crate glutin;
extern crate ncollide;
extern crate nalgebra;
extern crate clock_ticks;

mod engine;

use engine::entity::{Entity, RenderInfo};
use engine::entity::component::*;
use engine::event::Event;

struct Object {
    gfx: GraphicsComp,
}

impl Entity for Object {
    fn handle_event(&mut self, e: Event) {
    
    }

    fn render(&self) -> RenderInfo {
        self.gfx.get_render_info()
    }
}

fn main() {
    let mut engine: engine::Engine<Object> = engine::Engine::new();
    engine.run();
}
