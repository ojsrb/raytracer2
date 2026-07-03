use crate::utils::*;
use image::{Rgb, RgbImage};

pub struct Camera {
    pub position: Vector3,
    pub direction: Vector3,

    pub width: u32,
    pub height: u32,
    pub fov: f64, // in radians

    pub rays: Vec<Vec<Ray>>,
}

impl Camera {
    pub fn new(position: Vector3, direction: Vector3, width: u32, height: u32, fov: f64) -> Self {
        Self {
            position,
            direction,
            width,
            height,
            fov,
            rays: Vec::new(),
        }
    }

    pub fn initialize_rays(&mut self) {
        let radians_per_pixel = self.fov / self.width as f64;
        self.rays.clear();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                let x_centered = (x as f64) - ((self.width as f64) / 2.0) as f64;
                let y_centered = (y as f64) - ((self.height as f64) / 2.0) as f64;
                row.push(Ray::new(
                    self.position.clone(),
                    Vector3::new(
                        self.direction.x + (x_centered * radians_per_pixel).asin(),
                        self.direction.y + (y_centered * radians_per_pixel).asin(),
                        self.direction.z,
                    ),
                    0.1,
                ));
            }
            self.rays.push(row);
        }
        println!("Rays: {}", self.rays.len());
    }

    pub fn get_ray(&self, x: u32, y: u32) -> bool {
        if self.rays.is_empty() {
            return false;
        }
        self.rays[y as usize][x as usize].hit
    }
}

pub fn get_texture_color(texture: &RgbImage, u: f64, v: f64) -> Vector3 {
    let width = texture.width() as f64;
    let height = texture.height() as f64;

    let x = (u * width) as u32 % texture.width();
    let y = (v * height) as u32 % texture.height();

    let pixel = texture.get_pixel(x, y);
    Vector3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
}

pub struct Display {
    pub camera: Camera,
}

impl Display {
    pub fn new(camera: Camera) -> Self {
        Display { camera }
    }

    pub fn render(&self, filename: &str) {
        let mut image = RgbImage::new(self.camera.width as u32, self.camera.height as u32);
        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                if self.camera.get_ray(x, y) {
                    let ray = &self.camera.rays[y as usize][x as usize];
                    image.put_pixel(
                        x as u32,
                        y as u32,
                        Rgb([ray.color.x as u8, ray.color.y as u8, ray.color.z as u8]),
                    );
                } else {
                    image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
                }
            }
        }
        image.save(filename).unwrap();
    }
}
