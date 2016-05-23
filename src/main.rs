#[macro_use]
extern crate glium;
extern crate glutin;
extern crate ncollide;
extern crate nalgebra;
extern crate clock_ticks;
extern crate image;

#[macro_use]
mod macros;
mod engine;
mod game;

use game::object::Object;
use engine::Engine;

fn main() {
    let mut engine: Engine<Object> = Engine::new();
    game::asset::load_assets(&mut engine);
    engine.spawn(Box::new(|e| {
        game::object::player::Player::new(e)
    }));
    engine.run();
}
