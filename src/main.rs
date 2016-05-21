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
use engine::entity::{Entity, RenderInfo};
use engine::entity::component::*;
use engine::event::{Event, InputState};
use engine::graphics::SpriteVertex;

fn main() {
    let mut engine: Engine<Object> = Engine::new();
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in mat4 transform;
        in vec2 tex_coords;

        out vec2 v_tex_coords;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * transform;
            v_tex_coords = tex_coords;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        uniform sampler2D tex;
        in vec2 v_tex_coords;
        out vec4 color;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let vertex_buffer = engine.graphics.make_sprite_vbo(
                             &[SpriteVertex {
                                   position: [-0.5, -0.5],
                                   tex_coords: [0.0, 0.0],
                               },
                               SpriteVertex {
                                   position: [-0.5, 0.0],
                                   tex_coords: [0.0, 1.0],
                               },
                               SpriteVertex {
                                   position: [-0.25, 0.0],
                                   tex_coords: [1.0, 1.0],
                               },
                               SpriteVertex {
                                   position: [-0.25, -0.5],
                                   tex_coords: [1.0, 0.0],
                               }]);

    let texture = engine.graphics.load_asset("target/sakuya.png");
    engine.graphics.new_sprite(1, vertex_shader_src, fragment_shader_src, vertex_buffer, texture, 1);
    engine.run();
}
