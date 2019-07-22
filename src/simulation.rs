extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::body::Body;
use crate::state::State;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use piston::window::WindowSettings;

pub struct Simulation {
    graphics: GlGraphics,
    window: GlutinWindow,
    background_color: [f32; 4],
    state: State,
}

impl Simulation {
    pub fn new(width: u32, height: u32, bodies: &Vec<Body>) -> Simulation {
        let window_result = WindowSettings::new("nbody-rs", [width, height])
            .graphics_api(OpenGL::V4_3)
            .exit_on_esc(true)
            .build();
        match window_result {
            Err(_e) => panic!("could not initialise window"),
            Ok(w) => Simulation {
                graphics: GlGraphics::new(OpenGL::V4_3),
                window: w,
                background_color: [1.0, 1.0, 1.0, 1.0],
                state: State::new(bodies),
            },
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let c = self.graphics.draw_begin(args.viewport());
        clear(self.background_color, &mut self.graphics);
        let r = args.window_size[0] / 100.0;
        let global_trans = c.transform.trans(-r, -r);
        let bodies = self.state.get_bodies();
        for b in bodies.iter() {
            let dot = ellipse::circle(
                b.get_position().get_x() / 1000000000.0 * args.window_size[0],
                b.get_position().get_y() / 1000000000.0 * args.window_size[1],
                r,
            );
            // Color range: heavier is darker
            let color: [f32; 4] = [
                (b.get_mass() / 2E22) as f32,
                0.0,
                (b.get_mass() / 2E22) as f32,
                1.0,
            ];
            ellipse(color, dot, global_trans, &mut self.graphics);
        }
        self.graphics.draw_end();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // time multiplier, in ms simulation time to ms frame time.
        const TIME_SCALE: f64 = 60000.0;
        self.state.update_positions(TIME_SCALE * args.dt);
        self.state.update_velocities(TIME_SCALE * args.dt)
    }

    pub fn get_window(&mut self) -> &mut GlutinWindow {
        &mut self.window
    }
}
