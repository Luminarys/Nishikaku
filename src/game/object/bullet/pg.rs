use engine::entity::component::GraphicsComp;
use game::object::player::PLAYER_POSITION;

use nalgebra::{Vector1, Vector2, Point2, Isometry2, Translation};
use ncollide_geometry::bounding_volume::{self, AABB, BoundingVolume};
use ncollide::shape::ShapeHandle2;
use ncollide::world::GeometricQueryType;
use std::rc::Rc;

use engine::entity::Entity;
use engine::entity::component::PhysicsData;
use engine::scene::{Scene, PhysicsWorld, PhysicsInteraction};

/// Specialized PGComp for bullets which manages their bounding box
pub struct PGComp {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    physics: Vec<PhysicsComp>,
    graphics: GraphicsComp,
    world: Rc<PhysicsWorld>,
    pub scaler: f32,
    half_widths: (f32, f32),
}

impl PGComp {
    pub fn new(graphics: GraphicsComp,
               physics: Vec<PhysicsComp>,
               world: Rc<PhysicsWorld>)
        -> PGComp {
        PGComp {
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            graphics: graphics,
            physics: physics,
            scaler: world.scaler.clone(),
            world: world,
            half_widths: (0.0, 0.0),
        }
    }

    pub fn render(&self) {
        self.graphics.render();
    }

    pub fn translate_gfx(&mut self, delta: Vector2<f32>) {
        self.graphics.translate(delta.x / self.scaler, delta.y / self.scaler);
    }

    pub fn translate(&mut self, delta: Vector2<f32>) {
        self.graphics.translate(delta.x / self.scaler, delta.y / self.scaler);
        for comp in self.physics.iter_mut() {
            comp.translate(delta);
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let (x, y) = self.graphics.get_pos();
        (x * self.scaler, y * self.scaler)
    }

    pub fn get_vpos(&self) -> Vector2<f32> {
        let (x, y) = self.graphics.get_pos();
        Vector2::new(x * self.scaler, y * self.scaler)
    }

    pub fn get_gfx_pos(&self) -> (f32, f32) {
        self.graphics.get_pos()
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        let (new_x, new_y) = (pos.0 / self.scaler, pos.1 / self.scaler);
        let (old_x, old_y) = self.get_gfx_pos();
        let (delta_x, delta_y) = ((new_x - old_x) * self.scaler, (new_y - old_y) * self.scaler);
        let delta = Vector2::new(delta_x, delta_y);
        self.translate(delta);
    }

    pub fn set_pos_gfx(&mut self, pos: (f32, f32)) {
        let converted_pos = (pos.0 * self.scaler, pos.1 * self.scaler);
        self.set_pos(converted_pos);
    }

    pub fn update_gfx(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let delta = self.velocity * dt;
        self.translate_gfx(delta);
    }

    pub fn in_screen(&self) -> bool {
        let pos = self.get_gfx_pos();
        return !(pos.0 > 1.0 || pos.0 < -1.0 || pos.1 > 1.0 || pos.1 < -1.0)
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let delta = self.velocity * dt;
        self.translate(delta);
    }
}

pub struct PhysicsComp {
    id: usize,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pos: Isometry2<f32>,
    world: Rc<PhysicsWorld>,
    shape: ShapeHandle2<f32>,
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
        let id = scene.physics.add(position.clone(),
                                           shape.clone(),
                                           interactivity,
                                           query,
                                           Rc::new(PhysicsData::new(entity_id, tag)));
        PhysicsComp {
            id: id,
            pos: Isometry2::new(position, Vector1::new(0.0)),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            world: scene.physics.clone(),
            shape: shape,
        }
    }

    pub fn scaler(&self) -> f32 {
        self.world.scaler
    }

    pub fn translate(&mut self, delta: Vector2<f32>) {
        let pos = self.pos.append_translation(&delta);
        self.set_pos(pos);
    }

    pub fn get_pos(&self) -> Isometry2<f32> {
        self.pos
    }

    pub fn set_vpos(&mut self, pos: Vector2<f32>) {
        self.set_pos(Isometry2::new(pos, Vector1::new(0.0)));
    }

    pub fn sync_pos(&self) {
        self.world.set_pos(self.id, self.pos);
    }

    pub fn set_pos(&mut self, pos: Isometry2<f32>) {
        self.pos = pos;
        let vpos = pos.translation;
        let ppos = unsafe { PLAYER_POSITION };
        let bv = self.get_bounding_box(vpos, ppos, self.world.scaler);
        // bv: AABB<Point2<f32>>
        self.world.set_pos_bv(self.id, pos, bv);
    }

    fn get_bounding_box(&self, cpos: Vector2<f32>, ppos: Vector2<f32>, scaler: f32) -> AABB<Point2<f32>> {
        self.get_bounding_box_rec(cpos, ppos, scaler, Vector2::new(0.0, 0.0), 0, 2)
    }
    
    /// Recursively calculates quadrants for bounding boxes
    fn get_bounding_box_rec(&self, cpos: Vector2<f32>, ppos: Vector2<f32>, scaler: f32, mut pos_mod: Vector2<f32>,depth: u8, max_depth: u8) -> AABB<Point2<f32>> {
        let which_quad = |pos: Vector2<f32>| {
            if pos.x > 0.0 && pos.y > 0.0 { 1 }
            else if pos.x < 0.0 && pos.y > 0.0 { 2 }
            else if pos.x < 0.0 && pos.y < 0.0 { 3 }
            else { 4 }
        };
        let cquad = which_quad(cpos);
        let pquad = which_quad(ppos);
    
        if cquad != pquad {
            match cquad {
                1 => AABB::new(
                    Point2::new(0.0f32, 0.0) + pos_mod,
                    Point2::new(scaler, scaler) + pos_mod
                ),
                2 => AABB::new(
                    Point2::new(-1.0 * scaler, 0.0) + pos_mod,
                    Point2::new(0.0, scaler) + pos_mod
                ),
                3 => AABB::new(
                    Point2::new( -1.0 * scaler, -1.0 * scaler) + pos_mod,
                    Point2::new(0.0f32, 0.0) + pos_mod
                ),
                4 => AABB::new(
                    Point2::new(0.0, -1.0 * scaler) + pos_mod,
                    Point2::new(self.world.scaler, 0.0) + pos_mod
                ),
                _ => unreachable!(),
            }
        } else {
            pos_mod += match cquad {
                1 => Vector2::new(-0.5 * self.world.scaler, -0.5 * self.world.scaler),
                2 => Vector2::new(0.5 * self.world.scaler, -0.5 * self.world.scaler),
                3 => Vector2::new(0.5 * self.world.scaler, 0.5 * self.world.scaler),
                4 => Vector2::new(-0.5 * self.world.scaler, 0.5 * self.world.scaler),
                _ => unreachable!(),
            };
            if depth < max_depth {
                self.get_bounding_box_rec(cpos + pos_mod, ppos + pos_mod, scaler/2.0, pos_mod, depth + 1, max_depth)
            } else {
                let mut aabb = bounding_volume::aabb(self.shape.as_ref(), &Isometry2::new(cpos, Vector1::new(0.0)));
                aabb.loosen(0.5);
                aabb
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let delta = self.velocity * dt;
        self.translate(delta);
    }
}

impl Drop for PhysicsComp {
    fn drop(&mut self) {
        self.world.remove(self.id);
    }
}
