use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cmp::PartialEq;
use std::mem;

#[derive(Clone, Eq)]
pub enum Event {
    Update(u64),
    Collision(u64),
    Destroy,
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Event::Update(_) => state.write_u8(0),
            Event::Collision(_) => state.write_u8(1),
            Event::Destroy => state.write_u8(2),
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        match (self, other) {
            (&Event::Update(_), &Event::Update(_)) => true,
            (&Event::Collision(_), &Event::Collision(_)) => true,
            (&Event::Destroy, &Event::Destroy) => true,
            _ => false,
        }
    }
}

#[derive(Default)]
pub struct Handler {
    subscriptions: HashMap<Event, HashSet<usize>>,
    queue: Vec<(usize, Event)>
}

impl Handler {
    pub fn new() -> Handler {
        Default::default()
    }

    pub fn enqueue_all(&mut self, event: Event) {
        match self.subscriptions.get(&event) {
            Some(subscribers) => {
                for sub in subscribers {
                    self.queue.push((sub.clone(), event.clone()));
                };
            },
            None => { },
        }
    }

    pub fn enqueue_specific(&mut self, id: usize, event: Event) {
        self.queue.push((id, event));
    }

    pub fn flush(&mut self) -> Vec<(usize, Event)> {
        mem::replace(&mut self.queue, Default::default())
    }
}

#[test]
fn test_eventmap() {
    let mut evs = HashMap::new();
    evs.insert(Event::Update(0), 0);
    evs.insert(Event::Collision(0), 1);
    assert_eq!(evs.get(&Event::Update(1)).unwrap(), &0);
    assert_eq!(evs.get(&Event::Collision(1)).unwrap(), &1);
}
