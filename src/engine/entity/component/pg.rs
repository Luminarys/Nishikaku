use std::rc::Rc;
use nalgebra::{Vector2};

use engine::scene::{PhysicsWorld};
use engine::entity::RenderInfo;
use engine::entity::component::{PhysicsComp, GraphicsComp};

/// Combined physics and graphics component,
/// synchronizes their motion
pub struct PGComp {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    physics: Vec<PhysicsComp>,
    graphics: GraphicsComp,
    world: Rc<PhysicsWorld>,
    scaler: f32,
    screen_locked: bool,
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
            screen_locked: false,
            half_widths: (0.0, 0.0),
        }
    }

    pub fn screen_lock(&mut self, half_widths: (f32, f32)) {
        self.screen_locked = true;
        self.half_widths = (half_widths.0 / self.scaler, half_widths.1 / self.scaler);
    }

    pub fn get_render_info(&self) -> RenderInfo {
        self.graphics.get_render_info()
    }

    pub fn render(&self) {
        self.graphics.render();
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

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        let delta = self.velocity * dt;
        self.translate(delta);

        if self.screen_locked {
            let new_pos = self.get_gfx_pos();
            let mut actual_pos = new_pos;
            if new_pos.0 > 1.0 - self.half_widths.0 {
                actual_pos.0 = 1.0 - self.half_widths.0;
            } else if new_pos.0 < -1.0 + self.half_widths.0 {
                actual_pos.0 = -1.0 + self.half_widths.0;
            }

            if new_pos.1 > 1.0 - self.half_widths.1 {
                actual_pos.1 = 1.0 - self.half_widths.1;
            } else if new_pos.1 < -1.0 + self.half_widths.1 {
                actual_pos.1 = -1.0 + self.half_widths.1;
            }
            self.set_pos_gfx(actual_pos);
        }
    }
}
