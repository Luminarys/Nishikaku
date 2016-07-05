use nalgebra::Vector2;

use game::object::level::path::PathBuilder;
use game::object::level::pattern::{Angle, Pattern, PatternBuilder};
use game::object::level::Point;
use game::object::level::enemy::Enemy;

#[derive(Clone)]
pub struct Spawn {
    pub spawn_type: SpawnType,
    pub paths: Vec<PathBuilder>,
    pub pattern: Pattern,
    pub repeat: usize,
    pub repeat_delay: f32,
    pub mirror_x: bool,
    pub mirror_y: bool,
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

#[derive(Clone)]
pub enum SpawnType {
    Enemy(Enemy),
    Player,
}

pub struct SpawnBuilder {
    spawn_type: Option<SpawnType>,
    paths: Vec<PathBuilder>,
    repeat: usize,
    repeat_delay: Option<f32>,
    pattern: Option<PatternBuilder>,
    mirror_x: bool,
    mirror_y: bool,
}

impl SpawnBuilder {
    pub fn new() -> SpawnBuilder {
        SpawnBuilder {
            spawn_type: None,
            paths: Vec::new(),
            repeat: 0,
            repeat_delay: None,
            pattern: None,
            mirror_x: false,
            mirror_y: false,
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

    pub fn mirror_x(mut self, mirror: &bool) -> SpawnBuilder {
        self.mirror_x = *mirror;
        self
    }

    pub fn mirror_y(mut self, mirror: &bool) -> SpawnBuilder {
        self.mirror_y = *mirror;
        self
    }

    pub fn build(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Spawn {
        Spawn {
            spawn_type: self.spawn_type.unwrap(),
            pattern: self.pattern.unwrap().build(current_pos, player_pos),
            paths: self.paths,
            repeat: self.repeat,
            repeat_delay: self.repeat_delay.unwrap_or(0.0),
            mirror_x: self.mirror_x,
            mirror_y: self.mirror_y,
        }
    }
}
