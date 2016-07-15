use game::asset::level::Events;

pub enum Event {
    MouseClickedOver,
    MouseUnclickedOver,
    MouseOver,
    MouseLeft,
    Level(usize),
    Despawn(usize),
    Action(usize),
    Pattern(usize),
    LevelStart(Events, i32),
}
