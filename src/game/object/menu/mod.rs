use std::fs::{self, DirEntry};
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
use engine::util;
use game::asset::level::Events;

struct State {
    selected_level: i32,
    asset_files: Vec<DirEntry>,
    level_start_time: i32,
    level_time_max: i32,
    level: Result<Events, String>,
}

impl State {
    fn new() -> State {
        let files = fs::read_dir("assets/levels").unwrap().map(|f| f.unwrap()).collect::<Vec<_>>();
        State {
            selected_level: -1,
            level_start_time: 0,
            level_time_max: 100,
            asset_files: files,
            level: Ok(util::hashmap()),
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
        let w = WorldCompBuilder::new(engine)
            .with_alias(String::from("main_menu"))
            .with_tag(String::from("menu"))
            .build();
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
            let (ui, ev) = {
                let ui = renderer.frame();
                let mut ev = None;
                ui.window(im_str!("Nishikaku"))
                    .size((600.0, 600.0), ImGuiSetCond_FirstUseEver)
                    .title_bar(false)
                    .resizable(false)
                    .movable(false)
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
                            renderer.state.level = match level::load_level_file(graphics, &(String::from("assets/levels/") + &file)) {
                                Ok((_, _, _, events)) => {
                                    renderer.state.level_time_max = get_max_level_time(&events);
                                    Ok(events)  
                                },
                                Err(s) => Err(s),
                            };
                        }
                        ev = if renderer.state.level.is_ok() && renderer.state.selected_level != -1 {
                            ui.slider_int(im_str!("Start time"), &mut renderer.state.level_start_time, 0, renderer.state.level_time_max).build();
                            if ui.small_button(im_str!("Start")) {
                                // Actually load level
                                let e = Event::Custom(
                                    Box::new(
                                        CEvent::LevelStart(
                                            renderer.state.level.clone().unwrap(),
                                            renderer.state.level_start_time as i32
                                        )
                                    )
                                );
                                Some(e)
                            } else { None }
                        } else if renderer.state.level.is_err() {
                            ui.text_colored((1.0, 0.0, 0.0, 1.0), im_str!("Invalid level file: {:?}!", renderer.state.level));
                            None
                        } else {
                            ui.text(im_str!("Select a level file!"));
                            None
                        };
                        ui.separator();
                    });
                (ui, ev)
            };
            renderer.render(ui);
            if let Some(e) = ev {
                let cid = self.world.find_aliased_entity_id(&String::from("controller")).unwrap();
                self.ev.dispatch_to(cid, e);
            }
        }
    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}

fn get_max_level_time(level: &Events) -> i32 {
    get_level_time_rec(level, &String::from("start"), 0)
}

fn get_level_time_rec(level: &Events, cevent: &String, mut time: i32) -> i32 {
    if let Some(evs) = level.get(cevent) {
       time = evs.iter().map(|ev| {
           get_level_time_rec(level, &ev.name, time + (ev.delay as i32))
       }).max().unwrap();
    }
    time
}
