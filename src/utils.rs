use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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
    pub fn reciprocal(&self) -> Self {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    pub fn invert(&self) -> Self {
        Self::new(255.0 - self.x, 255.0 - self.y, 255.0 - self.z)
    }

    pub fn brighten(&self, factor: f64) -> Self {
        Self::new(
            (self.x * factor).clamp(0.0, 255.0),
            (self.y * factor).clamp(0.0, 255.0),
            (self.z * factor).clamp(0.0, 255.0),
        )
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

#[derive(Clone)]
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
        Self {
            position,
            direction,
            speed,
            hit: false,
            color: Vector3::new(0.0, 0.0, 0.0),
            brightness: 0.0,
        }
    }
}

use noise::{Fbm, NoiseFn, Perlin};

#[derive(Clone)]
pub struct ProceduralTexture {
    fbm: Fbm<Perlin>,
}

impl ProceduralTexture {
    pub fn new(seed: u32) -> Self {
        Self {
            fbm: Fbm::<Perlin>::new(seed),
        }
    }

    /// Sample procedural noise at polar coordinates.
    /// `t_raw` is the raw atan2 angle (in radians, -π..π).
    /// `angle` is the black hole rotation offset.
    pub fn sample(&self, r: f64, t_raw: f64, angle: f64) -> f64 {
        // Apply 4x angular frequency inside sin/cos so the branch cut
        // of atan2 is invisible — sin(4θ) is continuous across the cut.
        let theta = t_raw + angle;
        let cart_u = r * theta.cos();
        let cart_v = r * theta.sin();

        let noise_value = self.fbm.get([cart_u, cart_v]);

        // Map from [-1, 1] to [0, 1]
        let t = ((noise_value + 1.0) / 2.0).clamp(0.0, 0.6);

        return t;
    }
}

pub struct BlackHole {
    pub position: Vector3,
    pub mass: f64,
    pub min_distance: f64,

    pub acretion_disk_r: f64,
    pub color: Vector3,

    pub texture: Option<ProceduralTexture>,

    pub angle: f64,
}

impl BlackHole {
    pub fn new(
        position: Vector3,
        mass: f64,
        min_distance: f64,
        color: Vector3,
        texture: Option<ProceduralTexture>,
        angle: f64,
    ) -> Self {
        let acretion_disk_r = mass * 5.0;
        Self {
            position,
            mass,
            min_distance,
            acretion_disk_r,
            color,
            texture,
            angle,
        }
    }

    /// Gravitational acceleration term added to a ray's direction per step.
    /// This is the single source of truth for the gravity model used both
    /// for ray bending and for disc-crossing detection.
    pub fn acceleration(&self, dir_to_mass: &Vector3) -> Vector3 {
        let len = dir_to_mass.length();
        (dir_to_mass.normalize() * self.mass * 0.03) / len
    }
}
