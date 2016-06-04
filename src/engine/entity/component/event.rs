use std::rc::Rc;
use std::cell::RefCell;

use engine::Engine;
use engine::entity::Entity;
use engine::event::{Event, Handler, SysEvent};

pub struct Timer {
    pub id: usize,
    pub repeat: bool,
    amount: f32,
    left: f32,
}

impl Timer {
    pub fn new(id: usize, amount: f32, repeat: bool) -> Timer {
        Timer {
            id: id,
            repeat: repeat,
            left: amount,
            amount: amount,
        }
    }

    pub fn update(&mut self, time: f32) -> bool {
        self.left -= time;
        if self.repeat && self.left <= 0.0 {
            self.left = self.amount + self.left;
            true
        } else {
            self.left <= 0.0
        }
    }
}

pub struct EventComp<E: Entity> {
    id: usize,
    handler: Rc<RefCell<Handler<E>>>,
    timers: Vec<Timer>,
}

impl<E: Entity> EventComp<E> {
    pub fn new(id: usize, handler: Rc<RefCell<Handler<E>>>) -> EventComp<E> {
        EventComp {
            id: id,
            handler: handler,
            timers: vec![],
        }
    }

    pub fn update(&mut self, time: f32) {
        let mut expired = vec![];
        for (i, timer) in self.timers.iter_mut().enumerate() {
            if timer.update(time) {
                self.handler.borrow_mut().enqueue_specific(self.id, Event::Timer(timer.id));
                if !timer.repeat {
                    expired.push(i);
                }
            }
        }
        for i in expired {
            self.timers.remove(i);
        }
    }

    pub fn set_timer(&mut self, id: usize, amount: f32) {
        self.timers.push(Timer::new(id, amount, false));
    }

    pub fn set_repeating_timer(&mut self, id: usize, amount: f32) {
        self.timers.push(Timer::new(id, amount, true));
    }

    pub fn remove_timer(&mut self, id: usize) {
        match self.timers.iter().position(|timer| timer.id == id) {
            Some(pos) => {
                self.timers.remove(pos);
            }
            None => {}
        }
    }

    pub fn subscribe(&self, event: Event) {
        self.handler.borrow_mut().subscribe(self.id.clone(), event);
    }

    pub fn unsubscribe(&self, event: Event) {
        self.handler.borrow_mut().unsubscribe(self.id.clone(), event);
    }

    pub fn destroy_self(&self) {
        self.handler.borrow_mut().enqueue_sys(SysEvent::Destroy(self.id));
    }

    pub fn dispatch(&self, event: Event) {
        self.handler.borrow_mut().enqueue_all(event);
    }

    pub fn dispatch_to(&self, id: usize, event: Event) {
        self.handler.borrow_mut().enqueue_specific(id, event);
    }

    pub fn create_entity(&self, f: Box<Fn(&Engine<E>) -> E>) {
        self.handler.borrow_mut().enqueue_sys(SysEvent::Create(f));
    }
}

impl<E: Entity> Drop for EventComp<E> {
    fn drop(&mut self) {
        self.handler.borrow_mut().unsubscribe_all(self.id);
    }
}
