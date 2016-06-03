// use std::path::Path;

use engine::Engine;
use engine::graphics::SpriteVertex;
use game::object::Object;

pub fn load_assets(engine: &mut Engine<Object>) {
    println!("Loading assets!");
    load_bullet(engine);
    load_char(engine);
    load_menu(engine);
    load_sound(engine);
}

fn load_sound(_engine: &mut Engine<Object>) {
    println!("Loading songs!");
    // let path = Path::new("/tmp/song.flac");
    // engine.audio.borrow_mut().load(1, path);
    // engine.audio.borrow().play(&1);
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

    let mut gfx = engine.graphics.borrow_mut();
    let vertex_buffer = gfx.make_sprite_vbo(&[SpriteVertex {
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

    let texture = gfx.load_asset("assets/sakuya.png");
    gfx.new_sprite(1,
                   vertex_shader_src,
                   fragment_shader_src,
                   vertex_buffer,
                   Some(texture),
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

    let mut gfx = engine.graphics.borrow_mut();
    let size_dec = 10.0;
    let vertex_buffer = gfx.make_sprite_vbo(&[SpriteVertex {
                                                  position: [-0.125 / size_dec, -0.125 / size_dec],
                                                  tex_coords: [0.0, 0.0],
                                              },
                                              SpriteVertex {
                                                  position: [-0.125 / size_dec, 0.125 / size_dec],
                                                  tex_coords: [0.0, 1.0],
                                              },
                                              SpriteVertex {
                                                  position: [0.125 / size_dec, 0.125 / size_dec],
                                                  tex_coords: [1.0, 1.0],
                                              },
                                              SpriteVertex {
                                                  position: [0.125 / size_dec, -0.125 / size_dec],
                                                  tex_coords: [1.0, 0.0],
                                              }]);

    let texture = gfx.load_asset("assets/bullet.png");
    gfx.new_sprite(2,
                   vertex_shader_src,
                   fragment_shader_src,
                   vertex_buffer,
                   Some(texture),
                   300);
}

fn load_menu(engine: &mut Engine<Object>) {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in mat4 transform;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * transform;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.319,0.345,0.312,1.000);
        }
    "#;

    let mut gfx = engine.graphics.borrow_mut();
    let vertex_buffer = gfx.make_sprite_vbo(&[SpriteVertex {
                                                  position: [-0.6, -0.1],
                                                  tex_coords: [0.0, 0.0],
                                              },
                                              SpriteVertex {
                                                  position: [-0.6, 0.1],
                                                  tex_coords: [0.0, 1.0],
                                              },
                                              SpriteVertex {
                                                  position: [0.6, 0.1],
                                                  tex_coords: [1.0, 1.0],
                                              },
                                              SpriteVertex {
                                                  position: [0.6, -0.1],
                                                  tex_coords: [1.0, 0.0],
                                              }]);

    gfx.new_sprite(3,
                   vertex_shader_src,
                   fragment_shader_src,
                   vertex_buffer,
                   None,
                   10);
}
