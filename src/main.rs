use crossterm::{
    cursor,
    style::{self},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdin, stdout, Write};

use crate::{
    marching::{get_light, march, World},
    vectors::Vector3,
    world_objects::{Cube, Ground, Sphere},
};

pub mod camera;
pub mod marching;
pub mod vectors;
pub mod world_objects;

fn main() -> Result<()> {
    let mut stdout = stdout();
    let stdin = stdin();

    const WIDTH: u16 = 200;
    const HEIGHT: u16 = 60;
    const LIGHT_CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
    const SPEED: f32 = 1.0;

    let mut world = World {
        objects: Vec::new(),
        light_angle: 0.0,
    };

    let sphere = Sphere {
        position: Vector3 {
            x: 0.0,
            y: 5.0,
            z: 15.0,
        },
        radius: 3.0,
    };
    let cube = Cube {
        position: Vector3 {
            x: 0.0,
            y: 15.0,
            z: 15.0,
        },
        size: Vector3 {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        },
    };
    let ground = Ground {};

    world.objects.push(Box::new(sphere));
    world.objects.push(Box::new(cube));
    world.objects.push(Box::new(ground));

    let mut origin = Vector3 {
        x: 0.0,
        y: 3.0,
        z: 0.0,
    };

    let mut pitch = 0.0;
    let mut yaw = 0.0;

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    loop {
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        for j in (0..HEIGHT).rev() {
            for i in 0..WIDTH {
                let frag_coord = Vector3 {
                    x: i as f32,
                    y: j as f32,
                    z: 0.0,
                };

                let uv = (frag_coord.subtract(
                    Vector3 {
                        x: WIDTH as f32,
                        y: HEIGHT as f32,
                        z: 0.0,
                    }
                    .multiply(0.5),
                ))
                .divide(HEIGHT as f32);

                let direction = camera::get_forward(pitch, yaw)
                    .add(Vector3 {
                        x: uv.x,
                        y: uv.y,
                        z: 0.0,
                    })
                    .normalize();

                let march_info = march(&world, origin, direction);
                if march_info.object.is_none() {
                    print!(" ");
                    continue;
                }

                let point = origin.add(direction.multiply(march_info.total_distance));
                let dif = get_light(&world, point);

                let index = (dif * (LIGHT_CHARS.len() - 1) as f32).round();
                let character = LIGHT_CHARS[index as usize];

                stdout.queue(style::Print(character)).unwrap();
            }

            stdout.queue(style::Print("\n")).unwrap();
        }
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let char = input.trim();

        let offset: Vector3;
        match char {
            "w" => offset = camera::get_forward(pitch, yaw),
            "s" => offset = camera::get_backward(pitch, yaw),
            "d" => offset = camera::get_right(pitch, yaw),
            "a" => offset = camera::get_left(pitch, yaw),
            "z" => offset = camera::get_up(pitch, yaw),
            "c" => offset = camera::get_down(pitch, yaw),
            "x" => {
                world.light_angle += std::f32::consts::PI * 0.025;
                offset = Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
            }
            "e" => {
                yaw -= std::f32::consts::PI * 0.025;
                offset = Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
            }
            "q" => {
                yaw += std::f32::consts::PI * 0.025;
                offset = Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
            }
            _ => {
                offset = Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        }

        origin = origin.add(offset.multiply(SPEED));
        origin.print();
    }
}
