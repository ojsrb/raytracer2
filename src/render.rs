use crate::utils::*;
use image::{Rgb, RgbImage};

pub struct Camera {
    pub position: Vector3,
    pub direction: Vector3,

    pub width: u32,
    pub height: u32,
    pub fov: f64, // in radians

    pub rays: Vec<Ray>,
}

impl Camera {
    pub fn new(position: Vector3, direction: Vector3, width: u32, height: u32, fov: f64) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            width,
            height,
            fov,
            rays: Vec::new(),
        }
    }

    pub fn initialize_rays(&mut self) {
        let radians_per_pixel = self.fov / self.width as f64;
        let w = self.width as f64;
        let h = self.height as f64;
        let half_w = w / 2.0;
        let half_h = h / 2.0;

        self.rays.clear();
        self.rays.reserve((self.width * self.height) as usize);

        for y in 0..self.height {
            let y_centered = (y as f64) - half_h;
            for x in 0..self.width {
                let x_centered = (x as f64) - half_w;
                self.rays.push(Ray::new(
                    self.position,
                    Vector3::new(
                        self.direction.x + (x_centered * radians_per_pixel).asin(),
                        self.direction.y + (y_centered * radians_per_pixel).asin(),
                        self.direction.z,
                    ),
                    0.1,
                ));
            }
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> bool {
        if self.rays.is_empty() {
            return false;
        }
        self.rays[(y * self.width + x) as usize].hit
    }
}

pub struct Display {
    pub camera: Camera,
}

impl Display {
    pub fn new(camera: Camera) -> Self {
        Display { camera }
    }

    pub fn render(&self, filename: &str) {
        let mut image = RgbImage::new(self.camera.width, self.camera.height);
        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                let idx = (y * self.camera.width + x) as usize;
                if self.camera.rays[idx].hit {
                    let ray = &self.camera.rays[idx];
                    image.put_pixel(
                        x,
                        y,
                        Rgb([ray.color.x as u8, ray.color.y as u8, ray.color.z as u8]),
                    );
                } else {
                    image.put_pixel(x, y, Rgb([0, 0, 0]));
                }
            }
        }
        image.save(filename).unwrap();
    }
}
