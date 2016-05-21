pub mod component;

use nalgebra::Vector2;

use engine::event::Event;
use engine::Engine;
use engine::graphics::SpriteAttrs;

struct EntityData<D> {
    id: usize,
    velocity: Vector2<f32>,
    texture: Option<String>,
    data: D,
}

struct StateInfo {
    id: usize,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

pub struct RenderInfo {
    pub sprite: usize,
    pub attrs: SpriteAttrs,
}

pub trait Entity {
    fn handle_event(&mut self, e: Event);
    fn render(&self) -> Option<RenderInfo>;
    fn id(&self) -> usize;
}
