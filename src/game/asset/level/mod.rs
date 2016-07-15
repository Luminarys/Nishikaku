#[macro_use]
mod macros;

use std::fs::File;
use std::io::Read;
use toml;

use nalgebra::{Vector2, Point2};
use ncollide_geometry::shape::{ShapeHandle2, Ball, ConvexHull};

use std::rc::Rc;
use std::cell::RefCell;
use engine::graphics::{Graphics};

use engine::util::{HashMap};
use engine::util;
use game::asset;
use game::object::level::LevelEvent;
use game::object::level::bullet::{Bullet, Behavior};
use game::object::level::enemy::Enemy;

use game::object::level::spawn::{SpawnBuilder, SpawnType, Spawn};
use game::object::level::path::{PathBuilder, PathType, RotationDirection};
use game::object::level::action::{Action, ActionType};
use game::object::level::pattern::{Angle, PatternBuilder};
use game::object::level::point::Point;

use toml::Value;

pub type Events = HashMap<String, Vec<LevelEvent>>;
pub type Sprites = HashMap<String, usize>;
pub type Enemies = HashMap<String, Enemy>;
pub type Bullets = HashMap<String, Bullet>;

static ZERO: i64 = 0;

pub fn load_level_file(graphics: Rc<RefCell<Graphics>>, path: &str) -> (Result<(Sprites, Enemies, Bullets, Events), String>) {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => { },
        Err(s) => { return Err(format!("{:?}", s)); },
    };
    let mut parser = toml::Parser::new(&s[..]);
    if let Some(level) = parser.parse() {
        println!("Loading level file {:?}!", path);
        parse_level(graphics, level)
    } else {
        return Err(String::from("File does not have valid TOML format"));
    }
}

pub fn parse_level(graphics: Rc<RefCell<Graphics>>, level: toml::Table) -> Result<(Sprites, Enemies, Bullets, Events), String> {
    let mut sprites = util::hashmap();
    let mut enemies = util::hashmap();
    let mut bullets = util::hashmap();
    let mut events = util::hashmap();

    let v = Vec::new();
    for import in tget!(level, "import", Value::Array, "level config", &v) {
        match load_level_file(graphics.clone(), &(String::from("assets/levels/") + import.as_str().unwrap())[..]) {
            Ok((s, e, b, ev)) => {
                sprites.extend(s);
                enemies.extend(e);
                bullets.extend(b);
                events.extend(ev);
            }
            Err(e) => println!("Failed to load file {}, error: {}", import.as_str().unwrap(), e)
        }
    }

    let sprite_tab = tget!(level, "sprites", Value::Table, "level config");
    sprites.extend(try!(load_sprites(graphics, sprite_tab.clone())));

    let enemy_tab = tget!(level, "enemies", Value::Table, "level config");
    enemies.extend(try!(load_enemies(enemy_tab.clone(), &sprites)));

    let bullet_tab = tget!(level, "bullets", Value::Table, "level config");
    bullets.extend(try!(load_bullets(bullet_tab.clone(), &sprites)));

    let event_tab = tget!(level, "level", Value::Table, "level config");
    events.extend(try!(load_events(event_tab.clone(), &enemies, &bullets)));
    Ok((sprites, enemies, bullets, events))
}

fn load_sprites(graphics: Rc<RefCell<Graphics>>, sprite_tab: toml::Table) -> Result<HashMap<String, usize>, String>{
    let mut sprites = util::hashmap();
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
        let id = asset::make_sprite(graphics.clone(),
                                    &(String::from("assets/sprites/") + texture)[..],
                                    half_extents,
                                    max_amount,
                                    shape);
        sprites.insert(sprite_name, id);
    }
    Ok(sprites)
}

fn load_enemies(enemy_tab: toml::Table, sprites: &HashMap<String, usize>) -> Result<Enemies, String> {
    let mut enemies = util::hashmap();
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
    Ok(enemies)
}

