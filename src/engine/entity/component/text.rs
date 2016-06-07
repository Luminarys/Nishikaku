use std::cell::RefCell;
use std::rc::Rc;

use engine::Engine;
use engine::entity::Entity;
use engine::graphics::Graphics;

pub struct TextComp {
    font: usize,
    data: FontAttrs,
    scaler: f32,
    text: String,
    gfx: Rc<RefCell<Graphics>>
}

impl TextComp {
    pub fn render(&mut self) {
        self.gfx.borrow_mut().render_text(&self.font, &self.text[..], &self.data.transform, &self.data.color);
    }
}

pub struct TextCompBuilder {
    gfx: Rc<RefCell<Graphics>>,
    font: usize,
    scaler: f32,
    text: String,
    attrs: FontAttrs,
}

impl TextCompBuilder {
    pub fn new<E: Entity>(engine: &Engine<E>) -> TextCompBuilder {
        TextCompBuilder {
            gfx: engine.graphics.clone(),
            font: 1,
            scaler: 1.0,
            attrs: Default::default(),
            text: String::new(),
        }
    }

    pub fn new_scaled<E: Entity>(engine: &Engine<E>) -> TextCompBuilder {
        let scaler = engine.scene.physics.scaler;
        let mut builder = TextCompBuilder::new(engine);
        builder.scaler = scaler;
        builder
    }

    pub fn with_text(mut self, text: &str) -> TextCompBuilder {
        self.text = String::from(text);
        self
    }

    pub fn with_font(mut self, id: usize) -> TextCompBuilder {
        self.font = id;
        self
    }

    pub fn with_pos(mut self, pos: (f32, f32)) -> TextCompBuilder {
        self.attrs.transform[3][0] = pos.0/ self.scaler;
        self.attrs.transform[3][1] = pos.1/self.scaler;
        self
    }

    pub fn with_color(mut self, color: (f32, f32, f32, f32)) -> TextCompBuilder {
        self.attrs.color = color;
        self
    }

    pub fn build(self) -> TextComp {
        TextComp {
            font: self.font,
            data: self.attrs,
            scaler: self.scaler,
            text: self.text,
            gfx: self.gfx
        }
    }
}

pub struct FontAttrs {
    transform: [[f32; 4]; 4],
    color: (f32, f32, f32, f32)
}

impl Default for FontAttrs {
    fn default() -> FontAttrs {
        let scaler = 0.07;
        FontAttrs {
            transform: [
                        [scaler, 0.0, 0.0, 0.0],
                        [0.0, scaler, 0.0, 0.0],
                        [0.0, 0.0, scaler, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ],
            color: (1.0, 1.0, 1.0, 1.0)
        }
    }
}
