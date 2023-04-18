use crate::vectors::Vector3;

pub trait WorldObject {
    fn get_signed_distance(&self, point: Vector3) -> f32;
    fn get_position(&self) -> Vector3;
}

pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
}

impl WorldObject for Sphere {
    fn get_signed_distance(&self, point: Vector3) -> f32 {
        let distance = self.position.distance(point);
        let signed_distance = distance - self.radius;

        return signed_distance;
    }

    fn get_position(&self) -> Vector3 {
        return self.position;
    }
}

pub struct Cube {
    pub position: Vector3,
    pub size: Vector3,
}

impl WorldObject for Cube {
    fn get_signed_distance(&self, point: Vector3) -> f32 {
        let o = point.subtract(self.position).abs().subtract(self.size);
        let ud = o.max(0.0).length();
        let n = o.x.min(0.0).max(o.y.min(0.0)).max(o.z.min(0.0));

        return ud + n;
    }

    fn get_position(&self) -> Vector3 {
        return self.position;
    }
}

pub struct Ground {}

impl WorldObject for Ground {
    fn get_signed_distance(&self, point: Vector3) -> f32 {
        return point.y;
    }

    fn get_position(&self) -> Vector3 {
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
}
