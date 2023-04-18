use crate::vectors::Vector3;

pub fn get_forward(pitch: f32, yaw: f32) -> Vector3 {
    let xz_len = pitch.cos();

    let x = xz_len * (-yaw).sin();
    let y = pitch.sin();
    let z = xz_len * yaw.cos();

    return Vector3 { x: x, y: y, z: z }
}

pub fn get_backward(pitch: f32, yaw: f32) -> Vector3 {
    return get_forward(pitch, yaw).multiply(-1.0);
}

pub fn get_up(pitch: f32, yaw: f32) -> Vector3 {
    let xz_len = pitch.cos();

    let x = xz_len * (-yaw).sin();
    let y = xz_len * yaw.cos();
    let z = pitch.sin();

    return Vector3 { x: x, y: y, z: z }
}

pub fn get_down(pitch: f32, yaw: f32) -> Vector3 {
    return get_up(pitch, yaw).multiply(-1.0);
}

pub fn get_right(pitch: f32, yaw: f32) -> Vector3 {
    let forward = get_forward(pitch, yaw);

    return Vector3 { x: forward.z, y: forward.y, z: -forward.x }
}

pub fn get_left(pitch: f32, yaw: f32) -> Vector3 {
    return get_right(pitch, yaw).multiply(-1.0);
}