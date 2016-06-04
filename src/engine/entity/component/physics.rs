use nalgebra::{Vector2, Isometry2, Translation};
use ncollide::shape::ShapeHandle2;
use ncollide::world::GeometricQueryType;
use std::rc::Rc;

use engine::entity::Entity;
use engine::scene::{Scene, PhysicsWorld, PhysicsInteraction};

pub struct PhysicsComp {
    id: usize,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    world: Rc<PhysicsWorld>,
}

impl PhysicsComp {
    pub fn new<E: Entity>(entity_id: usize,
                          tag: usize,
                          position: Vector2<f32>,
                          shape: ShapeHandle2<f32>,
                          interactivity: PhysicsInteraction,
                          query: GeometricQueryType<f32>,
                          scene: &Scene<E>)
                          -> PhysicsComp {
        let id = scene.physics.add(position,
                                           shape,
                                           interactivity,
                                           query,
                                           Rc::new(PhysicsData::new(entity_id, tag)));
        PhysicsComp {
            id: id,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            world: scene.physics.clone(),
        }
    }

    pub fn translate(&self, delta: Vector2<f32>) {
        let pos = self.get_pos().append_translation(&delta);
        self.set_pos(pos);
    }

    pub fn get_pos(&self) -> Isometry2<f32> {
        self.world.get_pos(self.id).unwrap()
    }

    pub fn set_pos(&self, pos: Isometry2<f32>) {
        self.world.set_pos(self.id, pos);
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.translate(self.velocity * dt);
    }
}

impl Drop for PhysicsComp {
    fn drop(&mut self) {
        self.world.remove(self.id);
    }
}

#[derive(Clone, Default)]
pub struct PhysicsData {
    pub entity_id: usize,
    pub tag: usize,
}

impl PhysicsData {
    pub fn new(entity_id: usize, tag: usize) -> PhysicsData {
        PhysicsData {
            entity_id: entity_id,
            tag: tag,
        }
    }
}
