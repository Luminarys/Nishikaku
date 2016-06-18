use nalgebra::{Norm, Vector2, Point2};

//paths = [
//  { type = "curve", points = [[-200, 180], [-100, 140], [-200, 100]], speed = 40, action = {
//    type = "bullets",
//    id = "basic_straight",
//    time_start = 1.0,
//  }}
//  ]

// paths = [
//   { type = "arc",  center = [-50, 50], radius = 25, start = "current", degrees: 180, speed = 40 },
//   { type = "curve", points = ["current", [75, 0], [200, 0]], speed = 60, action = {
//     type = "bullets",
//     id = "basic_straight",
//     time_start = 1.0,
//     repeat = 2,
//     repeat_delay = 2.0,
//   }}
// ]

pub enum Path {
    Arc(Arc),
    Curve(Curve),
}

impl Path {
    pub fn travel(&mut self, dt: f32) -> Option<Vector2<f32>> {
        match self {
            &mut Path::Arc(ref mut a) => a.travel(dt),
            &mut Path::Curve(ref mut c) => c.travel(dt),
            }
        }

    pub fn finished(&self) -> bool {
        match self {
            &Path::Arc(ref a) => a.degrees <= 0.0,
            &Path::Curve(ref c) => c.points.len() == 1,
        }
    }
}

pub enum RotationDirection {
    Clockwise,
    CounterClockwise
}

pub struct Arc {
    center: Vector2<f32>,
    current_pos: Vector2<f32>,
    radius: f32,
    degrees: f32,
    speed: f32,
    direction: RotationDirection,
}

impl Arc {
    fn travel(&mut self, dt: f32) -> Option<Vector2<f32>> {
        use std::f32::consts::PI;

        if self.degrees > 0.0 {
            let dist = self.speed * dt;
            let circ = 2.0 * self.radius * PI;
            let ang = 360.0 * dist/circ;
            // Handle the case where the angle greatly surpasses degrees left?
            let mut c_ang = self.current_pos.y.atan2(self.current_pos.x);
            self.degrees -= ang;
            match self.direction {
                RotationDirection::Clockwise => {
                    c_ang -= ang
                }
                RotationDirection::CounterClockwise => {
                    c_ang += ang
                }
            };
            self.current_pos = Vector2::new(self.radius * c_ang.to_radians().cos(), self.radius * c_ang.to_radians().sin());
            Some(self.current_pos)
        } else {
            None
        }
    }
}

pub struct Curve {
    points: Vec<Point2<f32>>,
    current_pos: Vector2<f32>,
    node_dist_left: f32,
    speed: f32,
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
                let time_left = dist/self.speed;
                self.travel(time_left)
            } else if dist > self.node_dist_left && self.points.len() == 2 {
                // Return the final point if we finish up travelling to it
                // Do we care that we may have exceeded the final point?
                self.current_pos = self.points.remove(0).to_vector();
                Some(self.current_pos)
            } else {
                // Reduce node dist left and extend the current pos vector proportional to distnace travelled across the vector between the prev point and next point
                self.node_dist_left -= dist;
                let dp = self.points[1] - self.points[0];
                let dt = dp * dist/dp.norm();
                self.current_pos += dt;
                Some(self.current_pos)
            }
        } else {
            None
        }
    }
}

pub enum Point {
    Fixed(Vector2<f32>),
    Player(Vector2<f32>),
    Current(Vector2<f32>),
}

impl Point {
    pub fn eval(&self, current: &Vector2<f32>, player: &Vector2<f32>) -> Vector2<f32> {
        match self {
            &Point::Fixed(ref p) => *p,
            &Point::Player(ref p) => *p + *player,
            &Point::Current(ref p) => *p + *current,
        }
    }
}

pub struct PathBuilder {
    speed: Option<f32>,
    // Arc info
    center: Option<Point>,
    radius: Option<f32>,
    degrees: Option<f32>,
    direction: Option<RotationDirection>,
    // Curve info
    points: Option<Vec<Point>>,
}

impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder {
            speed: None,
            center: None,
            radius: None,
            degrees: None,
            direction: None,
            points: None
        }
    }

    pub fn speed(mut self, speed: f32) -> PathBuilder {
        self.speed = Some(speed);
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

    pub fn build_arc(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Arc {
        Arc {
            center: self.center.unwrap().eval(current_pos, player_pos),
            current_pos: *current_pos,
            radius: self.radius.unwrap(),
            degrees: self.degrees.unwrap(),
            speed: self.speed.unwrap(),
            direction: self.direction.unwrap(),
        }
    }

    pub fn points(mut self, points: Vec<Point>) -> PathBuilder {
        self.points = Some(points);
        self
    }

    pub fn build_curve(self, current_pos: &Vector2<f32>, player_pos: &Vector2<f32>) -> Curve {
        use ncollide::procedural::bezier_curve;

        let points: Vec<_> = self.points.unwrap().iter().map(|point| {
            point.eval(current_pos, player_pos).to_point()
        }).collect();
        let (points, _) = bezier_curve(&points[..], 100).unwrap();

        Curve {
            points: points,
            current_pos: *current_pos,
            node_dist_left: 0.0,
            speed: self.speed.unwrap(),
        }
    }
}
