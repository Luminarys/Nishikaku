use game::object::level::pattern::PatternBuilder;

pub struct Action {
    pub time_start: f32,
    pub action_type: ActionType,
}

pub enum ActionType {
    Bullets(PatternBuilder)
}
