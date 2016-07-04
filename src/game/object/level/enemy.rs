// [enemies.basic_cuve]
// sprite = 1
// health = 10
#[derive(Copy, Clone, Debug)]
pub struct Enemy {
    pub sprite: usize,
    pub health: usize,
    pub damage: usize,
}
