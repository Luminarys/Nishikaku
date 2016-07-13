use nalgebra::{Norm, Vector2, Point2};
use game::object::level::Point;
use game::object::level::action::{Action, ActionType};
use engine::util::ToCartesian;

// TODO: Write tests - this code is complicated and almost certaintly error prone

pub enum Path {
    Arc(Arc),
    Curve(Curve),
    Fixed(Fixed),
}

#[derive(Copy, Clone, Debug)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl RotationDirection {
    fn opposite(&self) -> RotationDirection {
        match *self {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
        }
    }
}

pub struct Fixed {
    time: f32,
    pos: Vector2<f32>,
    actions: Vec<Action>,
}

pub struct Arc {
    center: Vector2<f32>,
    current_pos: Vector2<f32>,
    radius: f32,
    degrees: f32,
    speed: f32,
    direction: RotationDirection,
    actions: Vec<Action>,
}

pub struct Curve {
    points: Vec<Point2<f32>>,
    current_pos: Vector2<f32>,
    node_dist_left: f32,
    speed: f32,
    actions: Vec<Action>,
}

impl Path {
    pub fn travel(&mut self, dt: f32) -> Option<Vector2<f32>> {
        match *self {
            Path::Arc(ref mut a) => a.travel(dt),
            Path::Curve(ref mut c) => c.travel(dt),
            Path::Fixed(ref mut f) => {
                if f.time >= 0.0 {
                    f.time -= dt;
                    Some(f.pos.clone())
                } else {
                    None
                }
            },
        }
    }

    pub fn finished(&self) -> bool {
        match *self {
            Path::Arc(ref a) => a.degrees <= 0.0,
            Path::Curve(ref c) => c.points.len() == 1,
            Path::Fixed(ref f) => f.time <= 0.0,
        }
    }

    pub fn actions(&mut self) -> Vec<Action> {
        use std::mem;
        match *self {
            Path::Arc(ref mut a) => mem::replace(&mut a.actions, Vec::new()),
            Path::Curve(ref mut c) => mem::replace(&mut c.actions, Vec::new()),
            Path::Fixed(ref mut f) => mem::replace(&mut f.actions, Vec::new()),
        }
    }
}

impl Arc {
    fn travel(&mut self, dt: f32) -> Option<Vector2<f32>> {
        use std::f32::consts::PI;
        use nalgebra::angle_between;

        if self.degrees > 0.0 {
            let dist = self.speed * dt;
            let circ = 2.0 * self.radius * PI;
            let ang = 360.0 * dist / circ;
            // Handle the case where the angle greatly surpasses degrees left?
            let mut c_ang = angle_between(&Vector2::new(1.0, 0.0),
                                          &(self.current_pos - self.center))
                                .to_degrees();
            self.degrees -= ang;
            match self.direction {
                RotationDirection::Clockwise => c_ang -= ang,
                RotationDirection::CounterClockwise => c_ang += ang,
            };
            let dp = Vector2::new(self.radius, c_ang.to_radians()).to_cartesian();
            self.current_pos = dp + self.center;
            Some(self.current_pos)
        } else {
            None
        }
    }
}

