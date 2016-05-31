// use std::path::Path;

use engine::Engine;
use engine::graphics::SpriteVertex;
use game::object::Object;

pub fn load_assets(engine: &mut Engine<Object>) {
    println!("Loading assets!");
    load_bullet(engine);
    load_char(engine);
    load_sound(engine);
}

fn load_sound(_engine: &mut Engine<Object>) {
    println!("Loading songs!");
    //let path = Path::new("/tmp/song.flac");
    //engine.audio.borrow_mut().load(1, path);
    //engine.audio.borrow().play(&1);
}

fn load_char(engine: &mut Engine<Object>) {
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

    let vertex_buffer = engine.graphics.make_sprite_vbo(&[SpriteVertex {
                                                              position: [-0.125, -0.25],
                                                              tex_coords: [0.0, 0.0],
                                                          },
                                                          SpriteVertex {
                                                              position: [-0.125, 0.25],
                                                              tex_coords: [0.0, 1.0],
                                                          },
                                                          SpriteVertex {
                                                              position: [0.125, 0.25],
                                                              tex_coords: [1.0, 1.0],
                                                          },
                                                          SpriteVertex {
                                                              position: [0.125, -0.25],
                                                              tex_coords: [1.0, 0.0],
                                                          }]);

    let texture = engine.graphics.load_asset("assets/sakuya.png");
    engine.graphics.new_sprite(1,
                               vertex_shader_src,
                               fragment_shader_src,
                               vertex_buffer,
                               texture,
                               1);
}

fn load_bullet(engine: &mut Engine<Object>) {
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

    let vertex_buffer = engine.graphics.make_sprite_vbo(&[SpriteVertex {
                                                              position: [-0.125, -0.125],
                                                              tex_coords: [0.0, 0.0],
                                                          },
                                                          SpriteVertex {
                                                              position: [-0.125, 0.125],
                                                              tex_coords: [0.0, 1.0],
                                                          },
                                                          SpriteVertex {
                                                              position: [0.125, 0.125],
                                                              tex_coords: [1.0, 1.0],
                                                          },
                                                          SpriteVertex {
                                                              position: [0.125, -0.125],
                                                              tex_coords: [1.0, 0.0],
                                                          }]);

    let texture = engine.graphics.load_asset("assets/bullet.png");
    engine.graphics.new_sprite(2,
                               vertex_shader_src,
                               fragment_shader_src,
                               vertex_buffer,
                               texture,
                               100);
}
