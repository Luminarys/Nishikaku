pub mod event;
pub mod scene;
pub mod entity;
pub mod graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use glium::glutin;
use clock_ticks;

pub struct Engine<'a, E: entity::Entity> {
    events: Rc<RefCell<event::Handler<E>>>,
    scene: scene::Scene<E>,
    graphics: graphics::Graphics<'a>,
}

impl <'a, E: entity::Entity>Engine<'a, E> {
    fn new() -> Engine<'a, E> {
        Engine {
            events: Rc::new(RefCell::new(event::Handler::new())),
            scene: scene::Scene::new(),
            graphics: graphics::Graphics::new(),
        }
    }

    fn run(&mut self) {
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut accumulator = 0;
        loop {
            self.graphics.render();
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
            thread::sleep(Duration::from_millis(((FRAME_DELAY_NANOSECS - accumulator) / 1000000) as u64));
        }
    }
}