impl Curve {
    fn travel(&mut self, dt: f32) -> Option<Vector2<f32>> {
        if self.points.len() > 2 {
            let mut dist = self.speed * dt;
            if dist > self.node_dist_left && self.points.len() > 2 {
                // Advance to the next point and then recall the method with reduced dt so that hopefully case 3 is used.
                dist -= self.node_dist_left;
                self.points.remove(0);

                let dp = self.points[1] - self.points[0];
                self.node_dist_left = dp.norm();
                self.current_pos = *self.points[0].as_vector();
                let time_left = dist / self.speed;
                self.travel(time_left)
            } else if dist > self.node_dist_left && self.points.len() == 2 {
                // Return the final point if we finish up travelling to it
                // Do we care that we may have exceeded the final point?
                // Theoretically dt is always small enough that it'll never matter
                self.current_pos = self.points.remove(0).to_vector();
                Some(self.current_pos)
            } else {
                // Reduce node dist left and extend the current pos vector proportional to distance travelled across the vector between the prev point and next point
                self.node_dist_left -= dist;
                let dp = self.points[1] - self.points[0];
                let dt = dp * dist / dp.norm();
                self.current_pos += dt;
                Some(self.current_pos)
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PathType {
    Arc,
    Curve,
    Fixed,
}

#[derive(Clone, Debug)]
pub struct PathBuilder {
    pub path_type: PathType,
    speed: Option<f32>,
    actions: Vec<Action>,
    // Arc info
    center: Option<Point>,
    radius: Option<f32>,
    degrees: Option<f32>,
    direction: Option<RotationDirection>,
    // Curve info
    points: Option<Vec<Point>>,
    // Fixed info
    time: Option<f32>,
}

impl PathBuilder {
    pub fn new(pt: PathType) -> PathBuilder {
        PathBuilder {
            speed: None,
            path_type: pt,
            center: None,
            radius: None,
            degrees: None,
            direction: None,
            points: None,
            time: None,
            actions: vec![],
        }
    }

    pub fn speed(mut self, speed: f32) -> PathBuilder {
        self.speed = Some(speed);
        self
    }

    pub fn actions(mut self, mut actions: Vec<Action>) -> PathBuilder {
        self.actions.append(&mut actions);
        self
    }

    pub fn center(mut self, center: Point) -> PathBuilder {
        self.center = Some(center);
        self
    }

    pub fn radius(mut self, radius: f32) -> PathBuilder {
        self.radius = Some(radius);
        self
    }

    pub fn degrees(mut self, degrees: f32) -> PathBuilder {
        self.degrees = Some(degrees);
        self
    }

    pub fn direction(mut self, direction: RotationDirection) -> PathBuilder {
        self.direction = Some(direction);
        self
    }

    pub fn time(mut self, time: f32) -> PathBuilder {
        self.time = Some(time);
        self
    }

    pub fn points(mut self, points: Vec<Point>) -> PathBuilder {
        self.points = Some(points);
        self
    }

    pub fn mirror_x(&self) -> PathBuilder {
        let mut path = self.clone();

        let mut actions = Vec::new();
        for action in self.actions.iter() {
            match action.action_type {
                ActionType::Bullets(ref bullet, ref pat) => {
                    actions.push(Action {
                        delay: action.delay,
                        action_type: ActionType::Bullets(bullet.clone(), pat.mirror_x())
                    });
                }
                ActionType::None => actions.push(action.clone())
            }
        }
        path.actions = actions;
        match self.path_type {
            PathType::Arc => {
                path.center = Some(path.center.unwrap().mirror_x());
                path.direction = Some(path.direction.unwrap().opposite());
            }
            PathType::Curve => {
                let mut points = Vec::new();
                for point in path.points.unwrap().iter() {
                    points.push(point.mirror_x());
                }
                path.points = Some(points);
            }
            PathType::Fixed => { }
        }
        path
    }

    pub fn mirror_y(&self) -> PathBuilder {
        let mut path = self.clone();

        let mut actions = Vec::new();
        for action in self.actions.iter() {
            match action.action_type {
                ActionType::Bullets(ref bullet, ref pat) => {
                    actions.push(Action {
                        delay: action.delay,
                        action_type: ActionType::Bullets(bullet.clone(), pat.mirror_y())
                    });
                }
                ActionType::None => actions.push(action.clone())
            }
        }
        path.actions = actions;
        match self.path_type {
            PathType::Arc => {
                path.center = Some(path.center.unwrap().mirror_y());
                path.direction = Some(path.direction.unwrap().opposite());
            }
            PathType::Curve => {
                let mut points = Vec::new();
                for point in path.points.unwrap().iter() {
                    points.push(point.mirror_y());
                }
                path.points = Some(points);
            }
            PathType::Fixed => { }
        }
        path
    }

    pub fn build(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Path {
        match self.path_type {
            PathType::Arc => self.build_arc(current_pos, player_pos),
            PathType::Curve => self.build_curve(current_pos, player_pos),
            PathType::Fixed => self.build_fixed(current_pos),
        }
    }

    fn build_fixed(self, current_pos: &Vector2<f32>) -> Path {
        Path::Fixed(Fixed {time: self.time.unwrap(), pos: current_pos.clone(), actions: self.actions})
    }

    fn build_arc(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Path {
        let center = self.center.unwrap().eval(current_pos, player_pos);
        Path::Arc(Arc {
            center: center,
            current_pos: *current_pos,
            radius: self.radius.unwrap(),
            degrees: self.degrees.unwrap(),
            speed: self.speed.unwrap(),
            direction: self.direction.unwrap(),
            actions: self.actions,
        })
    }

    fn build_curve(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Path {
        // TODO: Use a B-Spline
        use ncollide_procedural::bezier_curve;

        let points: Vec<_> = self.points
                                 .unwrap()
                                 .iter()
                                 .map(|point| point.eval(current_pos, player_pos).to_point())
                                 .collect();
        let (points, _) = bezier_curve(&points[..], 100).unwrap();

        Path::Curve(Curve {
            points: points,
            current_pos: *current_pos,
            node_dist_left: 0.0,
            speed: self.speed.unwrap(),
            actions: self.actions,
        })
    }
}
