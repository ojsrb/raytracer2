mod render;
mod utils;
use render::*;
use utils::*;

fn advance_ray(ray: &mut Ray, scene: &[BlackHole]) -> bool {
    for black_hole in scene {
        let dir_to_mass = black_hole.position.clone() - ray.position.clone();

        ray.direction = (dir_to_mass.clone() * black_hole.mass * 0.0005) + ray.direction.clone();
        ray.direction = ray.direction.normalize();

        if black_hole.intersects_with_disc(ray) {
            ray.hit = true;
            ray.color = black_hole.color.clone();
            ray.brightness =
                (black_hole.acretion_disk_r - dir_to_mass.length()) / black_hole.acretion_disk_r;
        } else if dir_to_mass.length() < black_hole.min_distance {
            return true;
        } else if ray.position.z > 100 as f64 {
            return true;
        }
    }
    ray.position.x += ray.direction.x * ray.speed;
    ray.position.y += ray.direction.y * ray.speed;
    ray.position.z += ray.direction.z * ray.speed;
    false
}

pub fn main() {
    let scene = vec![
        BlackHole {
            position: Vector3::new(3.0, 0.3, 10.0),
            mass: 1.0,
            min_distance: 0.5,
            acretion_disk_r: 3.0,
            color: Vector3::new(0.0, 0.0, 255.0),
        },
        // BlackHole {
        //     position: Vector3::new(-3.0, -0.3, 20.0),
        //     mass: 4.1,
        //     min_distance: 0.5,
        //     acretion_disk_r: 3.0,
        //     color: Vector3::new(255.0, 0.0, 0.0),
        // }
    ];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        500,
        500,
        1.57, // fov in radians
    );
    camera.initialize_rays();

    let mut ray_index = 0;
    for row in camera.rays.iter_mut() {
        for ray in row.iter_mut() {
            let mut frames = 0;
            loop {
                let result = advance_ray(ray, &scene);
                frames += 1;
                if result || frames > 1000 {
                    break;
                }
            }
            ray_index += 1;
            println!(
                "{}%",
                (ray_index as f64 / (camera.width * camera.height) as f64) * 100.0
            );
        }
    }

    let display = Display::new(camera);
    display.render("output.png");
}
