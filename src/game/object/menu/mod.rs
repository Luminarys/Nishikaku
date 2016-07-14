use std::rc::Rc;
use nalgebra::Vector2;
use ncollide_geometry::shape::{Cuboid, ShapeHandle2};
use imgui::*;
use glium::glutin::MouseButton;

use game::object::Object;
use game::event::Event as CEvent;
use engine::Engine;
use engine::entity::component::*;
use engine::event::{Event, InputState};

pub struct MainMenu {
    ev: EventComp<Object>,
    world: WorldComp<Object>,
    menu: MenuComp,
    mouse_pos: (f32, f32),
}

impl MainMenu {
    pub fn new(engine: &Engine<Object>) -> Object {
        let w = WorldCompBuilder::new(engine).with_tag(String::from("main_menu")).build();
        let e = EventComp::new(w.id, engine.events.clone());
        let m = MenuComp::new(engine.graphics.clone());
        Object::MainMenu(MainMenu {
            ev: e,
            world: w,
            menu: m,
            mouse_pos: (0.0, 0.0),
        })
    }

    pub fn handle_event(&mut self, e: Rc<Event>) {
        match *e {
            Event::Spawn => {
                self.ev.subscribe(Event::RenderMenu);
                self.ev.subscribe(Event::MouseMove((0.0, 0.0)));
                self.ev.subscribe(Event::MouseInput(InputState::Released, MouseButton::Left));
            }
            Event::MouseMove(pos) => {
                self.mouse_pos = pos;
            }
            Event::RenderMenu => {
                self.render_menu();
            }
            _ => {}
        };
    }

    fn render_menu(&mut self) {
        let scale = self.menu.imgui.display_framebuffer_scale();
        self.menu.imgui.set_mouse_pos(self.mouse_pos.0 / scale.0, self.mouse_pos.1 / scale.1);
        assert!(self.menu.imgui.mouse_pos().0 > self.mouse_pos.0 - 10.0);
        assert!(self.menu.imgui.mouse_pos().1 > self.mouse_pos.1 - 10.0);
        {
            let renderer = self.menu.get_renderer();
            // assert!(renderer.frame.imgui().mouse_pos().0 > self.mouse_pos.0 - 10.0);
            // assert!(renderer.frame.imgui().mouse_pos().1 > self.mouse_pos.1 - 10.0);
            let mouse_pos = self.mouse_pos;
            // ty ck
            {
                let ui = &renderer.frame;
                // assert!(ui.imgui().mouse_pos().0 > self.mouse_pos.0 - 10.0);
                // assert!(ui.imgui().mouse_pos().1 > self.mouse_pos.1 - 10.0);
                ui.window(im_str!("Hello world"))
                    .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
                    .build(|| {
                        ui.text(im_str!("Hello world!"));
                        ui.text(im_str!("This...is...imgui-rs!"));
                        ui.separator();
                        // let mouse_pos = ui.imgui().mouse_pos();
                        ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                    });
            }
            // renderer.render();
        }
        assert!(self.menu.imgui.mouse_pos().0 > self.mouse_pos.0 - 10.0);
        assert!(self.menu.imgui.mouse_pos().1 > self.mouse_pos.1 - 10.0);

    }

    pub fn id(&self) -> usize {
        self.world.id
    }
}
