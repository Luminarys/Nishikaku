pub mod event;
pub mod scene;
pub mod entity;
pub mod graphics;
pub mod audio;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::ops::Deref;
use std::mem;
use glium::glutin;
use ncollide::shape::{Plane, Cuboid, ShapeHandle2};
use ncollide::query::Proximity;
use nalgebra::Vector2;
use ncollide::world::GeometricQueryType;
use clock_ticks;

use self::entity::component::PhysicsData;
use self::scene::PhysicsInteraction;

pub struct Engine<E: entity::Entity> {
    pub events: Rc<RefCell<event::Handler<E>>>,
    pub scene: scene::Scene<E>,
    pub graphics: Rc<RefCell<graphics::Graphics>>,
    pub audio: Rc<RefCell<audio::Audio>>,
}

impl<E: entity::Entity> Engine<E> {
    pub fn new() -> Engine<E> {
        // Default physics dimensions are 400x400 - a square which has a half length 200 units away
        // from the origin
        let scene = scene::Scene::new(200.0);

        let plane_left = ShapeHandle2::new(Plane::new(Vector2::x()));
        let plane_bottom = ShapeHandle2::new(Plane::new(Vector2::y()));
        let plane_right = ShapeHandle2::new(Plane::new(-Vector2::x()));
        let plane_top = ShapeHandle2::new(Plane::new(-Vector2::y()));

        let plane_data = Rc::new(PhysicsData::new(0, String::from("view_border")));

        scene.physics.add(Vector2::new(-200.0, 0.0),
                          plane_left,
                          PhysicsInteraction::Interactive,
                          GeometricQueryType::Contacts(0.2),
                          plane_data.clone());
        scene.physics.add(Vector2::new(0.0, -200.0),
                          plane_bottom,
                          PhysicsInteraction::Interactive,
                          GeometricQueryType::Contacts(0.2),
                          plane_data.clone());
        scene.physics.add(Vector2::new(200.0, 0.0),
                          plane_right,
                          PhysicsInteraction::Interactive,
                          GeometricQueryType::Contacts(0.2),
                          plane_data.clone());
        scene.physics.add(Vector2::new(0.0, 200.0),
                          plane_top,
                          PhysicsInteraction::Interactive,
                          GeometricQueryType::Contacts(0.2),
                          plane_data.clone());

        let rect = ShapeHandle2::new(Cuboid::new(Vector2::new(200.0, 200.0)));
        let rect_data = Rc::new(PhysicsData::new(0, String::from("view_area")));
        scene.physics.add(Vector2::new(0.0, 0.0),
                          rect,
                          PhysicsInteraction::Interactive,
                          GeometricQueryType::Proximity(0.2),
                          rect_data);

        let eh = event::Handler::new();
        let queue = eh.queue.clone();
        let events = Rc::new(RefCell::new(eh));
        scene.physics.register_handlers(queue);

        Engine {
            events: events,
            scene: scene,
            graphics: Rc::new(RefCell::new(graphics::Graphics::new())),
            audio: Rc::new(RefCell::new(audio::Audio::new())),
        }
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

        // Lol
        let glut_mouse_ev_to_local = |e| {
            match e {
                glutin::MouseButton::Left => event::MouseButton::Left,
                glutin::MouseButton::Right => event::MouseButton::Right,
                glutin::MouseButton::Middle => event::MouseButton::Middle,
                glutin::MouseButton::Other(c) => event::MouseButton::Other(c),
            }
        };

        let mut fps_prev_clock = clock_ticks::precise_time_ms();
        let mut frames_drawn = 0;
        let mut key_counter = [0 as u8; 255];
        loop {
            self.events.deref().borrow_mut().enqueue_all(event::Event::Render);
            self.handle_events();
            self.graphics.borrow_mut().render();

            let now = clock_ticks::precise_time_ns();
            let fps_cur_clock = clock_ticks::precise_time_ms();
            frames_drawn += 1;
            if fps_cur_clock - fps_prev_clock >= 1000 {
                println!("{:?} ms/frame", 1000.0 / (frames_drawn as f32));
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
                    glutin::Event::MouseMoved(x, y) => Some(event::Event::MouseMove((x, y))),
                    glutin::Event::MouseInput(glutin::ElementState::Pressed, b) => {
                        Some(event::Event::MouseInput(event::InputState::Pressed,
                                                      glut_mouse_ev_to_local(b)))
                    }
                    glutin::Event::MouseInput(glutin::ElementState::Released, b) => {
                        Some(event::Event::MouseInput(event::InputState::Released,
                                                      glut_mouse_ev_to_local(b)))
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
                self.scene.update();
                accumulator -= FRAME_DELAY_NANOSECS;
            }

            let sys_ev_queue = {
                self.events.deref().borrow_mut().flush_sys()
            };
            for event in sys_ev_queue {
                match event {
                    event::SysEvent::Create(f) => self.spawn(f),
                    event::SysEvent::Destroy(id) => self.destroy(id),
                }
            }
            thread::sleep(Duration::from_millis(((FRAME_DELAY_NANOSECS - accumulator) / 1000000) as u64));
        }
    }

    fn handle_internal_event(&mut self, event: Rc<event::Event>) {
        match *event {
            event::Event::Proximity(id, ref data) => {
                match &data.this_object.tag[..] {
                    "view_area" => {
                        match data.proximity {
                            Proximity::Intersecting => {
                                self.events
                                    .deref()
                                    .borrow_mut()
                                    .enqueue_specific(id, event::Event::Entering);
                            }
                            Proximity::Disjoint => {
                                self.events
                                    .deref()
                                    .borrow_mut()
                                    .enqueue_specific(id, event::Event::Exiting);
                            }
                            _ => {}
                        }
                    }
                    "view_border" => {
                        let mut new_data = data.clone();
                        mem::swap(&mut new_data.this_object, &mut new_data.other_object);
                        self.events
                            .deref()
                            .borrow_mut()
                            .enqueue_specific(id, event::Event::Proximity(id, new_data));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
