use std::collections::HashMap;
use glium::program::Program;
use glium::VertexBuffer;
use glium::index::IndexBuffer;
use glium::draw_parameters::DrawParameters;
use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
use glium;

use engine::scene::Registry;

pub struct Graphics {
    custom_sprites: HashMap<usize, CustomSpriteData>,
    sprites: HashMap<usize, SpriteData>,
    display: GlutinFacade,
    pub dimensions: (u32, u32),
}

struct SpriteData {
    program: Program,
    vbo: VertexBuffer<SpriteVertex>,
    vertex_attrs: VertexBuffer<SpriteAttrs>,
    indices: IndexBuffer<u16>,
    texture: Option<CompressedSrgbTexture2d>,
    last_amount: usize,
    pub registry: Registry,
}

struct CustomSpriteData {
    program: Program,
    vbo: VertexBuffer<SpriteVertex>,
    indices: IndexBuffer<u16>,
}

impl Graphics {
    pub fn new(x_res: u32, y_res: u32) -> Graphics {
        Graphics {
            sprites: HashMap::new(),
            custom_sprites: HashMap::new(),
            display: glium::glutin::WindowBuilder::new()
                         .with_dimensions(x_res, y_res)
                         .with_max_dimensions(x_res, y_res)
                         .with_min_dimensions(x_res, y_res)
                         .with_vsync()
                         .build_glium()
                         .unwrap(),
            dimensions: (x_res, y_res),
        }
    }

    pub fn new_sprite(&mut self,
                      id: usize,
                      vertex_shader: &str,
                      fragment_shader: &str,
                      vbo: VertexBuffer<SpriteVertex>,
                      texture: Option<CompressedSrgbTexture2d>,
                      max_amount: usize) {
        let prog = Program::from_source(&self.display, vertex_shader, fragment_shader, None)
                       .unwrap();
        let vertex_attrs = VertexBuffer::empty_dynamic(&self.display, max_amount).unwrap();
        let indices = IndexBuffer::new(&self.display,
                                       glium::index::PrimitiveType::TriangleStrip,
                                       &[1 as u16, 2, 0, 3])
                          .unwrap();
        let mut reg = Registry::new();
        reg.no_reclaim();
        let data = SpriteData {
            program: prog,
            vbo: vbo,
            indices: indices,
            vertex_attrs: vertex_attrs,
            texture: texture,
            last_amount: 0,
            registry: reg,
        };
        self.sprites.insert(id, data);
    }

    pub fn new_custom_sprite(&mut self,
                      id: usize,
                      vertex_shader: &str,
                      fragment_shader: &str,
                      vbo: VertexBuffer<SpriteVertex>,
                      ) {
        let prog = Program::from_source(&self.display, vertex_shader, fragment_shader, None)
                       .unwrap();
        let indices = IndexBuffer::new(&self.display,
                                       glium::index::PrimitiveType::TriangleStrip,
                                       &[1 as u16, 2, 0, 3])
                          .unwrap();
        let data = CustomSpriteData {
            program: prog,
            vbo: vbo,
            indices: indices,
        };
        self.custom_sprites.insert(id, data);
    }

    pub fn get_id(&mut self, sprite: &usize) -> Option<usize> {
        match self.sprites.get_mut(sprite) {
            Some(s) => {
                Some(s.registry.get_id())
            }
            None => None,
        }
    }

    pub fn return_id(&mut self, sprite: &usize, id: usize) {
        match self.sprites.get_mut(sprite) {
            Some(s) => {
                s.registry.return_id(id)
            }
            None => { }
        }
    }

    pub fn set_sprite_attr(&mut self, sprite: &usize, pos: usize, attrs: &SpriteAttrs) {
        match self.sprites.get_mut(sprite) {
            Some(s) => {
                // Starts at 1 in registry
                s.vertex_attrs.slice_mut((pos-1)..(pos)).unwrap().write(&[*attrs]);
            }
            None => {}
        }
    }

    pub fn make_sprite_vbo(&self, vertices: &[SpriteVertex]) -> VertexBuffer<SpriteVertex> {
        VertexBuffer::new(&self.display, vertices).unwrap()
    }

    pub fn load_asset(&self, path: &str) -> CompressedSrgbTexture2d {
        use std::fs::File;
        use image;

        let f = File::open(path).unwrap();
        let image = image::load(f, image::PNG)
                        .unwrap()
                        .to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                       image_dimensions);
        glium::texture::CompressedSrgbTexture2d::new(&self.display, image).unwrap()
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        for (_, sprite_data) in &self.sprites {

            if let Some(ref tex) = sprite_data.texture {
                let uniforms = uniform! {
                    tex: tex,
                };
                target.draw((&sprite_data.vbo,
                             sprite_data.vertex_attrs.per_instance().unwrap()),
                            &sprite_data.indices,
                            &sprite_data.program,
                            &uniforms,
                            &Default::default())
                      .unwrap();
            } else {
                let uniforms = uniform!{ };
                target.draw((&sprite_data.vbo,
                             sprite_data.vertex_attrs.per_instance().unwrap()),
                            &sprite_data.indices,
                            &sprite_data.program,
                            &uniforms,
                            &Default::default())
                      .unwrap();
            }
        }
        target.finish().unwrap();
    }

    pub fn get_window_events(&self) -> PollEventsIter {
        self.display.poll_events()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SpriteVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(SpriteVertex, position, tex_coords);

#[derive(Clone, Copy, Debug)]
pub struct SpriteAttrs {
    pub transform: [[f32; 4]; 4],
}

impl SpriteAttrs {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.transform[0][3] += dx;
        self.transform[1][3] += dy;
    }

    pub fn set_rot(&mut self, t: f32) {
        self.transform[0][0] = t.cos();
        self.transform[0][1] = -1.0 * t.sin();
        self.transform[1][0] = t.sin();
        self.transform[1][1] = t.cos();
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.transform[0][3] = x;
        self.transform[1][3] = y;
    }

    pub fn get_pos(self) -> (f32, f32) {
        (self.transform[0][3], self.transform[1][3])
    }
}

impl Default for SpriteAttrs {
    fn default() -> SpriteAttrs {
        SpriteAttrs {
            transform: [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]],
        }
    }
}

implement_vertex!(SpriteAttrs, transform);
