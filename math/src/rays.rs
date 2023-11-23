use std::ops::{Add, Mul};

use super::Point3;
use super::Vector3;

#[derive(Clone, Copy)]
pub struct Ray<T> {
    orig: Point3<T>,
    dir: Vector3<T>,
}

impl<T> Ray<T>
where
    T: Copy,
{
    pub fn new(origin: Point3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Point3<T> {
        self.orig
    }
    pub fn direction(&self) -> Vector3<T> {
        self.dir
    }
}

impl<T> Ray<T>
where
    T: Copy,
    Vector3<T>: Mul<T, Output = Vector3<T>>,
    Point3<T>: Add<Vector3<T>, Output = Point3<T>>,
{
    pub fn at(&self, distance: T) -> Point3<T> {
        self.orig + self.dir * distance
    }
}
