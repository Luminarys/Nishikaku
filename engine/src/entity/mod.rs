pub mod component;

use std::rc::Rc;
use Engine;
use event::Event;

pub trait Entity {
    fn handle_event(&mut self, e: Rc<Event>);
    fn id(&self) -> usize;
}

pub trait EntityBuilder<E: Entity> {
    fn build(self, engine: &Engine<E>) -> E;
}
