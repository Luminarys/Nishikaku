use std::mem;
use std::fs::File;
use ncollide_geometry::shape::ShapeHandle2;
use glium::program::Program;
use glium::VertexBuffer;
use glium::index::IndexBuffer;
use glium::draw_parameters::DrawParameters;
use glium::uniforms::Uniforms;
use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::texture::compressed_srgb_texture2d::CompressedSrgbTexture2d;
use glium::{self, Frame};
use glium_text::{self, FontTexture, TextSystem, TextDisplay};
use imgui::{ImGui, Ui};
use imgui::glium_renderer::Renderer;

use util::{self, HashMap};
use scene::Registry;

// TODO: Using reversed matrices is probably a bad practice, invert everything

pub struct Graphics {
    custom_sprites: HashMap<usize, CustomSpriteData>,
    sprites: HashMap<usize, SpriteData>,
    display: GlutinFacade,
    fonts: HashMap<usize, FontTexture>,
    tex_sys: TextSystem,
    current_frame: Option<Frame>,
    pub dimensions: (u32, u32),
    pub scaler: f32,
}

struct SpriteData {
    program: Program,
    vbo: VertexBuffer<SpriteVertex>,
    pre_render: Vec<SpriteAttrs>,
    vertex_attrs: VertexBuffer<SpriteAttrs>,
    indices: IndexBuffer<u16>,
    texture: Option<CompressedSrgbTexture2d>,
    shape: Option<ShapeHandle2<f32>>,
    pub registry: Registry,
}

struct CustomSpriteData {
    program: Program,
    vbo: VertexBuffer<SpriteVertex>,
    indices: IndexBuffer<u16>,
    shape: Option<ShapeHandle2<f32>>,
}

use std::fmt;
impl fmt::Debug for SpriteData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sprite has shape? {:?}", !self.shape.is_none())
    }
}

impl Graphics {
    pub fn new((x_res, y_res): (u32, u32), scaler: f32) -> Graphics {
        let display = glium::glutin::WindowBuilder::new()
                          .with_dimensions(x_res, y_res)
                          .with_title(String::from("Nishikaku"))
                          .with_max_dimensions(x_res, y_res)
                          .with_min_dimensions(x_res, y_res)
                          .with_vsync()
                          .build_glium()
                          .unwrap();
        let tex_sys = TextSystem::new(&display);
        Graphics {
            scaler: scaler,
            sprites: util::hashmap(),
            custom_sprites: util::hashmap(),
            display: display,
            fonts: util::hashmap(),
            current_frame: None,
            tex_sys: tex_sys,
            dimensions: (x_res, y_res),
        }
    }

