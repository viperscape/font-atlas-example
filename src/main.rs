extern crate glutin;
#[macro_use] extern crate glium;
extern crate nalgebra as na;
extern crate font_atlas;
extern crate font_atlas_image;
extern crate image;

pub use font_atlas::{RenderedFont};
pub use image::DynamicImage;
pub type Font = RenderedFont<DynamicImage>;

use glium::{DisplayBuild,Surface};

use na::{Mat4,Vec2,Vec3,
         ToHomogeneous,
         Ortho3,Iso3, };

mod glyph;
mod atlas;
use glyph::GlyphDrawer;
use atlas::Atlas;

fn main() {
    let mut win_size = (400,400);
    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(win_size.0,win_size.1)
        .with_title("font-atlas example".to_string())
        .build_glium()
        .unwrap();

    let atlas = Atlas::new("assets/fonts/SourceCodePro-Regular-20")
        .expect("Font atlas cannot load, missing fonts?");
    let mut glyph = GlyphDrawer::new(atlas,&display);
    
    'main: loop {
        // update window size when needed
        if let Some(window) = display.get_window() {
            if let Some(size) = window.get_inner_size_pixels() {
                if size.0 > 0 &&
                    size.1 > 0 { win_size = size; }
            }
            else { continue }
        }
        else { break 'main }
        
        let transform = ortho(win_size) * translation(Vec2::new(0.0,0.0));


        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 0.0); //clear as grey
        glyph.draw("Hello World",
                   Vec2::new(1.0,1.0), //no resizing
                   [0.0,1.0,0.0], //overlay font color as green
                   true,
                   transform,
                   &mut target);
        target.finish().unwrap();

        
        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => (),
            }
        }

    }
}


fn ortho(win_size: (u32,u32)) -> Mat4<f32> {
    let ortho = Ortho3::new(
        win_size.0 as f32, win_size.1 as f32,
        -1.0, 1.0
    );

    ortho.to_mat()
}

fn translation(v: Vec2<f32>) -> Mat4<f32> {
    let translation = Iso3::new(
        Vec3::new(v.x, v.y, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    translation.to_homogeneous()
}
