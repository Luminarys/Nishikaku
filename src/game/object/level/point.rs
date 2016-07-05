use nalgebra::Vector2;

#[derive(Copy, Clone, Debug)]
pub enum Point {
    Fixed(Vector2<f32>),
    Player(Vector2<f32>),
    Current(Vector2<f32>),
}

impl Point {
    pub fn eval(&self, current: &Vector2<f32>, player: &Vector2<f32>) -> Vector2<f32> {
        match self {
            &Point::Fixed(ref p) => *p,
            &Point::Current(ref p) => *p + *current,
            &Point::Player(ref p) => *p + *player,
        }
    }
}

