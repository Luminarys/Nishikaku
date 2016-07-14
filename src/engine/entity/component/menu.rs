use std::rc::Rc;
use std::cell::RefCell;
use imgui::{ImGui, Ui};
use imgui::glium_renderer::Renderer;
use clock_ticks;

use engine::graphics::{Graphics};

pub struct MenuComp {
    pub imgui: ImGui,
    renderer: Renderer,
    graphics: Rc<RefCell<Graphics>>,
    prev_time: u64,
}

pub struct MenuRenderer<'a> {
    pub frame: Ui<'a>,
    renderer: &'a mut Renderer,
    graphics: Rc<RefCell<Graphics>>,
}

impl <'a> MenuRenderer<'a> {
    pub fn render(mut self) {
        self.graphics.borrow_mut().render_menu(&mut self.renderer, self.frame);
    }
}

impl<'comp> MenuComp {
    pub fn new(graphics: Rc<RefCell<Graphics>>) -> MenuComp {
        let mut imgui = ImGui::init();
        let renderer = graphics.borrow().make_menu_renderer(&mut imgui);
        MenuComp {
            imgui: imgui,
            renderer: renderer,
            graphics: graphics,
            prev_time: clock_ticks::precise_time_ns(),
        }
    }

    pub fn get_renderer(&'comp mut self) -> MenuRenderer<'comp> {
        let cur_time = clock_ticks::precise_time_ns();
        let delta_t = ((cur_time - self.prev_time) as f32/10.0e-9) as f32;
        self.prev_time = cur_time;
        let ui = self.graphics.borrow().get_menu_frame(&mut self.imgui, delta_t);

        MenuRenderer {
            frame: ui,
            renderer: &mut self.renderer,
            graphics: self.graphics.clone(),
        }
    }

    pub fn render<F: FnMut(&Ui)>(&'comp mut self, mut run_ui: F) {
        let cur_time = clock_ticks::precise_time_ns();
        let delta_t = ((cur_time - self.prev_time) as f32/10.0e-9) as f32;
        self.prev_time = cur_time;
        let ui = {
            self.graphics.borrow().get_menu_frame(&mut self.imgui, delta_t)
        };
        run_ui(&ui);
        self.graphics.borrow_mut().render_menu(&mut self.renderer, ui);
    }
}
