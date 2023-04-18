#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn distance(self, other: Vector3) -> f32 {
        let difference = self.subtract(other);

        return difference.length();
    }

    pub fn add(self, other: Vector3) -> Vector3 {
        return Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn subtract(self, other: Vector3) -> Vector3 {
        return Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    pub fn multiply(self, scale: f32) -> Vector3 {
        return Vector3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        };
    }

    pub fn divide(self, scale: f32) -> Vector3 {
        return Vector3 {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale
        };
    }

    pub fn abs(self) -> Vector3 {
        return Vector3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs()
        }
    }

    pub fn max(self, x: f32) -> Vector3 {
        return Vector3 {
            x: self.x.max(x),
            y: self.y.max(x),
            z: self.z.max(x)
        }
    }

    pub fn normalize(self) -> Vector3 {
        let length = self.length();
        
        return self.divide(length);
    }

    pub fn cross(self, other: Vector3) -> Vector3 {
        let x = self.y * other.z - self.z * other.y;
        let y = -(self.x * other.z - self.z * other.x);
        let z = self.x * other.y - self.y * other.x;

        return Vector3 { x, y, z }
    }

    pub fn dot(self, other: Vector3) -> f32 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    }

    pub fn length(self) -> f32 {
        let sum = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);

        return sum.sqrt();
    }

    pub fn print(self) {
        println!("( {}, {}, {} )", self.x, self.y, self.z);
    }
}