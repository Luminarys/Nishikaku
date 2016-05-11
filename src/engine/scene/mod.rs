pub mod entity;

use ncollide::world::{CollisionWorld, CollisionGroups, GeometricQueryType, CollisionObject2};
use nalgebra::{Isometry2, Point2};

pub struct Scene {
    world: CollisionWorld<Point2<f32>, Isometry2<f32>, entity::Entity>
}
