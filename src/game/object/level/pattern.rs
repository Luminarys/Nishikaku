use nalgebra::{angle_between, Vector2};
// TODO: Write tests - this code is complicated and almost certaintly error prone

#[derive(Copy, Clone)]
pub struct Pattern {
    pub time_int: f32,
    amount: usize,
    cur_angle: f32,
    stop_angle: f32,
    speed: f32,
    radius: f32,
}

impl Pattern {
    /// Consumes a portion of the arc and emits translation and velocity vectors of the object
    pub fn next(&mut self) -> Option<(Vector2<f32>, Vector2<f32>)> {
        if self.amount > 0 {
            let angle = self.cur_angle;
            self.cur_angle += (self.stop_angle - self.cur_angle) / self.amount as f32;
            self.amount -= 1;
            let base = Vector2::new(angle.to_radians().cos(), angle.to_radians().sin());
            Some((base * self.radius, base * self.speed))
        } else {
            None
        }
    }

    pub fn finished(&self) -> bool {
        self.amount == 0
    }
}

pub enum Angle {
    Fixed(f32),
    Player(f32),
}

impl Angle {
    fn eval(&self, cur_pos: &Vector2<f32>, player: &Vector2<f32>) -> f32 {
        match self {
            &Angle::Fixed(ref angle) => *angle,
            &Angle::Player(ref angle_mod) => {
                let mut ab = angle_between(&Vector2::new(1.0, 0.0), &(*player - *cur_pos))
                                 .to_degrees();
                if ab < 0.0 {
                    ab += 360.0;
                }
                ab + angle_mod
            }
        }
    }
}

pub struct PatternBuilder {
    time_int: f32,
    amount: usize,
    cur_angle: Angle,
    stop_angle: Angle,
    speed: f32,
    radius: f32,
}

impl PatternBuilder {
    pub fn new(start_angle: Angle,
               stop_angle: Angle,
               radius: f32,
               time_int: f32,
               amount: usize,
               speed: f32)
               -> PatternBuilder {
        PatternBuilder {
            cur_angle: start_angle,
            stop_angle: stop_angle,
            amount: amount,
            time_int: time_int,
            speed: speed,
            radius: radius,
        }
    }

    pub fn build(self, cur_pos: &Vector2<f32>, player: &Vector2<f32>) -> Pattern {
        Pattern {
            cur_angle: self.cur_angle.eval(cur_pos, player),
            stop_angle: self.stop_angle.eval(cur_pos, player),
            amount: self.amount,
            time_int: self.time_int,
            speed: self.speed,
            radius: self.radius,
        }
    }
}
