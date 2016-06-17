#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate ncollide;
extern crate nalgebra;
extern crate clock_ticks;
extern crate image;
extern crate sdl2;
extern crate sdl2_mixer;
extern crate toml;

#[macro_use]
mod macros;
mod engine;
mod game;

use game::object::Object;
use engine::Engine;

fn main() {
    let mut engine: Engine<Object> = Engine::new(200.0, 700);
    game::asset::load_assets(&mut engine);
    engine.spawn(Box::new(|e| {
        game::object::controller::Controller::new(e)
    }));
    engine.run();
}
