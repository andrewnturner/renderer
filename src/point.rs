use std::mem::swap;
use std::ops::{Add, Sub, Mul};

use num_traits::Num;

use crate::matrix::Matrix44;

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

    pub fn to_homog(self) -> Point4<T> {
        Point4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: T::one(),
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point4<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Num + Copy> Point4<T> {
    pub fn to_affine(self) -> Point3<T> {
        Point3 {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
        }
    }
}

impl<T: Num + Copy> Mul<Matrix44<T>> for Point4<T> {
    type Output = Self;

    fn mul(self, other: Matrix44<T>) -> Self {
        let answer_x = (self.x * other.m[0][0]) + (self.y * other.m[0][1]) + (self.z * other.m[0][2]) + (self.w * other.m[0][3]);
        let answer_y = (self.x * other.m[1][0]) + (self.y * other.m[1][1]) + (self.z * other.m[1][2]) + (self.w * other.m[1][3]);
        let answer_z = (self.x * other.m[2][0]) + (self.y * other.m[2][1]) + (self.z * other.m[2][2]) + (self.w * other.m[2][3]);
        let answer_w = (self.x * other.m[3][0]) + (self.y * other.m[3][1]) + (self.z * other.m[3][2]) + (self.w * other.m[3][3]);
        
        Point4 { x: answer_x, y: answer_y, z: answer_z, w: answer_w }
    }
}
