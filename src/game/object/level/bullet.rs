// [bullets.basic_straight]
// sprite = 10
// radius = 5
// behavior = "straight"
pub struct Bullet {
    pub sprite: usize,
    pub behavior: Behavior,
    pub damage: usize,
}

pub enum Behavior {
    Straight
}
