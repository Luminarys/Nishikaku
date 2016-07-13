use nalgebra::{Vector2, Translation};
use ncollide_geometry::shape::ShapeHandle2;
use std::rc::Rc;

use engine::entity::Entity;
use engine::scene::{Scene, PhysicsWorld};
use engine::physics::PhysicsEngine;

pub struct PhysicsComp {
    pub id: usize,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub pos: Vector2<f32>,
    pub world: Rc<PhysicsWorld>,
}

impl PhysicsComp {
    pub fn new<E: Entity>(entity_id: usize,
                          tag: usize,
                          position: Vector2<f32>,
                          shape: ShapeHandle2<f32>,
                          group: u8,
                          scene: &Scene<E>)
                          -> PhysicsComp {
        let id = scene.physics.add(position.clone(),
                                           shape,
                                           Rc::new(PhysicsData::new(entity_id, tag, group)));
        PhysicsComp {
            id: id,
            pos: position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            world: scene.physics.clone(),
        }
    }

    pub fn scaler(&self) -> f32 {
        self.world.scaler
    }

    pub fn translate(&mut self, delta: Vector2<f32>) {
        let pos = self.pos.append_translation(&delta);
        self.set_pos(pos);
    }

    pub fn get_pos(&self) -> Vector2<f32> {
        self.pos
    }

    pub fn sync_pos(&self) {
        self.world.set_pos(&self.id, self.pos);
    }

    pub fn set_pos(&mut self, pos: Vector2<f32>) {
        self.pos = pos;
        self.world.set_pos(&self.id, pos);
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let delta = self.velocity * dt;
        self.translate(delta);
    }
}

impl Drop for PhysicsComp {
    fn drop(&mut self) {
        self.world.remove(&self.id);
    }
}

#[derive(Clone, Default)]
pub struct PhysicsData {
    pub entity_id: usize,
    pub tag: usize,
    pub group: u8,
}

impl PhysicsData {
    pub fn new(entity_id: usize, tag: usize, group: u8) -> PhysicsData {
        PhysicsData {
            entity_id: entity_id,
            tag: tag,
            group: group,
        }
    }
}
