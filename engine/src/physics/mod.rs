use nalgebra::Vector2;
use std::rc::Rc;

pub trait PhysicsEngine<S, D> {
    fn update(&mut self, dt: f32) -> Vec<Collision<D>>;
    fn remove(&mut self, id: &usize);
    fn add(&mut self, id: usize, pos: Vector2<f32>, shape: S, data: Rc<D>);
    fn get_pos(&self, id: &usize) -> Option<Vector2<f32>>;
    fn set_pos(&mut self, id: &usize, pos: Vector2<f32>);
}

#[derive(Clone)]
pub struct Collision<D> {
    pub id1: usize,
    pub id2: usize,
    pub data1: Rc<D>,
    pub data2: Rc<D>,
}
