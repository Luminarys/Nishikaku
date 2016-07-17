use std::rc::Rc;
use std::cell::RefCell;

use graphics::{Graphics, SpriteAttrs};


pub struct GraphicsComp {
    sprite: usize,
    id: usize,
    data: SpriteAttrs,
    graphics: Rc<RefCell<Graphics>>
}

impl GraphicsComp {
    pub fn new(graphics: Rc<RefCell<Graphics>>, sprite: usize) -> GraphicsComp {
        let id = graphics.borrow_mut().get_id(&sprite).unwrap();
        GraphicsComp {
            sprite: sprite,
            data: Default::default(),
            graphics: graphics,
            id: id,
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.data.translate(dx, dy);
    }

    pub fn set_rot(&mut self, t: f32) {
        self.data.set_rot(t);
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.data.set_pos(x, y);
    }

    pub fn get_pos(&self) -> (f32, f32) {
        self.data.get_pos()
    }

    pub fn get_data(&self) -> &SpriteAttrs {
        &self.data
    }

    pub fn render(&self) {
        self.graphics.borrow_mut().set_sprite_attr(&self.sprite, self.id, &self.data);
    }
}

impl Drop for GraphicsComp {
    fn drop(&mut self) {
        self.set_pos(10.0, 10.0);
        self.graphics.borrow_mut().set_sprite_attr(&self.sprite, self.id, &self.data);
        self.graphics.borrow_mut().return_id(&self.sprite, self.id);
    }
}
