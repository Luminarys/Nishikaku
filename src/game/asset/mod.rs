use std::path::Path;
use std::fs::File;
use toml;

use nalgebra::{Vector2, Point2};
use ncollide::shape::{ShapeHandle2, Ball, Cuboid, ConvexHull};
use glium::VertexBuffer;

use engine::Engine;
use engine::util;
use engine::graphics::SpriteVertex;
use game::object::Object;

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
    load_level(engine);
    load_menu(engine);
    load_sound(engine);
    load_fonts(engine);
}

fn load_level(engine: &mut Engine<Object>) {
    use std::io::Read;
    let mut f = File::open("assets/level.toml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let mut parser = toml::Parser::new(&s[..]);
    match parser.parse() {
        Some(level) => {
            println!("{:?}", parse_level(engine, level).unwrap());
        }
        None => {
            println!("{:?}", parser.errors);
        }
    };
}

macro_rules! tget {
    ($t:expr, $k:expr, angle, $s:expr) => {
        match $t.get($k) {
            Some(v) => tangle!(v.clone(), $s),
            _ => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, point, $s:expr) => {
        match $t.get($k) {
            Some(ref v) => {
                if !v.as_slice().is_none() {
                    tpoint!(v.clone(), $s)
                } else {
                    return Err(format!("Expected array, got {:?}!", v))
                }
            }
            _ => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, num, $s:expr) => {
        match $t.get($k) {
            Some(v) => tint!(v.clone(), $s),
            None => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, $vt:path, $s:expr) => {
        match $t.get($k) {
            Some(v) => {
                match v {
                    &$vt(ref r) => r,
                    _ => return Err(format!("Could find key {:?} of correct type in {:?}!", $k, $s))
                }
            }
            None => return Err(format!("Could find key {:?} in {:?}!", $k, $s))
        }
    };

    ($t:expr, $k:expr, num, $s:expr, $d:expr) => {
        match $t.get($k) {
            Some(v) => tint!(v.clone(), $s),
            None => $d
        }
    };
    ($t:expr, $k:expr, $vt:path, $s:expr, $d:expr) => {
        match $t.get($k) {
            Some(v) => {
                match v {
                    &$vt(ref r) => r,
                    _ => return Err(format!("Could find key {:?} of correct type in {:?}!", $k, $s))
                }
            }
            None => $d
        }
    };
}

macro_rules! tint {
    ($v:expr, $s:expr) => {
        match $v.clone() {
            toml::Value::Integer(i) => i as f32,
            toml::Value::Float(f) => f as f32,
            t => return Err(format!("{:?} was expected to be integer or float, found as {:?}", $s, t))
        }
    }
}

macro_rules! tpoint {
    ($v:expr, $s:expr) => {
        match $v.clone() {
            toml::Value::Array(a) => {
                match &a[0] {
                    &toml::Value::String(ref s) => {
                        let x = if a.len() > 1 {
                            tint!(a[1], $s)
                        } else {
                            0.0
                        };
                        let y = if a.len() > 2 {
                            tint!(a[2], $s)
                        } else {
                            0.0
                        };
                        if s == "current" {
                            Point::Current(Vector2::new(x, y))
                        } else if s == "player" {
                            Point::Player(Vector2::new(x, y))
                        } else {
                            return Err(format!("{:?} was expected to be current or player, found as {:?}", $s, s))
                        }
                    }
                    &toml::Value::Integer(_) | &toml::Value::Float(_) => {
                        let x = tint!(a[0], $s);
                        let y = tint!(a[1], $s);
                        Point::Fixed(Vector2::new(x, y))
                    }
                    t => return Err(format!("{:?} was expected to have a first value of 'current', 'player', or a number, found as {:?}", $s, t))
                }
            }
            t => return Err(format!("{:?} was expected to be a point(must use array brackets), found as {:?}", $s, t))
        }
    }
}

macro_rules! tangle {
    ($v:expr, $s:expr) => {
        match $v.clone() {
            toml::Value::String(s) => {
                if s == String::from("player") {
                    Angle::Player(0.0)
                } else {
                    return Err(format!("Angles currently only support player for custom values"))
                }
            }
            toml::Value::Array(a) => {
                match &a[0] {
                    &toml::Value::String(ref s) => {
                        if *s == String::from("player") {
                            Angle::Player(tint!(a[1], $s))
                        } else {
                            return Err(format!("Angles currently only support player for custom values"))
                        }
                    }
                    t => {
                        return Err(format!("{:?} was expected to be a string, number, or array of string/number, found as {:?}", $s, t))
                    }
                }
            }
            toml::Value::Integer(_) | toml::Value::Float(_) => {
                Angle::Fixed(tint!($v, $s))
            }
            t => return Err(format!("{:?} was expected to be a point(must use array brackets), found as {:?}", $s, t))
        }
    }
}

fn parse_level(engine: &mut Engine<Object>, level: toml::Table) -> Result<(), String> {
    use toml::Value;

    // Load sprites
    let mut sprites = util::hashmap();
    let sprite_tab = tget!(level, "sprites", Value::Table, "level config");
    for (sprite_name, sprite) in sprite_tab {
        let sprite = sprite.as_table().unwrap();
        let texture = tget!(sprite, "texture", Value::String, sprite_name);
        let size = tget!(sprite, "size", Value::Array, sprite_name);
        let half_extents = Vector2::new(tint!(size[0], "Sprite x extant") / 2.0,
                                        tint!(size[0], "Sprite y extant") / 2.0);
        let max_amount = *tget!(sprite, "max_amount", Value::Integer, sprite_name) as usize;
        let shape = match &tget!(sprite, "hitbox", Value::String, sprite_name)[..] {
            "sphere" => {
                let radius = tget!(sprite, "radius", num, sprite_name);
                ShapeHandle2::new(Ball::new(radius))
            }
            "points" => {
                let points = tget!(sprite, "hitbox_bounds", Value::Array, sprite_name);
                let mut conv_points = Vec::new();
                for point in points {
                    let point = point.as_slice().unwrap();
                    let point = Point2::new(tint!(point[0], "Sprite hitbox point"),
                                            tint!(point[1], "Sprite hitbox point"));
                    conv_points.push(point);
                }
                ShapeHandle2::new(ConvexHull::new(conv_points))
            }
            _ => {
                return Err(format!("Only sphere/points are supported as hitboxes right now. \
                                    Please change the hitbox in {:?}",
                                   sprite_name))
            }
        };
        let id = make_sprite(engine,
                             &(String::from("assets/") + texture)[..],
                             half_extents,
                             max_amount,
                             shape);
        sprites.insert(sprite_name, id);
    }

    // Load enemies
    use game::object::level::enemy::Enemy;
    let mut enemies = util::hashmap();
    let enemy_tab = tget!(level, "enemies", Value::Table, "level config");
    for (enemy_name, enemy) in enemy_tab {
        let enemy = enemy.as_table().unwrap();
        let health = *tget!(enemy, "health", Value::Integer, enemy_name) as usize;
        let damage = *tget!(enemy, "damage", Value::Integer, enemy_name) as usize;
        let sprite_name = tget!(enemy, "sprite", Value::String, enemy_name);
        let sprite = match sprites.get(sprite_name) {
            Some(s) => *s,
            None => {
                return Err(format!("Invalid sprite {:?} used for enemy {:?}",
                                   sprite_name,
                                   enemy_name))
            }
        };
        enemies.insert(enemy_name,
                       Enemy {
                           sprite: sprite,
                           health: health,
                           damage: damage,
                       });
    }

    // Load bullets
    use game::object::level::bullet::{Bullet, Behavior};
    let mut bullets = util::hashmap();
    let bullet_tab = tget!(level, "bullets", Value::Table, "level config");
    for (bullet_name, bullet) in bullet_tab {
        let bullet = bullet.as_table().unwrap();
        let damage = *tget!(bullet, "damage", Value::Integer, bullet_name) as usize;
        let sprite_name = tget!(bullet, "sprite", Value::String, bullet_name);
        let sprite = match sprites.get(sprite_name) {
            Some(s) => *s,
            None => {
                return Err(format!("Invalid sprite {:?} used for enemy {:?}",
                                   sprite_name,
                                   bullet_name))
            }
        };
        let behavior = match &tget!(bullet, "behavior", Value::String, bullet_name)[..] {
            "straight" => Behavior::Straight,
            b => {
                return Err(format!("Invalid bullet behavior {:?} specified for bullet {:?}",
                                   b,
                                   bullet_name))
            }
        };
        bullets.insert(bullet_name,
                       Bullet {
                           sprite: sprite,
                           behavior: behavior,
                           damage: damage,
                       });
    }

    // [level]
    //
    // [level.pstart]
    // time = { after = "start", delay = 0.0 }
    // spawn = { type = "player", init_pos = [0, -180] }
    //
    // [level.wave1]
    // time = {after = "start", delay = 5.0}
    //
    // [level.wave1.spawn]
    // type = "enemy"
    // enemy_id = "basic_curve"
    // pattern = { type = "point", position = [-200, 180], amount = 5, delay = 0.5 }
    // repeat = 1
    // repeat_delay = 1.0
    // contact_dmg = 6
    // mirror_x = true
    //
    // [[level.wave1.spawn.paths]]
    // type = "curve"
    // points = [[-200, 180], [-100, 140], [-200, 100]]
    // speed = 40
    //
    // [[level.wave1.spawn.paths.action]]
    // type = "bullets"
    // time_start = 1.0
    // pattern = { type = "point", bullet_id = "basic_straight", angle = 270, speed = 40, amount = 5, time_int = 0.5 }
    use game::object::level::LevelEvent;
    use game::object::level::spawn::{SpawnBuilder, SpawnType, Spawn};
    use game::object::level::path::{PathBuilder, PathType, RotationDirection};
    use game::object::level::action::{Action, ActionType};
    use game::object::level::pattern::{Angle, PatternBuilder};
    use game::object::level::point::Point;

    let mut events = util::hashmap();
    let mut ev_counter = 0;
    let event_tab = tget!(level, "level", Value::Table, "level config");
    for (event_name, event) in event_tab {
        let event = event.as_table().unwrap();
        let ev_timing = tget!(event, "time", Value::Table, event_name);
        let ev_after = tget!(ev_timing, "after", Value::String, event_name);
        let delay = tget!(ev_timing, "delay", num, event_name);
        let spawn = tget!(event, "spawn", Value::Table, event_name);
        let parse_pos = format!("{:?} spawn", event_name);
        match &tget!(spawn, "type", Value::String, parse_pos)[..] {
            "player" => {
                let point = tget!(spawn, "position", Value::Array, parse_pos);
                let location = Vector2::new(tint!(point[0], "spawn location X"),
                                            tint!(point[1], "spawn location Y"));
                events.insert(ev_after,
                              LevelEvent {
                                  name: event_name.clone(),
                                  id: ev_counter,
                                  delay: delay,
                                  spawns: vec![Spawn::player(location)],
                              });
                ev_counter += 1;
            }
            "enemy" => {
                // type = "enemy"
                // enemy_id = "basic_curve"
                // pattern = { type = "point", position = [-200, 180], amount = 5, delay = 0.5 }
                // repeat = 1
                // repeat_delay = 1.0
                // mirror_x = true
                let enemy_name = tget!(spawn, "enemy_id", Value::String, parse_pos);
                let enemy = match enemies.get(enemy_name) {
                    Some(e) => *e,
                    None => {
                        return Err(format!("Invalid enemy {:?} used in {:?}",
                                           enemy_name,
                                           parse_pos))
                    }
                };
                static zero: i64 = 0;
                static no: bool = false;
                let mirror_x = tget!(spawn, "mirror_x", Value::Boolean, parse_pos, &no);
                let mirror_y = tget!(spawn, "mirror_y", Value::Boolean, parse_pos, &no);
                let repeat = *tget!(spawn, "repeat", Value::Integer, parse_pos, &zero) as usize;
                let repeat_delay = tget!(spawn, "repeat_delay", num, parse_pos, 0.0);

                let pattern = tget!(spawn, "pattern", Value::Table, parse_pos);
                let pattern = match &tget!(pattern, "type", Value::String, parse_pos)[..] {
                    "point" => {
                        let parse_pos = format!("{:?} spawn pattern", event_name);
                        // pattern = { type = "point", position = [-200, 180], amount = 5, delay = 0.5 }
                        let point = tget!(pattern, "position", Value::Array, parse_pos);
                        let location = Vector2::new(tint!(point[0], "Enemy spawn X"),
                                                    tint!(point[1], "Enemy spawn Y"));
                        let amount = *tget!(pattern, "amount", Value::Integer, parse_pos) as usize;
                        let time_int = tget!(pattern, "time_int", num, parse_pos);

                        PatternBuilder::new()
                            .amount(amount)
                            .center(location)
                            .fixed_angle(Angle::Fixed(270.0))
                            .time_int(time_int)
                    }
                    "arc" => {
                        let parse_pos = format!("{:?} spawn pattern", event_name);
                        // pattern = { type = "arc", center = [-50, 50], radius = 25, astart = 0, end = 360, amount = 10, time_int: 0 }
                        let point = tget!(pattern, "center", Value::Array, parse_pos);
                        let location = Vector2::new(tint!(point[0], "Enemy spawn X"),
                                                    tint!(point[1], "Enemy spawn Y"));
                        let amount = *tget!(pattern, "amount", Value::Integer, parse_pos) as usize;
                        let radius = tget!(pattern, "radius", num, parse_pos);
                        let astart = tget!(pattern, "astart", num, parse_pos);
                        let aend = tget!(pattern, "aend", num, parse_pos);
                        let time_int = tget!(pattern, "time_int", num, parse_pos);

                        PatternBuilder::new()
                            .amount(amount)
                            .center(location)
                            .start_angle(Angle::Fixed(astart))
                            .stop_angle(Angle::Fixed(aend))
                            .radius(radius)
                            .time_int(time_int)
                    }
                    s => {
                        return Err(format!("You cannot use pattern type {:?}! Please use 'arc' \
                                            or 'point'",
                                           s));
                    }
                };

                // [[level.wave1.spawn.paths]]
                // type = "curve"
                // points = [[-200, 180], [-100, 140], [-200, 100]]
                // speed = 40
                let path_ar = tget!(spawn, "paths", Value::Array, parse_pos);
                let mut paths = Vec::new();
                let mut pn = 1;
                for path in path_ar {
                    let parse_pos = format!("{:?} spawn path {:?}", event_name, pn);
                    let path = path.as_table().unwrap();

                    let speed = tget!(path, "speed", num, parse_pos);

                    // [[level.wave1.spawn.paths.action]]
                    // type = "bullets"
                    // time_start = 1.0
                    // pattern = { type = "point", bullet_id = "basic_straight", angle = 270, speed = 40, amount = 5, time_int = 0.5 }
                    let action = tget!(path, "action", Value::Table, parse_pos);
                    let action = match &tget!(action, "type", Value::String, parse_pos)[..] {
                        "bullets" => {
                            let bullet_name = tget!(action, "bullet_id", Value::String, parse_pos);
                            let bullet = match bullets.get(bullet_name) {
                                Some(e) => *e,
                                None => {
                                    return Err(format!("Invalid enemy {:?} used in {:?}",
                                                       enemy_name,
                                                       parse_pos))
                                }
                            };
                            let parse_pos = format!("{:?} spawn action pattern", event_name);
                            // pattern = { type = "arc", center = [-50, 50], radius = 25, astart = 0, end = 360, amount = 10, time_int: 0 }
                            let pattern = tget!(action, "pattern", Value::Table, parse_pos);
                            let speed = tget!(pattern, "speed", num, parse_pos);
                            let delay = tget!(pattern, "delay", num, parse_pos, 0.0);
                            let pattern =
                                match &tget!(pattern, "type", Value::String, parse_pos)[..] {
                                    "point" => {
                                        // pattern = { type = "point", bullet_id = "basic_straight", angle = 270, speed = 40, amount = 5, time_int = 0.5 }
                                        let parse_pos = format!("{:?} bullet pattern", event_name);
                                        // pattern = { type = "point", position = [-200, 180], amount = 5, delay = 0.5 }
                                        let angle = tget!(pattern, "angle", angle, parse_pos);
                                        let amount = *tget!(pattern, "amount", Value::Integer, parse_pos) as usize;
                                        let time_int = tget!(pattern, "time_int", num, parse_pos);

                                        PatternBuilder::new()
                                            .amount(amount)
                                            .speed(speed)
                                            .fixed_angle(angle)
                                            .time_int(time_int)
                                    }
                                    "arc" => {
                                        let parse_pos = format!("{:?} spawn pattern", event_name);
                                        // pattern = { type = "arc", amount = 5, astart = 240.0, aend = 300.0, speed = 40, time_int = 0.0, radius = 10.0 }

                                        let amount = *tget!(pattern, "amount", Value::Integer, parse_pos) as usize;
                                        let radius = tget!(pattern, "radius", num, parse_pos);
                                        let astart = tget!(pattern, "astart", angle, parse_pos);
                                        let aend = tget!(pattern, "aend", angle, parse_pos);
                                        let time_int = tget!(pattern, "time_int", num, parse_pos);

                                        PatternBuilder::new()
                                            .amount(amount)
                                            .speed(speed)
                                            .start_angle(astart)
                                            .stop_angle(aend)
                                            .radius(radius)
                                            .time_int(time_int)
                                    }
                                    _ => {
                                        return Err(format!("Pattern must be 'point' or 'arc', \
                                                            {:?} is invalid",
                                                           pattern))
                                    }
                                };
                            Action {
                                action_type: ActionType::Bullets(bullet, pattern),
                                delay: delay,
                            }
                        }
                        "none" => {
                            Action {
                                action_type: ActionType::None,
                                delay: 0.0,
                            }
                        }
                        a => return Err(format!("Action must be 'bullets' or 'none', {:?} is invalid", a)),
                    };

                    let path = match &tget!(path, "type", Value::String, parse_pos)[..] {
                        // pub path_type: PathType,
                        // speed: Option<f32>,
                        // actions: Vec<Action>,
                        // // Arc info
                        // center: Option<Point>,
                        // radius: Option<f32>,
                        // degrees: Option<f32>,
                        // direction: Option<RotationDirection>,
                        // // Curve info
                        // points: Option<Vec<Point>>,
                        "curve" => {
                            let point_arr = tget!(path, "points", Value::Array, parse_pos);
                            let mut points = Vec::new();
                            for point in point_arr {
                                points.push(tpoint!(point, format!("{:?} points", parse_pos)));
                            }
                            PathBuilder::new(PathType::Curve)
                                .speed(speed)
                                .points(points)
                                .actions(vec![action])
                        }
                        "arc" => {
                            let center = tget!(path, "center", point, parse_pos);
                            let radius = tget!(path, "radius", num, parse_pos);
                            let direction =
                                match &tget!(path, "direction", Value::String, parse_pos)[..] {
                                    "clockwise" => RotationDirection::Clockwise,
                                    "counterclockwise" => RotationDirection::CounterClockwise,
                                    _ => {
                                        return Err(format!("Rotation directions must either be \
                                                            clockwise or counterclockwise"))
                                    }
                                };
                            let degrees = tget!(path, "degrees", num, parse_pos);
                            PathBuilder::new(PathType::Arc)
                                .speed(speed)
                                .center(center)
                                .radius(radius)
                                .degrees(degrees)
                                .direction(direction)
                                .actions(vec![action])
                        }
                        p => {
                            return Err(format!("Pattern must be 'curve' or 'arc', {:?} is invalid",
                                               p))
                        }
                    };
                    paths.push(path);
                    pn += 1;
                }
                let spawn = SpawnBuilder::new()
                                .spawn_type(SpawnType::Enemy(enemy))
                                .paths(paths)
                                .repeat(repeat)
                                .repeat_delay(repeat_delay)
                                .pattern(pattern)
                                .mirror_x(mirror_x)
                                .mirror_y(mirror_y)
                                .build(&Vector2::new(0.0, 0.0), &Vector2::new(0.0, 0.0));
                events.insert(ev_after,
                              LevelEvent {
                                  name: event_name.clone(),
                                  id: ev_counter,
                                  delay: delay,
                                  spawns: vec![spawn],
                              });
            }
            s => return Err(format!("Spawn must be 'player' or 'enemy', {:?} is invalid", s)),
        };
    }

    println!("{:?}", enemies);
    println!("{:?}", bullets);
    println!("{:?}", events);
    Ok(())
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

fn make_sprite(engine: &mut Engine<Object>,
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
    let shape = ShapeHandle2::new(Cuboid::new(Vector2::new(25.0, 50.0)));
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
