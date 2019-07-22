use crate::vec2d::Vec2d;

#[derive(Debug, Clone)]
pub struct Body {
    mass: f64,
    position: Vec2d,
    velocity: Vec2d,
    id: i32,
}

impl Body {
    pub fn new(m: f64, x: f64, y: f64, id: i32) -> Body {
        Body {
            mass: m,
            position: Vec2d::new(x, y),
            velocity: Vec2d::new(0.0, 0.0),
            id: id,
        }
    }

    pub fn get_position(&self) -> Vec2d {
        self.position.clone()
    }

    pub fn get_velocity(&self) -> Vec2d {
        self.velocity.clone()
    }

    pub fn update_velocity(&mut self, d_vx: f64, d_vy: f64) {
        let (vx, vy) = (self.velocity.get_x(), self.velocity.get_y());
        self.velocity.set_x(vx + d_vx);
        self.velocity.set_y(vy + d_vy);
    }

    pub fn update_pos(&mut self, dx: f64, dy: f64) {
        let (x, y) = (self.position.get_x(), self.position.get_y());
        self.position.set_x(x + dx);
        self.position.set_y(y + dy);
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: write unit tests
    #[test]
    fn test_new() {
        let b1 = Body::new(100.0, 10.0, -10.0, 0);
        assert_eq!(b1.mass, 100.0);
        assert_eq!(b1.position.get_x(), 10.0);
        assert_eq!(b1.position.get_y(), -10.0);
    }

    #[test]
    fn test_update_pos() {
        let mut b1 = Body::new(0.0, 2.0, 5.0, 0);
        b1.update_pos(3.0, -6.5);
        let p = b1.position;
        assert_eq!(5.0, p.get_x());
        assert_eq!(-1.5, p.get_y())
    }

    #[test]
    fn test_update_velocity() {
        let mut b1 = Body::new(0.0, 0.0, 0.0, 0);
        b1.update_velocity(2.5, -3.5);
        let v = b1.velocity;
        assert_eq!(2.5, v.get_x());
        assert_eq!(-3.5, v.get_y())
    }

}
