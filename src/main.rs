#![recursion_limit="128"]

#[macro_use]
extern crate imgui;
extern crate glium;
extern crate ncollide_geometry;
extern crate ncollide_procedural;
extern crate nalgebra;
extern crate toml;
extern crate engine;
extern crate clock_ticks;

#[macro_use]
mod macros;
mod game;

use game::object::Object;
use game::physics::DanmakuPhysics;
use engine::Engine;

fn main() {
    let scaler = 200.0;
    let mut engine: Engine<Object> = Engine::new(scaler, 700, Box::new(DanmakuPhysics::new(scaler)));
    game::asset::load_assets(&mut engine);
    engine.spawn_entity(game::object::controller::Controller::new(&engine));
    engine.run();
}
