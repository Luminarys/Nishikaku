use std::rc::Rc;
use std::cell::RefCell;

use Engine;
use entity::Entity;
use event::{Event, Handler, SysEvent};

struct Timer {
    id: usize,
    repeat: bool,
    amount: f32,
    left: f32,
    class: u8,
    event: Rc<Event>
}

impl Timer {
    pub fn with_class(id: usize, amount: f32, repeat: bool, event: Event, class: u8) -> Timer {
        Timer {
            id: id,
            repeat: repeat,
            left: amount,
            amount: amount,
            class: class,
            event: Rc::new(event),
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
                self.handler.borrow_mut().enqueue_specific_rc(self.id, timer.event.clone());
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
        self.set_timer_manual(id, amount, false, Event::Timer(id), 0);
    }

    pub fn set_timer_with_class(&mut self, id: usize, amount: f32, class: u8) {
        self.set_timer_manual(id, amount, false, Event::CTimer(class, id), class);
    }

    pub fn set_repeating_timer(&mut self, id: usize, amount: f32) {
        self.set_timer_manual(id, amount, true, Event::Timer(id), 0);
    }

    pub fn set_repeating_timer_with_class(&mut self, id: usize, amount: f32, class: u8) {
        self.set_timer_manual(id, amount, true, Event::CTimer(class, id), class);
    }

    pub fn set_timer_manual(&mut self, id: usize, amount: f32, repeat: bool, event: Event, class: u8) {
        self.timers.push(Timer::with_class(id, amount, repeat, event, class));
    }

    pub fn remove_timer(&mut self, id: usize) {
        match self.timers.iter().position(|timer| timer.id == id) {
            Some(pos) => {
                self.timers.remove(pos);
            }
            None => {}
        }
    }

    pub fn remove_timer_with_class(&mut self, id: usize, class: u8) {
        match self.timers.iter().position(|timer| timer.id == id && timer.class == class) {
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

    pub fn fast_forward(&self, t: f32) {
        self.handler.borrow_mut().enqueue_sys(SysEvent::FastForward(t));
    }

    pub fn destroy_other(&self, id: usize) {
        self.handler.borrow_mut().enqueue_sys(SysEvent::Destroy(id));
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
