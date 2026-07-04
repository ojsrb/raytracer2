mod render;
mod utils;
use render::*;
use std::io::Write;
use std::time::Instant;
use utils::*;

fn advance_ray(ray: &mut Ray, scene: &[BlackHole]) -> bool {
    for black_hole in scene {
        let dir_to_mass = black_hole.position.clone() - ray.position.clone();

        let new_direction = (dir_to_mass.clone().normalize() * black_hole.mass * 0.03)
            / dir_to_mass.length()
            + ray.direction.clone();
        ray.direction = new_direction;
        ray.direction = ray.direction.normalize();

        if black_hole.intersects_with_disc(ray) {
            let dist_to_disk = (ray.position.y - black_hole.position.y).abs();
            let downwards_angle = (ray.direction.y / ray.direction.length()).acos();

            let last_step_dist = dist_to_disk / downwards_angle.cos();

            ray.position = ray.position.clone() + ray.direction.clone() * last_step_dist;

            ray.hit = true;

            let angle = black_hole.angle;

            let mut shifted_position = ray.position.clone() - black_hole.position.clone();
            shifted_position.y = black_hole.position.y;

            let r = shifted_position.length() * 2.0;
            let theta = shifted_position.z.atan2(shifted_position.x) / 4.0 + angle;

            let dist_brightness = black_hole.acretion_disk_r / dir_to_mass.length() * 0.4;

            ray.brightness =
                black_hole.texture.as_ref().unwrap().sample(r, theta) + dist_brightness;
            ray.color = black_hole.color.clone() * ray.brightness;
            return true;
        } else if dir_to_mass.length() < black_hole.min_distance {
            return true;
        } else if ray.position.z > 20 as f64 {
            return true;
        }
    }
    ray.position.x += ray.direction.x * ray.speed;
    ray.position.y += ray.direction.y * ray.speed;
    ray.position.z += ray.direction.z * ray.speed;
    false
}

pub fn main() {
    let normal_view_pos = Vector3::new(0.0, -1.0, 0.0);

    let disk_view_pos = Vector3::new(0.0, -4.0, 2.0);

    let count = 1;
    for i in 0..count {
        let angle = (i as f64) * (3.14 / count as f64);
        println!("Rendering angle {} radians", angle);
        let scene = vec![BlackHole::new(
            Vector3::new(0.0, 0.3, 10.0),
            1.0,
            0.5,
            Vector3::new(239.0, 116.0, 8.0),
            Some(ProceduralTexture::new(42)),
            angle,
        )];

        let mut camera = Camera::new(
            normal_view_pos.clone(),
            Vector3::new(0.0, 0.3, 10.0),
            384,
            216,
            1.57, // fov in radians
        );
        camera.initialize_rays();

        let total_pixels = (camera.width * camera.height) as f64;
        let start_time = Instant::now();
        let mut csv_file = std::fs::File::create("timings.csv").unwrap();
        writeln!(csv_file, "percentage,time_in_seconds").unwrap();

        let mut ray_index = 0;
        let mut last_int_pct = 0u32;
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
                let pct = (ray_index as f64 / total_pixels) * 100.0;
                let int_pct = pct as u32;
                if int_pct > last_int_pct {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    writeln!(csv_file, "{},{}", int_pct, elapsed).unwrap();
                    last_int_pct = int_pct;
                    println!("{}%", int_pct);
                }
            }
        }

        let display = Display::new(camera);
        display.render(&format!("output/{}.png", i));
    }
}
