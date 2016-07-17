use game::asset::level::Events;

pub enum Event {
    LevelStart(Events, i32),
}
