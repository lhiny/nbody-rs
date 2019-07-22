use crate::body::Body;
use crate::vec2d::Vec2d;

#[derive(Debug)]
pub struct State {
    bodies: Vec<Body>,
}

impl State {
    pub fn new(bodies: &Vec<Body>) -> State {
        State {
            bodies: bodies.to_owned(),
        }
    }

    pub fn get_bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    fn calculate_acc(&self) -> Vec<Vec2d> {
        const G: f64 = 6.67408E-11;
        let mut accelerations: Vec<Vec2d> = Vec::new();
        for b in self.bodies.iter() {
            let mut acc = Vec2d::new(0.0, 0.0);
            for other in self.bodies.iter() {
                if other.get_id() == b.get_id() {
                    continue;
                }
                let r2 = other.get_position().dist_sq(&b.get_position());
                let a_scalar = G * other.get_mass() / r2;
                let direction = (other.get_position() - b.get_position()).normalized();
                acc = acc + (direction * a_scalar);
            }
            accelerations.push(acc);
        }
        accelerations
    }

    // dt: time, in seconds, to update the system by. Not to be confused with frame time
    pub fn update_positions(&mut self, dt: f64) {
        // First, calculate acceleration
        for b in self.bodies.iter_mut() {
            let v = b.get_velocity();
            b.update_pos(v.get_x() * dt, v.get_y() * dt);
        }
    }

    pub fn update_velocities(&mut self, dt: f64) {
        let accelerations = self.calculate_acc();
        let mut id = 0;
        for b in self.bodies.iter_mut() {
            let acc = accelerations.get(id).unwrap();
            // print!("b{}: {} {} ", id, acc.get_x(), acc.get_y());
            b.update_velocity(acc.get_x() * dt, acc.get_y() * dt);
            id += 1;
        }
        // println!();
    }
}
