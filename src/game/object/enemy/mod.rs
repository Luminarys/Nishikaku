use ncollide::shape::Shape2;

pub struct Enemy {
    sprite: usize,
    health: u32,
}

pub struct Sprite {
    gfx_id: usize,
    hitbox: Shape2<f32>,
}
