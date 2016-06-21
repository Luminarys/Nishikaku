use game::object::level::pattern::PatternBuilder;
use game::object::level::bullet::Bullet;

pub struct Action {
    pub delay: f32,
    pub action_type: ActionType,
}

pub enum ActionType {
    Bullets(Bullet, PatternBuilder),
    None,
}
