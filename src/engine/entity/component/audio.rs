use std::cell::RefCell;
use std::rc::Rc;

use engine::Engine;
use engine::entity::Entity;
use engine::audio::Audio;

pub struct AudioComp {
    audio: Rc<RefCell<Audio>>
}

impl AudioComp {
    pub fn new<E: Entity>(engine: &Engine<E>) -> AudioComp {
        AudioComp {
            audio: engine.audio.clone()
        }
    }

    pub fn play(&self, id: &usize) {
        self.audio.borrow().play(id);
    }

    pub fn stop(&self) {
        self.audio.borrow().stop();
    }
}
