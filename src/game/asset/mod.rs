pub mod level;

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use engine::graphics::{Graphics};

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

pub fn load_assets(engine: &mut Engine<Object>) {
    println!("Loading assets!");
    load_char(engine);
    load_bullet(engine);
    load_sound(engine);
    load_fonts(engine);
}

fn load_level(engine: &mut Engine<Object>) -> Result<HashMap<String, Vec<LevelEvent>>, String> {
    match self::level::load_level_file(engine.graphics.clone(), "assets/level.toml") {
        Ok((_, _, _, events)) => Ok(events),
        Err(e) => Err(e)
    }
}

fn load_fonts(engine: &mut Engine<Object>) {
    println!("Loading fonts!");
    engine.graphics.borrow_mut().load_font(1, "assets/fonts/OxygenMono-Regular.ttf");
}

fn load_sound(engine: &mut Engine<Object>) {
    println!("Loading songs!");
    let path = Path::new("assets/audio/main.mp3");
    engine.audio.borrow_mut().load(1, path);
}

pub fn make_sprite(graphics: Rc<RefCell<Graphics>>,
               texture: &str,
               half_extents: Vector2<f32>,
               amount: usize,
               shape: ShapeHandle2<f32>)
               -> usize {
    let vert_shader = SPRITE_VERT_SHADER;
    let frag_shader = SPRITE_FRAG_SHADER;
    let vbo = make_vbo(graphics.clone(), half_extents);
    let mut gfx = graphics.borrow_mut();
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

fn make_vbo(graphics: Rc<RefCell<Graphics>>, half_extents: Vector2<f32>) -> VertexBuffer<SpriteVertex> {
    let half_extents = half_extents / graphics.borrow().scaler;
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
    let gfx = graphics.borrow();
    gfx.make_sprite_vbo(vertices)
}

fn load_char(engine: &mut Engine<Object>) {
    let shape = ShapeHandle2::new(Ball::new(5.0));
    make_sprite(engine.graphics.clone(),
                "assets/sprites/sakuya.png",
                Vector2::new(25.0, 50.0),
                1,
                shape);
}

fn load_bullet(engine: &mut Engine<Object>) {
    let shape = ShapeHandle2::new(Ball::new(5.0));
    make_sprite(engine.graphics.clone(),
                "assets/sprites/bullet.png",
                Vector2::new(2.5, 2.5),
                300,
                shape);
}
