pub enum Event {
    MouseClickedOver,
    MouseUnclickedOver,
    MouseOver,
    MouseLeft,
    Level(usize),
    Action(usize),
    Pattern(usize),
}
