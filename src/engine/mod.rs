pub mod event;
pub mod scene;
pub mod entity;
pub mod graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use std::collections::HashMap;
use glium::glutin;
use clock_ticks;

pub struct Engine<'a, E: entity::Entity> {
    pub events: Rc<RefCell<event::Handler<E>>>,
    pub scene: scene::Scene<E>,
    pub graphics: graphics::Graphics<'a>,
}

impl<'a, E: entity::Entity> Engine<'a, E> {
    pub fn new() -> Engine<'a, E> {
        Engine {
            events: Rc::new(RefCell::new(event::Handler::new())),
            scene: scene::Scene::new(),
            graphics: graphics::Graphics::new(),
        }
    }

    pub fn spawn(&self, spawner: Box<Fn(&Engine<E>) -> E>) {
        let mut e = spawner(&self);
        let id = e.id();
        self.events.deref().borrow_mut().subscribe(id.clone(), event::Event::Update(0.0));
        e.handle_event(event::Event::Spawn);
        self.scene.world.deref().insert(id, e);
    }

    pub fn destroy(&self, id: usize) {
        self.scene.world.deref().remove(&id);
    }

    pub fn run(&mut self) {
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut accumulator = 0;
        let mut entity_rendering: HashMap<usize, Vec<_>> = HashMap::new();

        // Lol
        let glut_mouse_ev_to_local = |e| {
            match e {
                glutin::MouseButton::Left => event::MouseButton::Left,
                glutin::MouseButton::Right => event::MouseButton::Right,
                glutin::MouseButton::Middle => event::MouseButton::Middle,
                glutin::MouseButton::Other(c) => event::MouseButton::Other(c),
            }
        };

        loop {
            for (_, entity) in self.scene.world.deref().entities.borrow().deref() {
                match entity.borrow().render() {
                    Some(info) => {
                        if !entity_rendering.contains_key(&info.sprite) {
                            entity_rendering.insert(info.sprite, vec![info.attrs]);
                        } else {
                            entity_rendering.get_mut(&info.sprite).unwrap().push(info.attrs);
                        };
                    }
                    None => { }
                }
            }

            for (id, v) in entity_rendering.iter_mut() {
                self.graphics.set_sprite_attrs(id, (&v[..]));
            }

            self.graphics.render();
            for (_, v) in entity_rendering.iter_mut() {
                v.clear();
            }

            let now = clock_ticks::precise_time_ns();
            accumulator += now - previous_clock;
            previous_clock = now;
            const FRAME_DELAY_NANOSECS: u64 = 16666667;

            for event in self.graphics.get_window_events() {
                match event {
                    glutin::Event::Closed => return,
                    glutin::Event::KeyboardInput(glutin::ElementState::Pressed, c, _) => {
                        self.events
                            .deref()
                            .borrow_mut()
                            .enqueue_all(event::Event::KeyInput(event::InputState::Pressed, c))
                    }
                    glutin::Event::KeyboardInput(glutin::ElementState::Released, c, _) => {
                        self.events
                            .deref()
                            .borrow_mut()
                            .enqueue_all(event::Event::KeyInput(event::InputState::Released, c))
                    }
                    glutin::Event::MouseMoved(x, y) => {
                        self.events
                            .deref()
                            .borrow_mut()
                            .enqueue_all(event::Event::MouseMove((x, y)))
                    }
                    glutin::Event::MouseInput(glutin::ElementState::Pressed, b) => {
                        self.events
                            .deref()
                            .borrow_mut()
                            .enqueue_all(event::Event::MouseInput(event::InputState::Pressed,
                                                                  glut_mouse_ev_to_local(b)))
                    }
                    glutin::Event::MouseInput(glutin::ElementState::Released, b) => {
                        self.events
                            .deref()
                            .borrow_mut()
                            .enqueue_all(event::Event::MouseInput(event::InputState::Released,
                                                                  glut_mouse_ev_to_local(b)))
                    }
                    _ => {}
                }
            }

            while accumulator >= FRAME_DELAY_NANOSECS {
                // Update state here
                self.events.deref().borrow_mut().enqueue_all(event::Event::Update(0.16666667f32));
                for (id, event) in self.events.deref().borrow_mut().flush() {
                    self.scene.dispatch(id, event);
                }
                accumulator -= FRAME_DELAY_NANOSECS;
            }
            for event in self.events.deref().borrow_mut().flush_sys() {
                match event {
                    event::SysEvent::Create(f) => self.spawn(f),
                    event::SysEvent::Destroy(id) => self.destroy(id),
                }
            }
            thread::sleep(Duration::from_millis(((FRAME_DELAY_NANOSECS - accumulator) / 1000000) as u64));
        }
    }
}
