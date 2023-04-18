use crate::{vectors::Vector3, world_objects::WorldObject};

pub struct DistanceInfo<'a> {
    pub distance: f32,
    pub object: &'a Box<dyn WorldObject>,
}

pub struct MarchInfo<'a> {
    pub total_distance: f32,
    pub object: Option<&'a Box<dyn WorldObject>>,
}

pub struct World {
    pub objects: Vec<Box<dyn WorldObject>>,
    pub light_angle: f32,
}

impl World {
    fn get_signed_distance(&self, point: Vector3) -> DistanceInfo {
        let mut minimum_distance = f32::MAX;
        let mut closest_object = &self.objects[0];

        for object in self.objects.iter() {
            let distance = object.get_signed_distance(point);

            if distance < minimum_distance {
                minimum_distance = distance;
                closest_object = object;
            }
        }

        return DistanceInfo {
            distance: minimum_distance,
            object: closest_object,
        };
    }

    fn get_light_direction(&self) -> Vector3 {
        return Vector3 {
            x: self.light_angle.cos(),
            y: 1.0,
            z: self.light_angle.sin(),
        }
        .normalize();
    }
}

const MAX_STEPS: i32 = 100;
const MAX_DISTANCE: f32 = 1000.0;
const THRESHOLD_DISTANCE: f32 = 0.001;
pub fn march(world: &World, origin: Vector3, direction: Vector3) -> MarchInfo {
    let mut total_distance = 0.0;

    for _i in 0..MAX_STEPS {
        let point = origin.add(direction.multiply(total_distance));
        let info = world.get_signed_distance(point);

        total_distance += info.distance;

        if total_distance > MAX_DISTANCE {
            break;
        }

        if info.distance < THRESHOLD_DISTANCE {
            return MarchInfo {
                total_distance: total_distance,
                object: Some(info.object),
            };
        }
    }

    return MarchInfo {
        total_distance: total_distance,
        object: None,
    };
}

pub fn get_light(world: &World, point: Vector3) -> f32 {
    let light = world.get_light_direction();

    let normal = get_normal(world, point);

    let march_info = march(
        world,
        point.add(normal.multiply(THRESHOLD_DISTANCE * 2.0)),
        light,
    );
    let mut dif = normal.dot(light).clamp(0.0, 1.0);

    if !march_info.object.is_none() {
        dif *= 0.1;
    }

    return dif;
}

fn get_normal(world: &World, point: Vector3) -> Vector3 {
    let distance = world.get_signed_distance(point).distance;
    const E: Vector3 = Vector3 {
        x: 0.01,
        y: 0.0,
        z: 0.0,
    };

    let normal = Vector3 {
        x: distance
            - world
                .get_signed_distance(point.subtract(Vector3 {
                    x: E.x,
                    y: E.y,
                    z: E.y,
                }))
                .distance,
        y: distance
            - world
                .get_signed_distance(point.subtract(Vector3 {
                    x: E.y,
                    y: E.x,
                    z: E.y,
                }))
                .distance,
        z: distance
            - world
                .get_signed_distance(point.subtract(Vector3 {
                    x: E.y,
                    y: E.y,
                    z: E.x,
                }))
                .distance,
    };

    return normal.normalize();
}
