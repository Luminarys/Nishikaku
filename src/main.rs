#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate ncollide_geometry;
extern crate ncollide_procedural;
extern crate nalgebra;
extern crate clock_ticks;
extern crate image;
extern crate sdl2;
extern crate sdl2_mixer;
extern crate toml;
extern crate fnv;

#[macro_use]
mod macros;
mod engine;
mod game;

use game::object::Object;
use game::physics::DanmakuPhysics;
use engine::Engine;

fn main() {
    let scaler = 200.0;
    let mut engine: Engine<Object> = Engine::new(scaler, 700, Box::new(DanmakuPhysics::new(scaler)));
    let assets = game::asset::load_assets(&mut engine);
    engine.spawn_entity(game::object::controller::Controller::new(&engine, assets));
    engine.run();
}
