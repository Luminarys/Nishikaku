use nalgebra::{angle_between, Vector2};
// TODO: Write tests - this code is complicated and almost certaintly error prone

#[derive(Copy, Clone, Debug)]
pub struct Pattern {
    pub time_int: f32,
    pub center: Vector2<f32>,
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct PatternBuilder {
    time_int: f32,
    amount: usize,
    cur_angle: Option<Angle>,
    stop_angle: Option<Angle>,
    center: Option<Vector2<f32>>,
    speed: f32,
    radius: f32,
}

impl PatternBuilder {
    pub fn new() -> PatternBuilder {
        PatternBuilder {
            cur_angle: None,
            stop_angle: None,
            center: None,
            amount: 1,
            time_int: 0.0,
            speed: 0.0,
            radius: 0.0,
        }
    }

    pub fn start_angle(mut self, angle: Angle) -> PatternBuilder {
        self.cur_angle = Some(angle);
        self
    }

    pub fn stop_angle(mut self, angle: Angle) -> PatternBuilder {
        self.stop_angle = Some(angle);
        self
    }

    pub fn fixed_angle(mut self, angle: Angle) -> PatternBuilder {
        self.start_angle(angle).stop_angle(angle)
    }

    pub fn center(mut self, center: Vector2<f32>) -> PatternBuilder {
        self.center = Some(center);
        self
    }

    pub fn radius(mut self, radius: f32) -> PatternBuilder {
        self.radius = radius;
        self
    }

    pub fn time_int(mut self, time_int: f32) -> PatternBuilder {
        self.time_int = time_int;
        self
    }

    pub fn amount(mut self, amount: usize) -> PatternBuilder {
        self.amount = amount;
        self
    }

    pub fn speed(mut self, speed: f32) -> PatternBuilder {
        self.speed = speed;
        self
    }

    pub fn build(self, cur_pos: &Vector2<f32>, player: &Vector2<f32>) -> Pattern {
        Pattern {
            center: if self.center.is_none() { *cur_pos } else { self.center.unwrap() },
            cur_angle: self.cur_angle.unwrap().eval(cur_pos, player),
            stop_angle: self.stop_angle.unwrap().eval(cur_pos, player),
            amount: self.amount,
            time_int: self.time_int,
            speed: self.speed,
            radius: self.radius,
        }
    }
}
