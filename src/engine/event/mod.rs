use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cmp::PartialEq;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use ncollide::query::{Contact, Proximity};
use nalgebra::Point2;
use glium::glutin::VirtualKeyCode;
use glium::glutin::MouseButton;

use engine::Engine;
use engine::entity::Entity;
use engine::entity::component::PhysicsData;

#[derive(Clone)]
pub enum InputState {
    Pressed,
    Released,
}

pub enum Event {
    Update(f32),
    Collision(usize, CollisionData),
    Proximity(usize, ProximityData),
    KeyInput(InputState, KeyCode),
    MouseMove((f32, f32)),
    MouseInput(InputState, MouseButton),
    Spawn,
    Timer(usize),
    Render,
    Custom(Box<Any>),
}

pub type KeyCode = VirtualKeyCode;

#[derive(Clone)]
pub struct CollisionData {
    pub contact: Contact<Point2<f32>>,
    pub this_object: Rc<PhysicsData>,
    pub other_object: Rc<PhysicsData>,
}

#[derive(Clone)]
pub struct ProximityData {
    pub proximity: Proximity,
    pub this_object: Rc<PhysicsData>,
    pub other_object: Rc<PhysicsData>,
}

pub enum SysEvent<E: Entity> {
    Destroy(usize),
    Create(Box<Fn(&Engine<E>) -> E>),
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Event::Update(_) => state.write_u8(0),
            Event::Collision(_, _) => state.write_u8(1),
            Event::KeyInput(_, _) => state.write_u8(2),
            Event::MouseMove(_) => state.write_u8(3),
            Event::MouseInput(_, _) => state.write_u8(4),
            Event::Spawn => state.write_u8(5),
            Event::Proximity(_, _) => state.write_u8(6),
            Event::Timer(_) => state.write_u8(7),
            Event::Render => state.write_u8(8),
            Event::Custom(_) => state.write_u8(9),
        }
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        match (self, other) {
            (&Event::Update(_), &Event::Update(_)) => true,
            (&Event::Collision(_, _), &Event::Collision(_, _)) => true,
            (&Event::Proximity(_, _), &Event::Proximity(_, _)) => true,
            (&Event::KeyInput(_, _), &Event::KeyInput(_, _)) => true,
            (&Event::MouseMove(_), &Event::MouseMove(_)) => true,
            (&Event::MouseInput(_, _), &Event::MouseInput(_, _)) => true,
            (&Event::Spawn, &Event::Spawn) => true,
            (&Event::Timer(_), &Event::Timer(_)) => true,
            (&Event::Render, &Event::Render) => true,
            (&Event::Custom(_), &Event::Custom(_)) => true,
            _ => false,
        }
    }
}

pub struct Dispatcher {
    pub queue: Rc<RefCell<Vec<(usize, Rc<Event>)>>>,
}

impl Dispatcher {
    pub fn dispatch(&self, id: usize, e: Event) {
        self.queue.borrow_mut().push((id, Rc::new(e)));
    }
}

#[derive(Default)]
pub struct Handler<E: Entity> {
    subscriptions: HashMap<Event, HashSet<usize>>,
    pub queue: Rc<RefCell<Vec<(usize, Rc<Event>)>>>,
    sysqueue: Vec<SysEvent<E>>,
}

impl<E: Entity> Handler<E> {
    pub fn new() -> Handler<E> {
        Handler {
            sysqueue: vec![],
            queue: Rc::new(RefCell::new(vec![])),
            subscriptions: Default::default(),
        }
    }

    pub fn subscribe(&mut self, id: usize, event: Event) {
        if !self.subscriptions.contains_key(&event) {
            let mut set: HashSet<usize> = Default::default();
            set.insert(id);
            self.subscriptions.insert(event, set);
        } else {
            self.subscriptions.get_mut(&event).unwrap().insert(id);
        }
    }

    pub fn unsubscribe(&mut self, id: usize, event: Event) {
        match self.subscriptions.get_mut(&event) {
            Some(subscribers) => {
                subscribers.remove(&id);
            }
            None => {}
        }
    }

    pub fn unsubscribe_all(&mut self, id: usize) {
        for (_, subs) in self.subscriptions.iter_mut() {
            subs.remove(&id);
        }
    }

    pub fn enqueue_all(&mut self, event: Event) {
        match self.subscriptions.get(&event) {
            Some(subscribers) => {
                let rc = Rc::new(event);
                for sub in subscribers {
                    self.queue.borrow_mut().push((sub.clone(), rc.clone()));
                }
            }
            None => {}
        }
    }

    pub fn enqueue_specific(&mut self, id: usize, event: Event) {
        self.queue.borrow_mut().push((id, Rc::new(event)));
    }

    pub fn enqueue_specific_rc(&mut self, id: usize, event: Rc<Event>) {
        self.queue.borrow_mut().push((id, event));
    }

    pub fn enqueue_sys(&mut self, event: SysEvent<E>) {
        self.sysqueue.push(event);
    }

    pub fn flush(&mut self) -> Vec<(usize, Rc<Event>)> {
        mem::replace(&mut self.queue.borrow_mut(), Default::default())
    }

    pub fn flush_sys(&mut self) -> Vec<SysEvent<E>> {
        mem::replace(&mut self.sysqueue, Default::default())
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
