use game::object::level::pattern::PatternBuilder;
use game::object::level::bullet::Bullet;

#[derive(Copy, Clone)]
pub struct Action {
    pub delay: f32,
    pub action_type: ActionType,
}

#[derive(Copy, Clone)]
pub enum ActionType {
    Bullets(Bullet, PatternBuilder),
    None,
}
