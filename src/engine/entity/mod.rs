pub mod component;

use nalgebra::Vector2;

use engine::event::Event;

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
    pos: Vector2<f32>,
    texture: String,
}

pub enum EntityFlags {
    ChangePosition(Vector2<f32>),
    Destroy,
}

pub trait Entity {
    fn handle_event(&mut self, e: Event) -> Vec<EntityFlags>;
    fn render_info(&self) -> RenderInfo;
}
