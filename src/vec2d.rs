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

    // Calculates the distance to another vector in space, squared
    pub fn dist_sq(&self, other: &Vec2d) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }

    // Calculates the length of this vector
    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    // Returns a normalized (i.e. length 1) version of this vector
    pub fn normalized(&self) -> Vec2d {
        let l = self.length_sq().sqrt();
        if l == 0.0 {
            return Vec2d::new(0.0, 0.0);
        }
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

// Operator overload for addition
impl Add for Vec2d {
    type Output = Vec2d;
    fn add(self, other: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Operator overload for subtraction
impl Sub for Vec2d {
    type Output = Vec2d;
    fn sub(self, other: Vec2d) -> Self::Output {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Operator overload for (scalar) multiplication
impl Mul<f64> for Vec2d {
    type Output = Vec2d;
    fn mul(self, other: f64) -> Self::Output {
        Vec2d {
            x: self.x * other,
            y: self.y * other,
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
    fn test_dist_sq() {
        let v1 = Vec2d::new(5.0, 4.0);
        let v2 = Vec2d::new(1.0, 2.0);
        let result = v1.dist_sq(&v2);
        assert_eq!(20.0, result);
        let result = v2.dist_sq(&v1);
        assert_eq!(20.0, result);
    }

    #[test]
    fn test_length_sq() {
        let v1 = Vec2d::new(0.0, 0.0);
        assert_eq!(0.0, v1.length_sq(),);
        let v2 = Vec2d::new(3.0, 4.0);
        assert_eq!(25.0, v2.length_sq(),);
    }

    #[test]
    fn test_normalized() {
        let v1 = Vec2d::new(0.0, 0.0).normalized();
        assert_eq!(0.0, v1.get_x());
        assert_eq!(0.0, v1.get_y());
        let v2 = Vec2d::new(3.0, 4.0).normalized();
        assert_eq!(0.6, v2.get_x());
        assert_eq!(0.8, v2.get_y());
    }

    #[test]
    fn test_mul() {
        let mut v1 = Vec2d::new(101.0, 121.0);
        v1 = v1 * 2.0;
        assert_eq!(202.0, v1.get_x());
        assert_eq!(242.0, v1.get_y());
    }
}