fn load_bullets(bullet_tab: toml::Table, sprites: &HashMap<String, usize>) -> Result<Bullets, String> {
    let mut bullets = util::hashmap();
    for (bullet_name, bullet) in bullet_tab {
        let parse_pos = format!("Bullet {:?}", bullet_name);
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
            "deaccel" => {
                let time = tget!(bullet, "deaccel_time", num, parse_pos);
                let amount = tget!(bullet, "deaccel_amount", num, parse_pos);
                Behavior::Deaccelerate(time, amount)
            }
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
    Ok(bullets)
}

fn load_spawn(spawn: toml::Table, enemies: &Enemies, bullets: &Bullets, event_name: String) -> Result<Vec<Spawn>, String> {
    let parse_pos = format!("{:?} spawn", event_name);
    let default_loc = vec![toml::Value::Integer(0), toml::Value::Integer(0)];
    let point = tget!(spawn, "location", Value::Array, parse_pos, &default_loc);
    let location = Vector2::new(tint!(point[0], "spawn location X"),
    tint!(point[1], "spawn location Y"));
    match &tget!(spawn, "type", Value::String, parse_pos)[..] {
        "player" => {
            Ok(vec![Spawn::player(location)])
        }
        "enemy" => {
            let enemy_name = tget!(spawn, "enemy_id", Value::String, parse_pos);
            let enemy = match enemies.get(enemy_name) {
                Some(e) => *e,
                None => {
                    return Err(format!("Invalid enemy {:?} used in {:?}",
                                       enemy_name,
                                       parse_pos))
                }
            };
            let no = false;
            let mirror_x = tget!(spawn, "mirror_x", Value::Boolean, parse_pos, &no);
            let mirror_y = tget!(spawn, "mirror_y", Value::Boolean, parse_pos, &no);
            let repeat = *tget!(spawn, "repeat", Value::Integer, parse_pos, &ZERO) as usize;
            let repeat_delay = tget!(spawn, "repeat_delay", num, parse_pos, 0.0);

            let pattern_tab = tget!(spawn, "pattern", Value::Table, parse_pos);
            let parse_pos = format!("{:?} spawn pattern", event_name);
            let pattern = try!(load_pattern(pattern_tab.clone(), parse_pos.clone()));

            let path_ar = tget!(spawn, "paths", Value::Array, parse_pos);
            let mut paths = Vec::new();
            let mut pn = 1;
            for path in path_ar {
                let path_tab = path.as_table().unwrap();

                let actions = {
                    match path_tab.get("action") {
                        Some(&Value::Array(ref a)) => {
                            let mut actions= Vec::new();
                            let mut actions_arr = a.clone();
                            while let Some(Value::Table(action_tab)) = actions_arr.pop() {
                                let parse_pos = format!("{:?} spawn action", event_name);
                                actions.push(try!(load_action(action_tab.clone(), bullets, parse_pos.clone())));
                            }
                            actions
                        }
                        Some(&Value::Table(ref action_tab)) => {
                            let parse_pos = format!("{:?} spawn action", event_name);
                            let action = try!(load_action(action_tab.clone(), bullets, parse_pos.clone()));
                            vec![action]
                        }
                        _ => vec![]
                    }
                };

                let parse_pos = format!("{:?} spawn path {:?}", event_name, pn);
                let path = try!(load_path(path_tab.clone(), parse_pos)).actions(actions);

                paths.push(path);
                pn += 1;
            }

            let mut spawns = Vec::new();
            let spawn = SpawnBuilder::new()
                .spawn_type(SpawnType::Enemy(enemy))
                .paths(paths)
                .repeat(repeat)
                .repeat_delay(repeat_delay)
                .pattern(pattern)
                .location(location);
            if *mirror_x {
                spawns.push(spawn.clone().mirror_x().build(&Vector2::new(0.0, 0.0), &Vector2::new(0.0, 0.0)));
            }
            if *mirror_y {
                spawns.push(spawn.clone().mirror_y().build(&Vector2::new(0.0, 0.0), &Vector2::new(0.0, 0.0)));
            }
            spawns.push(spawn.build(&Vector2::new(0.0, 0.0), &Vector2::new(0.0, 0.0)));
            Ok(spawns)
        }
        s => Err(format!("Spawn must be 'player' or 'enemy', {:?} is invalid", s)),
    }
}

fn load_action(action: toml::Table, bullets: &Bullets, parse_pos: String) -> Result<Action, String> {
    match &tget!(action, "type", Value::String, parse_pos)[..] {
        "bullets" => {
            let bullet_name = tget!(action, "bullet_id", Value::String, parse_pos);
            let bullet = match bullets.get(bullet_name) {
                Some(e) => *e,
                None => {
                    return Err(format!("Invalid bullet {} used in {}",
                                       bullet_name,
                                       parse_pos))
                }
            };
            let pattern_tab = tget!(action, "pattern", Value::Table, parse_pos);
            let parse_pos = format!("{:?} - bullet pattern", parse_pos);
            let pattern = try!(load_pattern(pattern_tab.clone(), parse_pos.clone()));
            let delay = tget!(action, "delay", num, parse_pos, 0.0);

            Ok(Action {
                action_type: ActionType::Bullets(bullet, pattern),
                delay: delay,
            })
        }
        "none" => {
            Ok(Action {
                action_type: ActionType::None,
                delay: 0.0,
            })
        }
        a => Err(format!("Action must be 'bullets' or 'none', {:?} is invalid", a)),
    }
}

fn load_pattern(pattern: toml::Table, parse_pos: String) -> Result<PatternBuilder, String> {
    let speed = tget!(pattern, "speed", num, parse_pos, 0.0);
    let repeat = *tget!(pattern, "repeat", Value::Integer, parse_pos, &ZERO) as usize;
    let repeat_delay = tget!(pattern, "repeat_delay", num, parse_pos, 0.0);

    let (wobble_angle, wobble_time, wobble_dir) = match pattern.get("wobble") {
        Some(&Value::Table(ref wt)) => {
            let wobble_angle = tget!(wt, "half_angle", num, parse_pos);
            let wobble_time = tget!(wt, "quarter_time", num, parse_pos) * 4.0;
            let wobble_dir = match wt.get("initial_dir") {
                Some(&Value::String(ref s)) => {
                    match &s[..] {
                        "clockwise" => RotationDirection::Clockwise,
                        "counterclockwise" => RotationDirection::CounterClockwise,
                        _ => return Err(format!("Invalid wobble direction defined at {:?}, must be clockwise or counterclockwise", parse_pos)),
                    }
                }
                _ => RotationDirection::CounterClockwise,
            };
            (wobble_angle, wobble_time, wobble_dir)
        }
        _ => (0.0, 1.0, RotationDirection::CounterClockwise),
    };

    match &tget!(pattern, "type", Value::String, parse_pos)[..] {
        "point" => {
            let angle = tget!(pattern, "angle", angle, parse_pos, Angle::Fixed(0.0));
            let amount = *tget!(pattern, "amount", Value::Integer, parse_pos) as usize;
            let time_int = tget!(pattern, "time_int", num, parse_pos, 0.0);

            Ok(PatternBuilder::new()
                .amount(amount)
                .speed(speed)
                .fixed_angle(angle)
                .repeat(repeat)
                .repeat_delay(repeat_delay)
                .wobble(wobble_angle, wobble_time, wobble_dir)
                .time_int(time_int))
        }
        "arc" => {
            let amount = *tget!(pattern, "amount", Value::Integer, parse_pos) as usize;
            let radius = tget!(pattern, "radius", num, parse_pos);
            let astart = tget!(pattern, "astart", angle, parse_pos);
            let aend = tget!(pattern, "aend", angle, parse_pos);
            let time_int = tget!(pattern, "time_int", num, parse_pos);

            Ok(PatternBuilder::new()
                .amount(amount)
                .speed(speed)
                .start_angle(astart)
                .stop_angle(aend)
                .radius(radius)
                .repeat(repeat)
                .repeat_delay(repeat_delay)
                .wobble(wobble_angle, wobble_time, wobble_dir)
                .time_int(time_int))
        }
        _ => {
            Err(format!("Pattern must be 'point' or 'arc', \
                                {:?} is invalid",
                                pattern))
        }
    }
}

fn load_path(path: toml::Table, parse_pos: String) -> Result<PathBuilder, String> {
    let speed = tget!(path, "speed", num, parse_pos, 0.0);
    match &tget!(path, "type", Value::String, parse_pos)[..] {
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
            Ok(PathBuilder::new(PathType::Curve)
                .speed(speed)
                .points(points))
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
            Ok(PathBuilder::new(PathType::Arc)
                .speed(speed)
                .center(center)
                .radius(radius)
                .degrees(degrees)
                .direction(direction))
        }
        "fixed" => {
            let time = tget!(path, "time", num, parse_pos);
            Ok(PathBuilder::new(PathType::Fixed).time(time))
        }
        p => {
            Err(format!("Pattern must be 'curve' or 'arc', {:?} is invalid",
                               p))
        }
    }
}

