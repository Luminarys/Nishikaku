// [bullets.basic_straight]
// sprite = 10
// radius = 5
// behavior = "straight"

#[derive(Clone, Copy, Debug)]
pub struct Bullet {
    pub sprite: usize,
    pub behavior: Behavior,
    pub damage: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Behavior {
    Straight,
    Deaccelerate(f32, f32),
}
