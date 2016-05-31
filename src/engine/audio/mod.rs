use std::path::Path;
use std::collections::HashMap;
use sdl2;
use sdl2_mixer;
use sdl2::AudioSubsystem;
use sdl2_mixer::{INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG, INIT_OGG, AUDIO_S16LSB, Music, Sdl2MixerContext};

pub struct Audio {
    sounds: HashMap<usize, Music>,
    mixer: Sdl2MixerContext,
    subsystem: AudioSubsystem,
}

impl Audio {
    pub fn new() -> Audio {
        let sdl = sdl2::init().unwrap();
        let audio = sdl.audio().unwrap();
        let mixer_context = sdl2_mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH | INIT_MODPLUG | INIT_OGG).unwrap();
        let freq = 44100;
        let format = AUDIO_S16LSB; // signed 16 bit samples
        let channels = 2; // Stereo
        let chunk_size = 1024;
        let _ = sdl2_mixer::open_audio(freq, format, channels, chunk_size).unwrap();
        sdl2_mixer::allocate_channels(0);
        Audio {
            sounds: Default::default(),
            mixer: mixer_context,
            subsystem: audio,
        }
    }

    pub fn load(&mut self, id: usize, path: &Path) {
        let music = sdl2_mixer::Music::from_file(path).unwrap();
        self.sounds.insert(id, music);
    }

    pub fn play(&self, id: &usize) {
        match self.sounds.get(id) {
            Some(music) => { music.play(1).unwrap(); }
            None => { println!("Tried to play invalid audio with id: {}", id); }
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        sdl2_mixer::close_audio();
    }
}
