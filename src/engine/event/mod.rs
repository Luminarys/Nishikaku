use std::collections::HashMap;
use std::collections::HashSet;

pub enum Event {
    Redraw,
    Collision,
}

pub struct Handler {
    subscriptions: HashMap<Event, HashSet<usize>>
}
