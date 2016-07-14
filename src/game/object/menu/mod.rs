use std::fs::{self, DirEntry};
use nalgebra::Vector2;
use ncollide_geometry::shape::{Cuboid, ShapeHandle2};
use imgui::*;
use glium::glutin::{MouseButton, MouseScrollDelta, TouchPhase};
use std::rc::Rc;
use std::cell::RefCell;
use engine::graphics::Graphics;

use game::object::Object;
use game::event::Event as CEvent;
use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};

struct State {
    selected_level: i32,
    asset_files: Vec<DirEntry>,
    level_start_time: i32,
    level_time_max: i32,
    valid_level: bool,
}

impl State {
    fn new() -> State {
        let files = fs::read_dir("assets/").unwrap().map(|f| f.unwrap()).collect::<Vec<_>>();
        State {
            selected_level: -1,
            level_start_time: 0,
            level_time_max: 100,
            asset_files: files,
            valid_level: true,
        }
    }
}

pub struct MainMenu {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    graphics: Rc<RefCell<Graphics>>,
    menu: MenuComp,
    state: State,
}

impl MainMenu {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).with_tag(String::from("main_menu")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        let m = MenuComp::new(engine);
        Object::MainMenu(MainMenu {
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
                self.render_menu();
            }
            _ => {}
        };
    }

    fn render_menu(&mut self) {
        {
            let mut renderer = self.menu.get_renderer(&mut self.state);
            let graphics = self.graphics.clone();
            // ty ck
            let ui = {
                let mut ui = renderer.frame();
                ui.window(im_str!("Nishikaku"))
                    .title_bar(false)
                    .resizable(false)
                    .movable(false)
                    .size((600.0, 600.0), ImGuiSetCond_FirstUseEver)
                    .build(|| {
                        let file_names: Vec<_> = renderer.state.asset_files.iter().map(|f| {
                            ImStr::from(f.file_name().into_string().unwrap())
                        }).collect();

                        ui.text(im_str!("Nishikaku Testing"));
                        ui.separator();
                        ui.text(im_str!("Load a level"));
                        if ui.list_box(im_str!("Level file"), &mut renderer.state.selected_level, &file_names[..], 5) {
                            use game::asset::level;
                            let file = renderer.state.asset_files[renderer.state.selected_level as usize].file_name().into_string().unwrap();
                            match level::load_level_file(graphics, &(String::from("assets/") + &file)) {
                                Ok(_) => renderer.state.valid_level = true,
                                Err(_) => renderer.state.valid_level = false,
                            }
                        }
                        if renderer.state.valid_level && renderer.state.selected_level != -1 {
                            ui.slider_int(im_str!("Start time"), &mut renderer.state.level_start_time, 0, renderer.state.level_time_max).build();
                            if ui.small_button(im_str!("Start")) {
                                // Actually load level
                            }
                        } else if !renderer.state.valid_level {
                            ui.text_colored((1.0, 0.0, 0.0, 1.0), im_str!("Invalid level file!"));
                        } else {
                            ui.text(im_str!("Select a level file!"));
                        }
                        ui.separator();
                    });
                ui
            };
            renderer.render(ui);
        }
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
