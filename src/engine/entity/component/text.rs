use std::cell::RefCell;
use std::rc::Rc;

use engine::Engine;
use engine::entity::Entity;
use engine::graphics::Graphics;

pub struct TextComp {
    font: usize,
    data: FontAttrs,
    scaler: f32,
    gfx: Rc<RefCell<Graphics>>
}

pub struct TextCompBuilder {
    gfx: Rc<RefCell<Graphics>>,
    font: usize,
    scaler: f32,
    attrs: FontAttrs,
}

impl TextCompBuilder {
    fn new<E: Entity>(engine: Engine<E>) -> TextCompBuilder {
        TextCompBuilder {
            gfx: engine.graphics.clone(),
            font: 1,
            scaler: 1.0,
            attrs: Default::default()
        }
    }

    fn new_scaled<E: Entity>(engine: Engine<E>) -> TextCompBuilder {
        TextCompBuilder {
            gfx: engine.graphics.clone(),
            font: 1,
            scaler: engine.scene.physics.scaler,
            attrs: Default::default()
        }
    }

    fn with_font(mut self, id: usize) -> TextCompBuilder {
        self.font = id;
        self
    }

    fn with_pos(mut self, pos: (f32, f32)) -> TextCompBuilder {
        self.attrs.transform[0][3] = pos.0 * self.scaler;
        self.attrs.transform[1][3] = pos.1 * self.scaler;
        self
    }

    fn with_color(mut self, color: (f32, f32, f32, f32)) -> TextCompBuilder {
        self.attrs.color = color;
        self
    }

    fn build(self) -> TextComp {
        TextComp {
            font: self.font,
            data: self.attrs,
            scaler: self.scaler,
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
        FontAttrs {
            transform: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ],
            color: (1.0, 1.0, 1.0, 1.0)
        }
    }
}

impl TextComp {
}
