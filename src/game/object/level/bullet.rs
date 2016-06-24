// [bullets.basic_straight]
// sprite = 10
// radius = 5
// behavior = "straight"

#[derive(Clone, Copy)]
pub struct Bullet {
    pub sprite: usize,
    pub behavior: Behavior,
    pub damage: usize,
}

#[derive(Clone, Copy)]
pub enum Behavior {
    Straight
}
