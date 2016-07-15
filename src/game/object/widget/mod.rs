use imgui::*;
use glium::glutin::{MouseButton, MouseScrollDelta, TouchPhase};
use std::rc::Rc;
use std::cell::RefCell;
use engine::graphics::Graphics;
use clock_ticks;

use game::object::Object;
use game::object::bullet::BULLET_COUNT;
use game::object::enemy::ENEMY_COUNT;
use engine::event::{Event, InputState};
use engine::Engine;
use engine::entity::component::*;

struct State {
    start_time: u64,
    prev_time: u64,
    frames_drawn: u32,
    total_frames_drawn: u32,
    mspf_history: Vec<f32>,
    mspf: f32,
    avg_mspf: f32,
}

impl State {
    fn new() -> State {
        let now = clock_ticks::precise_time_ms();
        State{
            start_time: now,
            prev_time: now,
            frames_drawn: 0,
            total_frames_drawn: 0,
            mspf: 0.0,
            avg_mspf: 0.0,
            mspf_history: vec![0.0; 16],
        }
    }
}

pub struct LevelStatus {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    graphics: Rc<RefCell<Graphics>>,
    menu: MenuComp,
    state: State,
}

impl LevelStatus {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).build();
        let e = EventComp::new(w.id, engine.events.clone());
        let m = MenuComp::new(engine);
        Object::LevelStatus(LevelStatus {
            ev: e,
            world: w,
            menu: m,
            graphics: engine.graphics.clone(),
            state: State::new(),
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                self.ev.subscribe(Event::RenderMenu);
                self.ev.subscribe(Event::MouseMove((0.0, 0.0)));
                self.ev.subscribe(Event::MouseInput(InputState::Released, MouseButton::Left));
                self.ev.subscribe(Event::MouseScroll(MouseScrollDelta::LineDelta(0.0, 0.0),
                TouchPhase::Moved));
            }
            Event::MouseMove(pos) => {
                self.menu.set_mouse_pos(pos);
            }
            Event::MouseInput(ref state, ref button) => {
                self.menu.set_mouse_button(state, button);
            }
            Event::MouseScroll(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) => {
                self.menu.set_mouse_scroll(y);
            }
            Event::MouseScroll(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => {
                self.menu.set_mouse_scroll(y);
            }
            Event::RenderMenu => {
                self.render_ui();
            }
            _ => { }
        }
    }

    fn render_ui(&mut self) {
        let now = clock_ticks::precise_time_ms();
        let mut renderer = self.menu.get_renderer(&mut self.state);
        let graphics = self.graphics.clone();

        renderer.state.frames_drawn += 1;
        renderer.state.total_frames_drawn += 1;
        let time_diff = now - renderer.state.prev_time;
        if time_diff >= 1000 {
            renderer.state.mspf = time_diff as f32/(renderer.state.frames_drawn as f32);
            renderer.state.frames_drawn = 0;
            renderer.state.prev_time = now;
            renderer.state.avg_mspf = (now - renderer.state.start_time) as f32/(renderer.state.total_frames_drawn as f32);
            renderer.state.mspf_history.push(renderer.state.mspf);
        }

        let ui = {
            let ui = renderer.frame();
            ui.window(im_str!("Level Status"))
                .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
                .bg_alpha(0.7)
                .title_bar(false)
                .resizable(true)
                .movable(true)
                .collapsible(true)
                .build(|| {
                    ui.text(im_str!("ms/frame: {:?}", renderer.state.mspf));
                    ui.text(im_str!("avg. ms/frame: {:?}", renderer.state.avg_mspf));
                    let len = renderer.state.mspf_history.len();
                    ui.plot_lines(im_str!(""), &renderer.state.mspf_history[(len-10)..len])
                        .scale_min(15.0)
                        .scale_max(25.0)
                        .build();
                    ui.text(im_str!("enemies: {:?}", unsafe { ENEMY_COUNT }));
                    ui.text(im_str!("bullets: {:?}", unsafe { BULLET_COUNT }));
                });
            ui
        };
        renderer.render(ui);
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
