use std::collections::HashMap;
use std::collections::HashSet;

pub enum Event {
    Update(u64),
    Render,
}

pub struct Handler {
    subscriptions: HashMap<Event, HashSet<usize>>
}
