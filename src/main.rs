extern crate rand;
mod body;
mod nbody;
mod simulation;
mod vec2d;

use body::Body;
use piston::event_loop::*;
use piston::input::*;
use rand::Rng;
use simulation::Simulation;

fn main() {
    // Initiate vector of bodies with random pos. and mass
    let mut rng = rand::thread_rng();
    let mut bodies = Vec::new();
    bodies.reserve(10);
    for i in 0..100 {
        let x = rng.gen::<f64>() * 1000000000.0;
        let y = rng.gen::<f64>() * 1000000000.0;
        let m = rng.gen::<f64>() * 1E22;
        let mut b = Body::new(m, x, y, i);
        b.update_velocity(
            (rng.gen::<f64>() - 0.5) * 1000.0,
            (rng.gen::<f64>() - 0.5) * 1000.0,
        );
        bodies.push(b);
    }

    // Init simulation
    let mut sim = Simulation::new(800, 600, &bodies);
    // Event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(sim.get_window()) {
        if let Some(r) = e.render_args() {
            sim.render(&r);
        }
        if let Some(u) = e.update_args() {
            sim.update(&u);
        }
    }
}
