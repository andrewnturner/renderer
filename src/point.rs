use std::mem::swap;
use std::ops::{Add, Sub, Mul};

use num_traits::Num;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2 {
            x: x,
            y: y,
        }
    }

    pub fn transpose(&mut self) {
        swap(&mut self.x, &mut self.y);
    } 
}

impl Point2<i32> {
    pub fn to_f32(self) -> Point2<f32> {
        Point2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl<T: Num> Add for Point2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T: Num> Sub for Point2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<T: Num + Copy> Mul<T> for Point2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn length_squared(&self) -> T {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }
}

impl Point3<f32> {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalise(&self) -> Self {
        let length = self.length();

        Point3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

impl<T: Num> Sub for Point3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}
