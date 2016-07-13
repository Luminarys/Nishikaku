mod level;

use std::path::Path;

use nalgebra::{Vector2};
use ncollide_geometry::shape::{ShapeHandle2, Ball};
use glium::VertexBuffer;

use engine::Engine;
use engine::util::{HashMap};
use engine::graphics::SpriteVertex;
use game::object::Object;
use game::object::level::LevelEvent;

const SPRITE_VERT_SHADER: &'static str = r#"
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

const SPRITE_FRAG_SHADER: &'static str = r#"
    #version 140

    uniform sampler2D tex;
    in vec2 v_tex_coords;
    out vec4 color;

    void main() {
        color = texture(tex, v_tex_coords);
    }
"#;

pub struct Assets {
    pub levels: Vec<HashMap<String, Vec<LevelEvent>>>
}

pub fn load_assets(engine: &mut Engine<Object>) -> Assets {
    println!("Loading assets!");
    load_char(engine);
    load_bullet(engine);
    load_menu(engine);
    load_sound(engine);
    load_fonts(engine);
    let level = load_level(engine);
    Assets {
        levels: vec![level],
    }
}

fn load_level(engine: &mut Engine<Object>) -> HashMap<String, Vec<LevelEvent>> {
    let (_, _, _, events) = self::level::load_level_file(engine, "assets/level.toml").unwrap();
    events
}

fn load_fonts(engine: &mut Engine<Object>) {
    println!("Loading fonts!");
    engine.graphics.borrow_mut().load_font(1, "assets/OxygenMono-Regular.ttf");
}

fn load_sound(engine: &mut Engine<Object>) {
    println!("Loading songs!");
    let path = Path::new("assets/main.mp3");
    engine.audio.borrow_mut().load(1, path);
}

pub fn make_sprite(engine: &mut Engine<Object>,
               texture: &str,
               half_extents: Vector2<f32>,
               amount: usize,
               shape: ShapeHandle2<f32>)
               -> usize {
    let vert_shader = SPRITE_VERT_SHADER;
    let frag_shader = SPRITE_FRAG_SHADER;
    let vbo = make_vbo(engine, half_extents);
    let mut gfx = engine.graphics.borrow_mut();
    let texture = gfx.load_texture(&texture[..]);
    let id = gfx.sprite_amount() + 1;
    gfx.new_sprite(id,
                   vert_shader,
                   frag_shader,
                   vbo,
                   Some(texture),
                   amount,
                   Some(shape));
    id
}

fn make_vbo(engine: &mut Engine<Object>, half_extents: Vector2<f32>) -> VertexBuffer<SpriteVertex> {
    let half_extents = half_extents / engine.scene.physics.scaler;
    let vertices = &[SpriteVertex {
                         position: [-1.0 * half_extents.x, -1.0 * half_extents.y],
                         tex_coords: [0.0, 0.0],
                     },
                     SpriteVertex {
                         position: [-1.0 * half_extents.x, half_extents.y],
                         tex_coords: [0.0, 1.0],
                     },
                     SpriteVertex {
                         position: [half_extents.x, half_extents.y],
                         tex_coords: [1.0, 1.0],
                     },
                     SpriteVertex {
                         position: [half_extents.x, -1.0 * half_extents.y],
                         tex_coords: [1.0, 0.0],
                     }];
    let gfx = engine.graphics.borrow();
    gfx.make_sprite_vbo(vertices)
}

fn load_char(engine: &mut Engine<Object>) {
    let shape = ShapeHandle2::new(Ball::new(5.0));
    make_sprite(engine,
                "assets/sakuya.png",
                Vector2::new(25.0, 50.0),
                1,
                shape);
}

fn load_bullet(engine: &mut Engine<Object>) {
    let shape = ShapeHandle2::new(Ball::new(5.0));
    make_sprite(engine,
                "assets/bullet.png",
                Vector2::new(2.5, 2.5),
                300,
                shape);
}

fn load_menu(engine: &mut Engine<Object>) {
    let vertex_shader_src = SPRITE_VERT_SHADER;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.319,0.345,0.312,1.000);
        }
    "#;

    let vertex_buffer = make_vbo(engine, Vector2::new(120.0, 20.0));
    let mut gfx = engine.graphics.borrow_mut();
    gfx.new_sprite(3,
                   vertex_shader_src,
                   fragment_shader_src,
                   vertex_buffer,
                   None,
                   10,
                   None);
}
