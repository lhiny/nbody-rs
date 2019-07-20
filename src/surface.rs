extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct Surface {
    gl: GlGraphics,
    window: GlutinWindow,
    background_color: [f32; 4],
    drawing_color: [f32; 4],
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Surface {
        let window_result = WindowSettings::new("nbody-rs", [width, height])
            .graphics_api(OpenGL::V4_3)
            .exit_on_esc(true)
            .build();
        match window_result {
            Err(e) => panic!("could not initialise window"),
            Ok(w) => Surface {
                gl: GlGraphics::new(OpenGL::V4_3),
                window: w,
                background_color: [1.0, 1.0, 1.0, 1.0],
                drawing_color: [1.0, 0.0, 0.0, 1.0],
            },
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let c = self.gl.draw_begin(args.viewport());
        clear(self.background_color, &mut self.gl);
        let transform = c.transform.trans(x, y).trans(-25.0, -25.0);
        rectangle(self.drawing_color, square, transform, &mut self.gl);
        self.gl.draw_end();
    }

    pub fn update(&mut self, args: &UpdateArgs) {}
}
