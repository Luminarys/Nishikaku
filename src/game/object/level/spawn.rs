use nalgebra::Vector2;

use game::object::level::path::PathBuilder;
use game::object::level::pattern::{Angle, Pattern, PatternBuilder};
use game::object::level::enemy::Enemy;

#[derive(Clone, Debug)]
pub struct Spawn {
    pub spawn_type: SpawnType,
    pub paths: Vec<PathBuilder>,
    pub pattern: Pattern,
    pub repeat: usize,
    pub repeat_delay: f32,
    pub location: Vector2<f32>,
}

impl Spawn {
    pub fn player(location: Vector2<f32>) -> Spawn {
        let pattern = PatternBuilder::new().fixed_angle(Angle::Fixed(270.0));
        SpawnBuilder::new()
            .spawn_type(SpawnType::Player)
            .pattern(pattern)
            .build(&location, &Vector2::new(0.0, 0.0))
    }
}

#[derive(Clone, Debug)]
pub enum SpawnType {
    Enemy(Enemy),
    Player,
}

#[derive(Clone, Debug)]
pub struct SpawnBuilder {
    spawn_type: Option<SpawnType>,
    paths: Vec<PathBuilder>,
    repeat: usize,
    repeat_delay: Option<f32>,
    pattern: Option<PatternBuilder>,
    location: Vector2<f32>,
}

impl SpawnBuilder {
    pub fn new() -> SpawnBuilder {
        SpawnBuilder {
            spawn_type: None,
            paths: Vec::new(),
            repeat: 0,
            repeat_delay: None,
            pattern: None,
            location: Vector2::new(0.0, 0.0)
        }
    }

    pub fn spawn_type(mut self, spawn_type: SpawnType) -> SpawnBuilder {
        // Perhaps validate the type? might not be known now
        self.spawn_type = Some(spawn_type);
        self
    }

    pub fn pattern(mut self, pattern: PatternBuilder) -> SpawnBuilder {
        self.pattern = Some(pattern);
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
        let mut paths = Vec::new();
        for path in self.paths.iter() {
            paths.push(path.mirror_x());
        }
        self.pattern = Some(self.pattern.unwrap().mirror_x());
        self.paths = paths;
        self.location.x *= -1.0;
        self
    }

    pub fn mirror_y(mut self) -> SpawnBuilder {
        let mut paths = Vec::new();
        for path in self.paths.iter() {
            paths.push(path.mirror_y());
        }
        self.pattern = Some(self.pattern.unwrap().mirror_y());
        self.paths = paths;
        self.location.y *= -1.0;
        self
    }

    pub fn location(mut self, location: Vector2<f32>) -> SpawnBuilder {
        self.location = location;
        self
    }

    pub fn build(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Spawn {
        Spawn {
            spawn_type: self.spawn_type.unwrap(),
            pattern: self.pattern.unwrap().build(current_pos, player_pos),
            paths: self.paths,
            repeat: self.repeat,
            repeat_delay: self.repeat_delay.unwrap_or(0.0),
            location: self.location,
        }
    }
}