    pub fn new_sprite(&mut self,
                      id: usize,
                      vertex_shader: &str,
                      fragment_shader: &str,
                      vbo: VertexBuffer<SpriteVertex>,
                      texture: Option<CompressedSrgbTexture2d>,
                      max_amount: usize,
                      shape: Option<ShapeHandle2<f32>>) {
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
            registry: reg,
            shape: shape,
            pre_render: vec![SpriteAttrs::hidden(); max_amount],
        };
        self.sprites.insert(id, data);
    }

    pub fn sprite_amount(&self) -> usize {
        self.sprites.len()
    }

    pub fn new_custom_sprite(&mut self,
                             id: usize,
                             vertex_shader: &str,
                             fragment_shader: &str,
                             vbo: VertexBuffer<SpriteVertex>,
                             shape: Option<ShapeHandle2<f32>>) {
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
            shape: shape,
        };
        self.custom_sprites.insert(id, data);
    }

    pub fn get_id(&mut self, sprite: &usize) -> Option<usize> {
        match self.sprites.get_mut(sprite) {
            Some(s) => Some(s.registry.get_id()),
            None => None,
        }
    }

    pub fn return_id(&mut self, sprite: &usize, id: usize) {
        match self.sprites.get_mut(sprite) {
            Some(s) => s.registry.return_id(id),
            None => {}
        }
    }

    pub fn get_sprite_shape(&self, sprite: &usize) -> Option<ShapeHandle2<f32>> {
        match self.sprites.get(sprite) {
            Some(s) => s.shape.clone(),
            None => None
        }
    }

    pub fn set_sprite_attr(&mut self, sprite: &usize, pos: usize, attrs: &SpriteAttrs) {
        match self.sprites.get_mut(sprite) {
            Some(s) => {
                // Starts at 1 in registry
                s.pre_render[pos-1] = *attrs;
                // match s.vertex_attrs.slice_mut((pos - 1)..(pos)) {
                //     Some(slice) => slice.write(&[*attrs]),
                //     None => panic!(format!("Failed to write to sprite {:?} at pos {:?}", sprite, pos))
                // }
            }
            None => {}
        }
    }

    pub fn make_sprite_vbo(&self, vertices: &[SpriteVertex]) -> VertexBuffer<SpriteVertex> {
        VertexBuffer::new(&self.display, vertices).unwrap()
    }

    pub fn load_texture(&self, path: &str) -> CompressedSrgbTexture2d {
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

    pub fn load_font(&mut self, id: usize, path: &str) {
        let f = File::open(path).unwrap();
        let font = FontTexture::new(&self.display, f, 36).unwrap();
        self.fonts.insert(id, font);
    }

    pub fn make_menu_renderer(&self, imgui: &mut ImGui) -> Renderer {
        Renderer::init(imgui, &self.display).unwrap()
    }

    pub fn start_frame(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        self.current_frame = Some(target);
    }

    pub fn get_menu_frame<'imgui>(&self, imgui: &'imgui mut ImGui, dt: f32) -> Ui<'imgui> {
        let window = self.display.get_window().unwrap();
        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();
        imgui.frame(size_points, size_pixels, dt)
    }

    pub fn render_menu(&mut self, renderer: &mut Renderer, ui_frame: Ui) {
        match self.current_frame {
            Some(ref mut target) => {
                renderer.render(target, ui_frame).unwrap();
            }
            None => {
                println!("Cannot render menu without initialized frame!");
            }
        }
    }

    pub fn render_custom<U: Uniforms>(&mut self,
                                      sprite: &usize,
                                      uniforms: &U,
                                      params: &DrawParameters) {
        match (&mut self.current_frame, self.custom_sprites.get(sprite)) {
            (&mut Some(ref mut target), Some(sprite_data)) => {
                target.draw(&sprite_data.vbo,
                            &sprite_data.indices,
                            &sprite_data.program,
                            uniforms,
                            &params)
                      .unwrap();
            }
            (&mut None, _) => {
                println!("Cannot render custom sprite without initialized frame!");
            }
            (_, None) => {
                println!("Invalid custom sprite identifier passed!");
            }
        }
    }

    pub fn render_text(&mut self,
                       id: &usize,
                       msg: &str,
                       transform: &[[f32; 4]; 4],
                       color: &(f32, f32, f32, f32)) {
        match self.current_frame {
            Some(ref mut target) => {
                if let Some(font) = self.fonts.get(id) {
                    let text = TextDisplay::new(&self.tex_sys, font, msg);
                    glium_text::draw(&text, &self.tex_sys, target, *transform, *color);
                }
            }
            None => {
                println!("Cannot render text without initialized frame!");
            }
        }
    }

    pub fn render(&mut self) {
        use glium::Blend;

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };
        match self.current_frame {
            Some(ref mut target) => {
                for (&_, ref mut sprite_data) in self.sprites.iter_mut() {
                    sprite_data.vertex_attrs.write(&sprite_data.pre_render[..]);
                    if let Some(ref tex) = sprite_data.texture {
                        let uniforms = uniform! {
                            tex: tex,
                        };
                        target.draw((&sprite_data.vbo,
                                     sprite_data.vertex_attrs.per_instance().unwrap()),
                                    &sprite_data.indices,
                                    &sprite_data.program,
                                    &uniforms,
                                    &params)
                              .unwrap();
                    } else {
                        let uniforms = uniform![];
                        target.draw((&sprite_data.vbo,
                                     sprite_data.vertex_attrs.per_instance().unwrap()),
                                    &sprite_data.indices,
                                    &sprite_data.program,
                                    &uniforms,
                                    &params)
                              .unwrap();
                    }
                }
            }
            None => {
                println!("Frame must be started before rendering can occur!");
            }
        }
    }

    pub fn finish_frame(&mut self) {
        if self.current_frame.is_none() {
            println!("A frame must be started before being finished!");
        } else {
            let target = mem::replace(&mut self.current_frame, None);
            target.unwrap().finish().unwrap();
        }
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
    pub fn hidden() -> SpriteAttrs {
        let mut sa = SpriteAttrs::default();
        sa.set_pos(10f32, 10f32);
        sa
    }
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
