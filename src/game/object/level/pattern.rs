use nalgebra::{angle_between, Vector2};
use game::object::enemy::PosFetcher;
use game::object::level::path::RotationDirection;
use engine::util::ToCartesian;
// TODO: Write tests - this code is complicated and almost certaintly error prone

#[derive(Clone, Debug)]
pub struct Pattern {
    pub time_int: f32,
    pos_fetcher: Option<PosFetcher>,
    repeat: usize,
    repeat_delay: f32,
    amount: usize,
    actual_start: Angle,
    actual_stop: Angle,
    start_angle: f32,
    stop_angle: f32,
    speed: f32,
    radius: f32,
    rep_time: f32,
    active_patterns: Vec<PatternState>,
    // The amplitude of the wobble
    wobble_angle: f32,
    // The period of the wobble
    wobble_time: f32,
    cur_wobble_time: f32,
    wobble_dir: RotationDirection,
}

#[derive(Clone, Debug)]
struct PatternState {
    cur_angle: f32,
    amount_left: usize,
    int_time: f32,
    wobble_angle: f32,
}

impl Pattern {
    fn update(&mut self, dt: f32) {
        self.cur_wobble_time += dt;
        self.rep_time += dt;
        if self.rep_time >= self.repeat_delay && self.repeat > 0 {
            self.repeat -= 1;
            self.rep_time= 0.0;
            if let Some(ref fetcher) = self.pos_fetcher {
                let pos_info = fetcher.fetch();
                self.start_angle = self.actual_start.eval(&pos_info.0, &pos_info.1);
                self.stop_angle = self.actual_stop.eval(&pos_info.0, &pos_info.1);
            }

            use std::f32::consts;

            self.active_patterns.push(PatternState {
                cur_angle: self.start_angle,
                amount_left: self.amount,
                int_time: 0.0,
                wobble_angle: self.wobble_angle * (consts::PI * 2.0 * self.cur_wobble_time/self.wobble_time).sin()
            })
        }
        for pattern in self.active_patterns.iter_mut() {
            pattern.int_time += dt;
        }
    }

    pub fn set_pos_fetcher(&mut self, fetcher: PosFetcher) {
        self.pos_fetcher = Some(fetcher);
    }

    /// Consumes a portion of the arc and emits translation and velocity vectors of the object
    pub fn next(&mut self, dt: f32) -> Vec<(Vector2<f32>, Vector2<f32>)> {
        self.update(dt);
        let mut res = Vec::new();
        for pattern in self.active_patterns.iter_mut() {
            while pattern.int_time >= self.time_int && pattern.amount_left > 0 {
                pattern.int_time -= self.time_int;
                pattern.amount_left -= 1;
                let wobble_angle = match self.wobble_dir {
                    RotationDirection::CounterClockwise => pattern.wobble_angle,
                    RotationDirection::Clockwise => pattern.wobble_angle * -1.0,
                };
                let angle = pattern.cur_angle + wobble_angle;
                pattern.cur_angle += (self.stop_angle - self.start_angle) / self.amount as f32;
                let base = Vector2::new(1.0, angle.to_radians()).to_cartesian();
                res.push((base * self.radius, base * self.speed))
            }
        }
        res
    }

    pub fn finished(&self) -> bool {
        self.repeat == 0 && self.active_patterns.iter().all(|p| {
            p.amount_left == 0
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Angle {
    Fixed(f32),
    Player(f32),
}

impl Angle {
    fn mirror_x(&self) -> Angle {
        match *self {
            Angle::Fixed(ref angle) => Angle::Fixed(180.0 - angle),
            Angle::Player(ref angle_mod) => Angle::Player(-1.0 * angle_mod),
        }
    }

    fn mirror_y(&self) -> Angle {
        match *self {
            Angle::Fixed(ref angle) => Angle::Fixed(180.0 + angle),
            Angle::Player(ref angle_mod) => Angle::Fixed(-1.0 * angle_mod),
        }
    }

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
    start_angle: Option<Angle>,
    stop_angle: Option<Angle>,
    speed: f32,
    radius: f32,
    repeat: usize,
    repeat_delay: f32,
    wobble_angle: f32,
    wobble_time: f32,
    wobble_dir: RotationDirection,
}

impl PatternBuilder {
    pub fn new() -> PatternBuilder {
        PatternBuilder {
            start_angle: None,
            stop_angle: None,
            amount: 1,
            time_int: 0.0,
            speed: 0.0,
            radius: 0.0,
            repeat: 0,
            repeat_delay: 0.0,
            wobble_angle: 0.0,
            wobble_time: 1.0,
            wobble_dir: RotationDirection::CounterClockwise,
        }
    }

    pub fn start_angle(mut self, angle: Angle) -> PatternBuilder {
        self.start_angle = Some(angle);
        self
    }

    pub fn stop_angle(mut self, angle: Angle) -> PatternBuilder {
        self.stop_angle = Some(angle);
        self
    }

    pub fn fixed_angle(self, angle: Angle) -> PatternBuilder {
        self.start_angle(angle).stop_angle(angle)
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

    pub fn repeat(mut self, repeat: usize) -> PatternBuilder {
        self.repeat = repeat;
        self
    }

    pub fn repeat_delay(mut self, delay: f32) -> PatternBuilder {
        self.repeat_delay = delay;
        self
    }

    pub fn wobble(mut self, angle: f32, delay: f32, direction: RotationDirection) -> PatternBuilder {
        self.wobble_angle = angle;
        self.wobble_time = delay;
        self.wobble_dir = direction;
        self
    }

    pub fn mirror_x(&self) -> PatternBuilder {
        let mut pattern = self.clone();
        pattern.start_angle = Some(pattern.start_angle.unwrap().mirror_x());
        pattern.stop_angle = Some(pattern.stop_angle.unwrap().mirror_x());
        pattern
    }

    pub fn mirror_y(&self) -> PatternBuilder {
        let mut pattern = self.clone();
        let stop_angle = Some(pattern.start_angle.unwrap().mirror_y());
        let start_angle = Some(pattern.stop_angle.unwrap().mirror_y());
        pattern.stop_angle = stop_angle;
        pattern.start_angle = start_angle;
        pattern
    }

    pub fn build(self, cur_pos: &Vector2<f32>, player: &Vector2<f32>) -> Pattern {
        let sa = self.start_angle.unwrap().eval(cur_pos, player);
        Pattern {
            start_angle: sa,
            actual_start: self.start_angle.unwrap(),
            stop_angle: self.stop_angle.unwrap().eval(cur_pos, player),
            actual_stop: self.stop_angle.unwrap(),
            amount: self.amount,
            time_int: self.time_int,
            speed: self.speed,
            radius: self.radius,
            repeat: self.repeat,
            repeat_delay: self.repeat_delay,
            active_patterns: vec![PatternState { amount_left: self.amount, cur_angle: sa, int_time: 0.0, wobble_angle: 0.0 }],
            rep_time: 0.0,
            pos_fetcher: None,
            cur_wobble_time: 0.0,
            wobble_angle: self.wobble_angle,
            wobble_time: self.wobble_time,
            wobble_dir: self.wobble_dir,
        }
    }
}
