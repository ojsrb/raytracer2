use std::ops::{Add, Sub, Mul, Div};

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length == 0.0 {
            Self::new(0.0, 0.0, 0.0)
        } else {
            Self::new(self.x / length, self.y / length, self.z / length)
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
    pub speed: f64,
    pub hit: bool,
    pub color: Vector3,
    pub brightness: f64,
}

impl Ray {
    pub fn new(position: Vector3, direction: Vector3, speed: f64) -> Self {
        Self { position, direction, speed, hit: false, color: Vector3::new(0.0, 0.0, 0.0), brightness: 0.0 }
    }

    pub fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            direction: self.direction.clone(),
            speed: self.speed,
            hit: self.hit,
            color: self.color.clone(),
            brightness: self.brightness
        }
    }
}

pub struct BlackHole {
    pub position: Vector3,
    pub mass: f64,
    pub min_distance: f64,

    pub acretion_disk_r: f64,
    pub color: Vector3,
}

impl BlackHole {
    pub fn new(position: Vector3, mass: f64, min_distance: f64, acretion_disk_r: f64, color: Vector3) -> Self {
        Self { position, mass, min_distance, acretion_disk_r, color }
    }

    pub fn intersects_with_disc(&self, ray: &Ray) -> bool {
        let dir_to_mass = self.position.clone() - ray.position.clone();
        let mut new_direction = (dir_to_mass.clone() * self.mass * 0.0001) + ray.direction.clone();
        new_direction = new_direction.normalize();

        let next_position = ray.position.clone() + new_direction * ray.speed;
        if (ray.position.y > self.position.y && next_position.y < self.position.y) || (ray.position.y < self.position.y && next_position.y > self.position.y || (ray.position.y - self.position.y).abs() < 0.005) {
            return dir_to_mass.length() < self.acretion_disk_r;
        }
        return false;
    }

}