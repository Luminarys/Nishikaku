use nalgebra::{angle_between, Vector2};
use game::object::enemy::PosFetcher;
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
}

#[derive(Clone, Debug)]
struct PatternState {
    cur_angle: f32,
    amount_left: usize,
    int_time: f32,
}

impl Pattern {
    fn update(&mut self, dt: f32) {
        self.rep_time += dt;
        if self.rep_time >= self.repeat_delay && self.repeat > 0 {
            self.repeat -= 1;
            self.rep_time= 0.0;
            if let Some(ref fetcher) = self.pos_fetcher {
                let pos_info = fetcher.fetch();
                self.start_angle = self.actual_start.eval(&pos_info.0, &pos_info.1);
                self.stop_angle = self.actual_stop.eval(&pos_info.0, &pos_info.1);
            }
            self.active_patterns.push(PatternState { cur_angle: self.start_angle, amount_left: self.amount, int_time: 0.0 })
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
                let angle = pattern.cur_angle;
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
    speed: f32,
    radius: f32,
    repeat: usize,
    repeat_delay: f32,
}

impl PatternBuilder {
    pub fn new() -> PatternBuilder {
        PatternBuilder {
            cur_angle: None,
            stop_angle: None,
            amount: 1,
            time_int: 0.0,
            speed: 0.0,
            radius: 0.0,
            repeat: 0,
            repeat_delay: 0.0,
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

    pub fn build(self, cur_pos: &Vector2<f32>, player: &Vector2<f32>) -> Pattern {
        let sa = self.cur_angle.unwrap().eval(cur_pos, player);
        Pattern {
            start_angle: sa,
            actual_start: self.cur_angle.unwrap(),
            stop_angle: self.stop_angle.unwrap().eval(cur_pos, player),
            actual_stop: self.stop_angle.unwrap(),
            amount: self.amount,
            time_int: self.time_int,
            speed: self.speed,
            radius: self.radius,
            repeat: self.repeat,
            repeat_delay: self.repeat_delay,
            active_patterns: vec![PatternState { amount_left: self.amount, cur_angle: sa, int_time: 0.0 }],
            rep_time: 0.0,
            pos_fetcher: None,
        }
    }
}
