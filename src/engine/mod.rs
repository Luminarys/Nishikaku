pub mod event;
pub mod scene;
pub mod entity;
pub mod graphics;
pub mod physics;
pub mod audio;
pub mod util;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use glium::glutin;
use clock_ticks;

pub struct Engine<E: entity::Entity> {
    pub events: Rc<RefCell<event::Handler<E>>>,
    pub scene: scene::Scene<E>,
    pub graphics: Rc<RefCell<graphics::Graphics>>,
    pub audio: Rc<RefCell<audio::Audio>>,
}

pub static mut BULLET_COUNT: usize = 0;

impl<E: entity::Entity> Engine<E> {
    pub fn new(size: f32, res: u32, p: scene::PhysicsHandler) -> Engine<E> {
        let scene = scene::Scene::new(size, p);
        let eh = event::Handler::new();
        let events = Rc::new(RefCell::new(eh));

        Engine {
            events: events,
            scene: scene,
            graphics: Rc::new(RefCell::new(graphics::Graphics::new(res, res))),
            audio: Rc::new(RefCell::new(audio::Audio::new())),
        }
    }

    pub fn spawn_entity(&self, mut e: E) {
        let id = e.id();
        self.events.deref().borrow_mut().subscribe(id.clone(), event::Event::Update(0.0));
        self.events.deref().borrow_mut().subscribe(id.clone(), event::Event::Render);
        e.handle_event(Rc::new(event::Event::Spawn));
        self.scene.world.deref().insert(id, e);
    }

    pub fn spawn(&self, spawner: Box<Fn(&Engine<E>) -> E>) {
        let mut e = spawner(&self);
        let id = e.id();
        self.events.deref().borrow_mut().subscribe(id.clone(), event::Event::Update(0.0));
        self.events.deref().borrow_mut().subscribe(id.clone(), event::Event::Render);
        e.handle_event(Rc::new(event::Event::Spawn));
        self.scene.world.deref().insert(id, e);
    }

    pub fn destroy(&self, id: usize) {
        self.scene.world.deref().remove(&id);
    }

    pub fn handle_events(&mut self) {
        let ev_queue = {
            self.events.deref().borrow_mut().flush()
        };
        for (id, event) in ev_queue {
            if id != 0 {
                self.scene.dispatch(id, event);
            } else {
                self.handle_internal_event(event);
            }
        }
    }

    pub fn run(&mut self) {
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut accumulator = 0;

        let mut fps_prev_clock = clock_ticks::precise_time_ms();
        let fps_start_clock = clock_ticks::precise_time_ms();
        let mut frames_drawn = 0;
        let mut total_frames_drawn = 0;
        let mut key_counter = [0 as u8; 255];
        let res_x = self.graphics.borrow().dimensions.0 as f32;
        let res_y = self.graphics.borrow().dimensions.1 as f32;
        let scaler = self.scene.physics.scaler;
        loop {
            // TODO: Figure out a cleaner way to get prioritiztion of custom sprites/text over generic sprites
            // Maybe just don't care and force text to be used with custom sprites?
            let now = clock_ticks::precise_time_ns();
            let fps_cur_clock = clock_ticks::precise_time_ms();
            frames_drawn += 1;
            total_frames_drawn += 1;
            if fps_cur_clock - fps_prev_clock >= 1000 {
                println!("{:?} ms/frame, {:?} total average ms/frame, {:?} bullet count",
                         1000.0/(frames_drawn as f32),
                         1.0/(total_frames_drawn as f32/(fps_cur_clock - fps_start_clock) as f32),
                         unsafe { BULLET_COUNT });
                frames_drawn = 0;
                fps_prev_clock = fps_cur_clock;
            }

            accumulator += now - previous_clock;
            previous_clock = now;
            const FRAME_DELAY_NANOSECS: u64 = 16666667;

            for event in self.graphics.borrow_mut().get_window_events() {
                let to_queue = match event {
                    glutin::Event::Closed => return,
                    glutin::Event::KeyboardInput(glutin::ElementState::Pressed, n, c) => {
                        if key_counter[n as usize] == 0 && !c.is_none() {
                            key_counter[n as usize] = 1;
                            Some(event::Event::KeyInput(event::InputState::Pressed, c.unwrap()))
                        } else {
                            None
                        }
                    }
                    glutin::Event::KeyboardInput(glutin::ElementState::Released, n, c) => {
                        match c {
                            Some(code) => {
                                key_counter[n as usize] = 0;
                                Some(event::Event::KeyInput(event::InputState::Released, code))
                            }
                            None => None,
                        }
                    }
                    glutin::Event::MouseMoved(x, y) => Some(event::Event::MouseMove((x as f32 * (2.0 * scaler/res_x) - scaler, -1.0 * (y as f32 * (2.0 * scaler/res_y) - scaler)))),
                    glutin::Event::MouseInput(glutin::ElementState::Pressed, b) => {
                        Some(event::Event::MouseInput(event::InputState::Pressed, b))
                    }
                    glutin::Event::MouseInput(glutin::ElementState::Released, b) => {
                        Some(event::Event::MouseInput(event::InputState::Released, b))
                    }
                    _ => None,
                };
                if let Some(ev) = to_queue {
                    self.events.borrow_mut().enqueue_all(ev);
                }
            }

            while accumulator >= FRAME_DELAY_NANOSECS {
                // Update state here
                self.events.deref().borrow_mut().enqueue_all(event::Event::Update(0.016666667f32));
                self.handle_events();
                self.scene.update(0.016666667f32);
                let sys_ev_queue = {
                    self.events.deref().borrow_mut().flush_sys()
                };
                for event in sys_ev_queue {
                    match event {
                        event::SysEvent::Create(f) => self.spawn(f),
                        event::SysEvent::Destroy(id) => self.destroy(id),
                    }
                }
                accumulator -= FRAME_DELAY_NANOSECS;
            }

            self.graphics.borrow_mut().start_frame();
            self.graphics.borrow_mut().render();
            self.events.deref().borrow_mut().enqueue_all(event::Event::Render);
            self.handle_events();
            self.graphics.borrow_mut().finish_frame();

            thread::sleep(Duration::from_millis(((FRAME_DELAY_NANOSECS - accumulator) / 1000000) as u64));
        }
    }

    fn handle_internal_event(&mut self, _event: Rc<event::Event>) {
        // Maybe do something here in the future?
    }
}
