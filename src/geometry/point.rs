/// Stores 3 vertex values (x, y, z)
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Point Methods
impl Point {
    // New point
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // Cross product
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Dot product
    pub fn dot(&self, light: &[f32; 3]) -> f32 {
        self.x * light[0] + self.y * light[1] + self.z * light[2]
    }

    // Normalize values 
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length != 0.0 {
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        } else {
            self.clone()
        }
    }
}

// XOR
impl std::ops::BitXor for Point {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// Subtraction
impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

