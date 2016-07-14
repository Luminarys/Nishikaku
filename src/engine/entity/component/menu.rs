use std::rc::Rc;
use std::cell::RefCell;
use imgui::{ImGui, Ui};
use imgui::glium_renderer::Renderer;
use glium::glutin::MouseButton;
use engine::event::InputState;
use clock_ticks;

use engine::Engine;
use game::object::Object;
use engine::graphics::{Graphics};

pub struct MenuComp {
    pub imgui: ImGui,
    renderer: Renderer,
    graphics: Rc<RefCell<Graphics>>,
    prev_time: u64,
    scaler: f32,
    mouse_pressed: (bool, bool, bool),
    mouse_wheel: f32,
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
    pub fn new(engine: &Engine<Object>) -> MenuComp {
        let mut imgui = ImGui::init();
        let renderer = engine.graphics.borrow().make_menu_renderer(&mut imgui);
        MenuComp {
            imgui: imgui,
            renderer: renderer,
            graphics: engine.graphics.clone(),
            scaler: engine.scene.physics.scaler,
            prev_time: clock_ticks::precise_time_ns(),
            mouse_pressed: (false, false, false),
            mouse_wheel: 0.0,
        }
    }

    pub fn set_mouse_pos(&mut self, (x, y): (f32, f32)) {
        let scale = self.imgui.display_framebuffer_scale();
        let scaler = self.scaler;
        let dims = self.graphics.borrow().dimensions;
        let dim_x = dims.0 as f32;
        let dim_y = dims.1 as f32;
        self.imgui.set_mouse_pos(((x + scaler)/(2.0 * scaler) * dim_x)/(scale.0 as f32), ((-1.0 * y + scaler)/(2.0 * scaler) * dim_y)/(scale.1 as f32));
    }

    pub fn set_mouse_button(&mut self, state: &InputState, button: &MouseButton) {
        let pressed = match *state {
            InputState::Pressed => true,
            _ => false,
        };
        match *button {
            MouseButton::Left => self.mouse_pressed.0 = pressed,
            MouseButton::Right => self.mouse_pressed.1 = pressed,
            MouseButton::Middle => self.mouse_pressed.2 = pressed,
            _ => { }
        }
        self.imgui.set_mouse_down(&[self.mouse_pressed.0, self.mouse_pressed.1, self.mouse_pressed.2, false, false]);
    }

    pub fn set_mouse_scroll(&mut self, amount: f32) {
        let scale = self.imgui.display_framebuffer_scale();
        self.imgui.set_mouse_wheel(amount/scale.1);
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
