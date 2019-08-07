use crate::body::Body;
use crate::nbody::NBody;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use piston::window::WindowSettings;

pub struct Simulation {
    graphics: GlGraphics,
    window: GlutinWindow,
    background_color: [f32; 4],
    state: NBody,
}

impl Simulation {
    // Creates a new NBody simulation and a window to display its results
    pub fn new(width: u32, height: u32, bodies: &Vec<Body>) -> Simulation {
        let window_result = WindowSettings::new("nbody-rs", [width, height])
            .graphics_api(OpenGL::V4_0)
            .exit_on_esc(true)
            .build();
        match window_result {
            Err(_e) => panic!("could not initialise window"),
            Ok(w) => Simulation {
                graphics: GlGraphics::new(OpenGL::V4_3),
                window: w,
                background_color: [1.0, 1.0, 1.0, 1.0],
                state: NBody::new(bodies),
            },
        }
    }

    // Draws all of the bodies with according colors to indicate mass.
    pub fn render(&mut self, args: &RenderArgs) {
        const COLOR_SCHEME: [(f32, f32, f32); 20] = [
            (0.871, 0.929, 0.812),
            (0.8, 0.902, 0.741),
            (0.722, 0.871, 0.667),
            (0.631, 0.839, 0.596),
            (0.533, 0.808, 0.525),
            (0.455, 0.776, 0.478),
            (0.384, 0.745, 0.451),
            (0.314, 0.71, 0.431),
            (0.247, 0.675, 0.424),
            (0.114, 0.604, 0.424),
            (0.106, 0.576, 0.443),
            (0.098, 0.549, 0.459),
            (0.086, 0.522, 0.475),
            (0.078, 0.494, 0.482),
            (0.075, 0.443, 0.467),
            (0.067, 0.384, 0.439),
            (0.059, 0.329, 0.408),
            (0.051, 0.278, 0.38),
            (0.047, 0.227, 0.349),
            (0.039, 0.184, 0.318),
        ];
        use graphics::*;
        let c = self.graphics.draw_begin(args.viewport());
        clear(self.background_color, &mut self.graphics);
        let pixels_per_meter = args.window_size[0] / 1000000000.0;
        const RADIUS: f64 = 5000000.0;
        let global_trans = c
            .transform
            .trans(-pixels_per_meter * RADIUS, -pixels_per_meter * RADIUS);
        let bodies = self.state.get_bodies();
        for b in bodies.iter() {
            let dot = ellipse::circle(
                b.get_position().get_x() / 1000000000.0 * args.window_size[0],
                b.get_position().get_y() / 1000000000.0 * args.window_size[1],
                RADIUS * pixels_per_meter,
            );
            // Color range: heavier is darker
            let color_index = (b.get_mass() * 19.0 / 1E22).round() as usize;
            let color = COLOR_SCHEME[color_index];
            ellipse(
                [color.0, color.1, color.2, 1.0],
                dot,
                global_trans,
                &mut self.graphics,
            );
        }
        self.graphics.draw_end();
    }

    // Updates the state of the simulation.
    pub fn update(&mut self, args: &UpdateArgs) {
        // time multiplier, in ms simulation time per ms frame time.
        const TIME_SCALE: f64 = 60000.0;
        self.state.update_positions(TIME_SCALE * args.dt);
        self.state.update_velocities(TIME_SCALE * args.dt)
    }

    pub fn get_window(&mut self) -> &mut GlutinWindow {
        &mut self.window
    }
}
