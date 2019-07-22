use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vec2d {
    x: f64,
    y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x: x, y: y }
    }

    pub fn dist_sq(&self, other: &Vec2d) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }

    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Vec2d {
        let l = self.length_sq().sqrt();
        Vec2d {
            x: self.x / l,
            y: self.y / l,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, value: f64) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: f64) {
        self.y = value;
    }
}

impl Add for Vec2d {
    type Output = Vec2d;
    fn add(self, other: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Scalar multiply
impl Mul<f64> for Vec2d {
    type Output = Vec2d;
    fn mul(self, other: f64) -> Self::Output {
        Vec2d {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;
    fn sub(self, other: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let v1 = Vec2d::new(20.0, 5.0);
        assert_eq!(v1.x, 20.0);
        assert_eq!(v1.y, 5.0)
    }
    #[test]
    fn test_add() {
        let v1 = Vec2d::new(1.0, 2.0);
        let v2 = Vec2d::new(3.0, 4.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 6.0);
    }
    #[test]
    fn test_sub() {
        let v1 = Vec2d::new(0.0, 1.0);
        let v2 = Vec2d::new(5.0, -10.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -5.0);
        assert_eq!(v3.y, 11.0);
    }

    #[test]
    fn test_dist_squared() {
        // TODO
    }
}