fn load_events(event_tab: toml::Table, enemies: &Enemies, bullets: &Bullets) -> Result<HashMap<String, Vec<LevelEvent>>, String> {
    let mut events: HashMap<String, Vec<LevelEvent>> = util::hashmap();
    let mut ev_counter = 0;
    let insert_or_app = |map: &mut HashMap<String, Vec<LevelEvent>>, key: String, val: LevelEvent| {
        if map.contains_key(&key) {
            map.get_mut(&key).unwrap().push(val);
        } else {
            map.insert(key, vec![val]);
        }
    };

    for (event_name, event) in event_tab {
        let parse_pos = format!("event {:?}", event_name);
        let event = event.as_table().unwrap();
        let ev_timing = tget!(event, "time", Value::Table, event_name);
        let ev_after = tget!(ev_timing, "after", Value::String, event_name);
        let delay = tget!(ev_timing, "delay", num, event_name);

        let spawn_tab = tget!(event, "spawn", Value::Table, parse_pos);
        let spawns = try!(load_spawn(spawn_tab.clone(), enemies, bullets, event_name.clone()));
        insert_or_app(&mut events, ev_after.clone(),
            LevelEvent {
                name: event_name.clone(),
                id: ev_counter,
                delay: delay,
                spawns: spawns,
            }
        );
        ev_counter += 1;

    }


    Ok(events)
}
