use std::cell::RefCell;
use std::rc::Rc;

use Engine;
use entity::Entity;
use graphics::Graphics;

pub struct TextComp {
    font: usize,
    pub data: FontAttrs,
    pub text: String,
    gfx: Rc<RefCell<Graphics>>
}

impl TextComp {
    pub fn render(&mut self) {
        self.gfx.borrow_mut().render_text(&self.font, &self.text, &self.data.transform, &self.data.color);
    }
}

pub struct TextCompBuilder {
    gfx: Rc<RefCell<Graphics>>,
    font: usize,
    scaler: f32,
    text: String,
    pos: (f32, f32),
    color: (f32, f32, f32, f32),
}

impl TextCompBuilder {
    pub fn new<E: Entity>(engine: &Engine<E>) -> TextCompBuilder {
        TextCompBuilder {
            gfx: engine.graphics.clone(),
            font: 1,
            scaler: 0.07,
            text: String::new(),
            pos: (0.0, 0.0),
            color: (0.0, 0.0, 0.0, 0.0),
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
        self.pos = pos;
        self
    }

    pub fn with_color(mut self, color: (f32, f32, f32, f32)) -> TextCompBuilder {
        self.color = color;
        self
    }

    pub fn build(self) -> TextComp {
        let mut fa = FontAttrs::new(self.scaler);
        fa.transform[3][0] = self.pos.0/self.scaler;
        fa.transform[3][1] = self.pos.1/self.scaler;
        fa.color = self.color;

        TextComp {
            font: self.font,
            data: fa,
            text: self.text,
            gfx: self.gfx
        }
    }
}

pub struct FontAttrs {
    pub transform: [[f32; 4]; 4],
    pub color: (f32, f32, f32, f32)
}

impl FontAttrs {
    fn new(scaler: f32) -> FontAttrs {
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
