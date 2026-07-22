mod render;
mod utils;
use rayon::prelude::*;
use render::*;
use std::io::{SeekFrom::Start, Write};
use utils::*;

fn advance_ray(ray: &mut Ray, scene: &[BlackHole]) -> bool {
    for black_hole in scene {
        let dir_to_mass = black_hole.position - ray.position;

        if ray.position.z > 20.0 {
            return true;
        }

        // Use the same gravity model the integrator uses, so the predicted
        // crossing point matches the actual trajectory.
        let accel = black_hole.acceleration(&dir_to_mass);
        let new_direction = (accel + ray.direction).normalize();

        let plane_y = black_hole.position.y;
        let full_step = ray.speed;
        let dy = new_direction.y * full_step;
        let crosses_plane = (ray.position.y - plane_y) * (ray.position.y + dy - plane_y) < 0.0;

        let d_pos = new_direction * full_step;

        let crosses_horizon =
            (ray.position + d_pos - black_hole.position).length() < black_hole.min_distance;

        let n_substeps = if crosses_plane || crosses_horizon {
            8
        } else {
            1
        };
        let sub_step = full_step / n_substeps as f64;

        for _ in 0..n_substeps {
            let next_y = ray.position.y + new_direction.y * sub_step;
            let crossed = (ray.position.y - plane_y) * (next_y - plane_y) <= 0.0
                && (ray.position.y - plane_y).abs() > 1e-9;

            let in_horizon =
                (ray.position - black_hole.position).length() < black_hole.min_distance;

            if crossed {
                let t = (plane_y - ray.position.y) / (new_direction.y * sub_step);
                let t = t.clamp(0.0, 1.0);
                let hit_pos = ray.position + new_direction * (t * sub_step);

                let hit_dir_to_mass = black_hole.position - hit_pos;
                let hit_dist = hit_dir_to_mass.length();
                if hit_dist < black_hole.acretion_disk_r {
                    ray.position = hit_pos;
                    ray.hit = true;

                    let angle = black_hole.angle;

                    let mut shifted_position = ray.position - black_hole.position;
                    shifted_position.y = black_hole.position.y;

                    let r = shifted_position.length();
                    let theta_raw = shifted_position.x.atan2(shifted_position.z);

                    let dist_brightness = black_hole.acretion_disk_r / hit_dist;

                    ray.brightness = black_hole
                        .texture
                        .as_ref()
                        .unwrap()
                        .sample(r, theta_raw, angle)
                        * 0.8
                        + dist_brightness.powf(1.5)
                        - 1.2;
                    ray.color = black_hole.color.brighten(ray.brightness);
                    return true;
                }
            } else if in_horizon {
                ray.position = ray.position + new_direction * sub_step;
                ray.hit = true;
                ray.brightness = 0.0;
                ray.color = Vector3::new(0.0, 0.0, 0.0);
                return true;
            }

            ray.position = ray.position + new_direction * sub_step;
        }

        ray.direction = new_direction;
    }
    false
}

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let normal_view_pos = Vector3::new(0.0, -1.0, 0.0);

    let count = args[1].parse::<u32>().unwrap_or(1);
    for i in 0..count {
        let angle = (i as f64) * (6.28 / count as f64);
        if count > 1 {
            println!(
                "Rendering item {} of {}: angle {} radians",
                i + 1,
                count,
                angle
            );
        }
        let scene = vec![BlackHole::new(
            Vector3::new(0.0, 0.3, 10.0),
            1.0,
            0.5,
            Vector3::new(239.0, 116.0, 8.0).brighten(0.15),
            Some(ProceduralTexture::new(42)),
            angle,
        )];

        let width: u32;
        let height: u32;

        let start_time = std::time::Instant::now();

        if args.len() < 4 {
            width = 384;
            height = 216;
        } else {
            width = args[2].parse::<u32>().unwrap_or(384);
            height = args[3].parse::<u32>().unwrap_or(216);
        }

        let mut camera = Camera::new(
            normal_view_pos,
            Vector3::new(0.0, 0.5, 6.0),
            width,
            height,
            1.57, // fov in radians
        );
        camera.initialize_rays();

        let total_pixels = (camera.width * camera.height) as f64;

        // Process rays in parallel using rayon.
        let completed = std::sync::atomic::AtomicU32::new(0);
        let last_reported = std::sync::atomic::AtomicU32::new(0);

        camera.rays.par_iter_mut().for_each(|ray| {
            let mut frames = 0;
            loop {
                let result = advance_ray(ray, &scene);
                frames += 1;
                if result || frames > 1000 {
                    break;
                }
            }
            let done = completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            let int_pct = ((done as f64 / total_pixels) * 100.0) as u32;
            let prev = last_reported.load(std::sync::atomic::Ordering::Relaxed);
            if int_pct > prev && int_pct % 10 == 0 {
                if last_reported
                    .compare_exchange(
                        prev,
                        int_pct,
                        std::sync::atomic::Ordering::Relaxed,
                        std::sync::atomic::Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    if count <= 1 {
                        println!("{}%", int_pct);
                    }
                }
            }
        });

        let elapsed = start_time.elapsed().as_secs_f64();
        println!("Done in {:.2}s", elapsed);

        let display = Display::new(camera);
        display.render(&format!("output/{}.png", i));
    }
}
