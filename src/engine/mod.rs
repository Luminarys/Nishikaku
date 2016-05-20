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
    events: Rc<RefCell<event::Handler<E>>>,
    scene: scene::Scene<E>,
    graphics: graphics::Graphics<'a>,
}

impl <'a, E: entity::Entity>Engine<'a, E> {
    pub fn new() -> Engine<'a, E> {
        Engine {
            events: Rc::new(RefCell::new(event::Handler::new())),
            scene: scene::Scene::new(),
            graphics: graphics::Graphics::new(),
        }
    }

    pub fn run(&mut self) {
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut accumulator = 0;
        let mut entity_rendering: HashMap<usize, Vec<_>> = HashMap::new();

        loop {
            for (_, entity) in self.scene.world.deref().entities.borrow().deref() {
                let info = entity.borrow().render();
                if !entity_rendering.contains_key(&info.sprite) {
                    entity_rendering.insert(info.sprite, vec![info.attrs]);
                } else {
                    entity_rendering.get_mut(&info.sprite).unwrap().push(info.attrs);
                
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
                    glutin::Event::KeyboardInput(e, c, vkc) => self.events.deref().borrow_mut().enqueue_all(event::Event::KeyInput(e, c, vkc)),
                    glutin::Event::MouseMoved(x, y) => self.events.deref().borrow_mut().enqueue_all(event::Event::MouseMove((x, y))),
                    glutin::Event::MouseInput(e, b) => self.events.deref().borrow_mut().enqueue_all(event::Event::MouseInput(e, b)),
                    _ => { }
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
                // Create/Destroy shit
            }
            thread::sleep(Duration::from_millis(((FRAME_DELAY_NANOSECS - accumulator) / 1000000) as u64));
        }
    }
}
