use nalgebra::Vector2;

use game::object::level::path::PathBuilder;
use game::object::level::Point;

pub struct Spawn {
    pub spawn_type: SpawnType,
    pub location: Vector2<f32>,
    pub paths: Vec<PathBuilder>,
    pub repeat: usize,
    pub repeat_delay: f32,
    pub mirror_x: bool,
    pub mirror_y: bool,
}

pub enum SpawnType {
    Enemy(String),
    Player,
}

pub struct SpawnBuilder {
    spawn_type: Option<SpawnType>,
    location: Option<Point>,
    paths: Vec<PathBuilder>,
    repeat: usize,
    repeat_delay: Option<f32>,
    mirror_x: bool,
    mirror_y: bool,
}

impl SpawnBuilder {
    pub fn new() -> SpawnBuilder {
        SpawnBuilder {
            spawn_type: None,
            location: None,
            paths: Vec::new(),
            repeat: 0,
            repeat_delay: None,
            mirror_x: false,
            mirror_y: false,
        }
    }

    pub fn spawn_type(mut self, spawn_type: SpawnType) -> SpawnBuilder {
        // Perhaps validate the type? might not be known now
        self.spawn_type = Some(spawn_type);
        self
    }

    pub fn location(mut self, location: Point) -> SpawnBuilder {
        self.location = Some(location);
        self
    }

    pub fn paths(mut self, mut paths: Vec<PathBuilder>) -> SpawnBuilder {
        self.paths.append(&mut paths);
        self
    }

    pub fn repeat(mut self, repeat: usize) -> SpawnBuilder {
        self.repeat = repeat;
        self
    }

    pub fn repeat_delay(mut self, repeat_delay: f32) -> SpawnBuilder {
        self.repeat_delay = Some(repeat_delay);
        self
    }

    pub fn mirror_x(mut self) -> SpawnBuilder {
        self.mirror_x = true;
        self
    }

    pub fn mirror_y(mut self) -> SpawnBuilder {
        self.mirror_y = true;
        self
    }

    pub fn build(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Spawn {
        Spawn {
            spawn_type: self.spawn_type.unwrap(),
            location: self.location.unwrap().eval(current_pos, player_pos),
            paths: self.paths,
            repeat: self.repeat,
            repeat_delay: self.repeat_delay.unwrap(),
            mirror_x: self.mirror_x,
            mirror_y: self.mirror_y,
        }
    }
}
