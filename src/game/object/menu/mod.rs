use std::rc::Rc;
use nalgebra::Vector2;
use ncollide_geometry::shape::{Cuboid, ShapeHandle2};
use imgui::*;
use glium::glutin::{MouseButton, MouseScrollDelta, TouchPhase};

use game::object::Object;
use game::event::Event as CEvent;
use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};

pub struct MainMenu {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    menu: MenuComp,
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
            let renderer = self.menu.get_renderer();
            // ty ck
            {
                let ui = &renderer.frame;
                ui.window(im_str!("Hello world"))
                  .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
                  .build(|| {
                      ui.text(im_str!("Hello world!"));
                      ui.text(im_str!("This...is...imgui-rs!"));
                      ui.separator();
                      let mouse_pos = ui.imgui().mouse_pos();
                      ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                  });
            }
            renderer.render();
        }

    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
