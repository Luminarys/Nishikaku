use std::collections::HashMap;
use glium::program::Program;
use glium::VertexBuffer;
use glium::index::IndexBuffer;
use glium::draw_parameters::DrawParameters;
use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
use glium;

pub struct Graphics<'a> {
    sprites: HashMap<usize, SpriteData<'a>>,
    display: GlutinFacade,
}

struct SpriteData<'a> {
    program: Program,
    vbo: VertexBuffer<SpriteVertex>,
    vertex_attrs: VertexBuffer<SpriteAttrs>,
    indices: IndexBuffer<u16>,
    texture: CompressedSrgbTexture2d,
    draw_params: DrawParameters<'a>,
    last_amount: usize,
}

impl<'a> Graphics<'a> {
    pub fn new() -> Graphics<'a> {
        Graphics {
            sprites: Default::default(),
            display: glium::glutin::WindowBuilder::new().build_glium().unwrap(),
        }
    }

    pub fn new_sprite(&mut self,
                      id: usize,
                      vertex_shader: &str,
                      fragment_shader: &str,
                      vbo: VertexBuffer<SpriteVertex>,
                      texture: CompressedSrgbTexture2d,
                      max_amount: usize) {
        let prog = Program::from_source(&self.display, vertex_shader, fragment_shader, None)
                       .unwrap();
        let vertex_attrs = VertexBuffer::empty_dynamic(&self.display, max_amount).unwrap();
        let draw_params = Default::default();
        let indices = IndexBuffer::new(&self.display,
                                       glium::index::PrimitiveType::TriangleStrip,
                                       &[1 as u16, 2, 0, 3])
                          .unwrap();
        let data = SpriteData {
            program: prog,
            vbo: vbo,
            indices: indices,
            vertex_attrs: vertex_attrs,
            texture: texture,
            draw_params: draw_params,
            last_amount: 0,
        };
        self.sprites.insert(id, data);
    }

    pub fn set_sprite_attrs(&mut self, id: &usize, attrs: &[SpriteAttrs]) {
        const REMOVED_ATTRS: SpriteAttrs = SpriteAttrs {
            transform: [[1.0, 0.0, 0.0, 100.0],
                        [0.0, 1.0, 0.0, 100.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]],
        };
        match self.sprites.get_mut(id) {
            Some(s) => {
                s.vertex_attrs.invalidate();
                if attrs.len() > 0 {
                    s.vertex_attrs.slice_mut(0..(attrs.len() - 1)).unwrap().write(attrs);
                }
                if attrs.len() < s.last_amount {
                    // This might not be that efficient
                    s.vertex_attrs
                     .slice_mut(attrs.len()..s.last_amount)
                     .unwrap()
                     .write(&vec![REMOVED_ATTRS; s.last_amount - attrs.len()]);
                }
                s.last_amount = attrs.len();
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
            let uniforms = uniform! {
                tex: &sprite_data.texture,
            };
            target.draw((&sprite_data.vbo,
                         sprite_data.vertex_attrs.per_instance().unwrap()),
                        &sprite_data.indices,
                        &sprite_data.program,
                        &uniforms,
                        &sprite_data.draw_params)
                  .unwrap();
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
