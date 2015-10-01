use std::collections::HashMap;
use std::char;

use na::{Mat4,Vec2,Vec3,Iso3,Vec4, ToHomogeneous};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use font_atlas::{CharInfo};
use glium::texture::{Texture2d};


use Font;
use atlas::Atlas;

static VERT_SRC: &'static str = r"
    #version 120

    attribute vec2 pos;
    attribute vec2 tex;

    uniform mat4 transform;
    uniform vec2 size;

    varying vec2 v_tex_coord;

    void main() {
        gl_Position = transform * vec4(pos * size, 0.0, 1.0);
        v_tex_coord = tex;
    }
";

static FRAG_SRC: &'static str = r"
    #version 120

    varying vec2 v_tex_coord;

    uniform sampler2D sample;
    uniform vec4 o_color;

    void main() {
        gl_FragColor = o_color * texture2D(sample, v_tex_coord);
    }
";

#[derive(Copy,Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

pub type GlyphCache = HashMap<char,(CharInfo,Texture2d)>;

pub struct GlyphDrawer {
    vbo: glium::vertex::VertexBufferAny,
    program: glium::Program,
    font: Font,
    cache:  GlyphCache,
}

impl GlyphDrawer {
    pub fn new(mut font: Font, window: &Display) -> GlyphDrawer {
        implement_vertex!(Vertex, pos, tex);
        let verts = vec![
            Vertex { pos: [ -0.5,  0.5 ], tex: [ 0.0, 0.0 ] },
            Vertex { pos: [ -0.5, -0.5 ], tex: [ 0.0, 1.0 ] },
            Vertex { pos: [  0.5,  0.5 ], tex: [ 1.0, 0.0 ] },
            Vertex { pos: [  0.5, -0.5 ], tex: [ 1.0, 1.0 ] },
            ];
        
        let program = program!(window,
                               120 => { vertex: VERT_SRC,
                                        fragment: FRAG_SRC, } ).unwrap();
        let vbo = glium::vertex::VertexBuffer::new(window, &verts).unwrap().into_vertex_buffer_any();

        let cache = GlyphDrawer::load_glyphs(&mut font, window);
        
        GlyphDrawer {
            vbo: vbo,
            program: program,
            font: font,
            cache: cache,
        }
    }

    pub fn draw(
        &mut self,
        text: &str,
        color: [f32;3],
        transform: Mat4<f32>,
        target: &mut glium::Frame,
        ) {
        let width = text.chars().count();

        for (i, c) in text.chars().enumerate() {
            if let Some(cache) = self.cache.get(&c) {
                let offset_x = (width as i32 * cache.0.advance.0) / 2;
                
                let position = Vec2::new((i as f32 *
                                          cache.0.advance.0 as f32)
                                         - offset_x as f32,
                                         0.0);

                let img_size = Vec2::new(cache.0.image_size.0 as f32,
                                         cache.0.image_size.1 as f32);
                
                let position = position + (img_size * 0.5);

                let translation = Iso3::new(
                    Vec3::new(position.x, position.y, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    );
                let transform = transform * translation.to_homogeneous();
                
                
                let uniforms = uniform! {
                    transform: *transform.as_array(),
                    size: *img_size.as_array(),
                    sample: &cache.1,
                    o_color: *Vec4::new(color[0],color[1],color[2],1.0).as_array(),
                };

                // draw parameters
                let params = glium::DrawParameters {
                    blend: glium::Blend {
                        color: glium::BlendingFunction::Addition {
                            source: glium::LinearBlendingFactor::One,
                            destination: glium::LinearBlendingFactor::One
                        },
                        alpha: glium::BlendingFunction::Addition {
                            source: glium::LinearBlendingFactor::One,
                            destination: glium::LinearBlendingFactor::One
                        },
                        constant_value: (1.0, 1.0, 1.0, 1.0)
                    },
                    .. Default::default()
                };

                target.draw(&self.vbo,
                            &glium::index::NoIndices
                            (glium::index::PrimitiveType::TriangleStrip),
                            &self.program, &uniforms, &params).unwrap();
            }
            else if c!=' ' { println!("{:?}, no char found",c); }
        }
    }

    fn load_glyphs (font: &mut Font,
                    display: &Display) -> GlyphCache {
        let mut cache = HashMap::new();

        for i in (0 .. 0xd7ff + 1)
            .chain((0xe000 .. 0x10ffff + 1)) {
            let c = char::from_u32(i).unwrap();
            
            let g = Atlas::sample_tex(c,
                                      font,
                                      display);
            if let Some(g) = g {
                cache.insert(c,g);
            }

        }
        
        cache
    }
}


