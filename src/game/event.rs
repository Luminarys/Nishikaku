pub enum Event {
    MouseClickedOver,
    MouseUnclickedOver,
    MouseOver,
    MouseLeft,
    Level(usize),
    Despawn(usize),
    Action(usize),
    Pattern(usize),
    StartGame,
}
