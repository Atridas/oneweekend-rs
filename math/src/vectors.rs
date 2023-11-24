use crate::{floatops::Float, RandomSource};

use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    // creation utilities

    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn random<RNG: RandomSource<T>>(rng: &mut RNG) -> Self {
        Self {
            x: rng.next(),
            y: rng.next(),
            z: rng.next(),
        }
    }

    pub fn random_range<RNG: RandomSource<T>>(rng: &mut RNG, min: T, max: T) -> Self {
        Self {
            x: rng.next_range(min, max),
            y: rng.next_range(min, max),
            z: rng.next_range(min, max),
        }
    }

    pub fn random_unit_vector<RNG: RandomSource<T>>(rng: &mut RNG) -> Self {
        Vector3::random_in_unit_sphere(rng).unit_vector()
    }

    pub fn random_on_hemisphere<RNG: RandomSource<T>>(rng: &mut RNG, normal: Self) -> Self {
        let on_unit_sphere = Vector3::random_unit_vector(rng);
        if on_unit_sphere.dot(normal) > T::constant(0.0) {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_sphere<RNG: RandomSource<T>>(rng: &mut RNG) -> Self {
        loop {
            let p = Vector3::random(rng);
            if p.length_squared() < T::one() {
                return p;
            }
        }
    }

    // geometric operations

    pub fn dot(&self, rhs: Vector3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(&self) -> T {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> T {
        self.dot(*self)
    }

    pub fn unit_vector(self) -> Vector3<T> {
        self / self.length()
    }

    pub fn cross(&self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn hproduct(&self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    pub fn reflect(&self, normal: Vector3<T>) -> Vector3<T> {
        *self - normal * (self.dot(normal) * T::constant(2.0))
    }

    pub fn refract(&self, n: Vector3<T>, etai_over_etat: T) -> Vector3<T> {
        let cos_theta = -self.dot(n).min(T::constant(1.0));
        let r_out_perp = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(T::constant(1.0) - r_out_perp.length_squared())
            .abs()
            .sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        let s = T::constant(1e-8);
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn is_unit_vector(&self) -> bool {
        let s = T::constant(1e-8);
        (self.length_squared() - T::constant(1.0)).abs() < s
    }
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

// overrides vec[idx]
impl<T, Idx: Into<i64>> Index<Idx> for Vector3<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        match index.into() {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides vec[idx]
impl<T, Idx: Into<i64>> IndexMut<Idx> for Vector3<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        match index.into() {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides point[idx]
impl<T, Idx: Into<i64>> Index<Idx> for Point3<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        match index.into() {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides point[idx]
impl<T, Idx: Into<i64>> IndexMut<Idx> for Point3<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        match index.into() {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides -vec
impl<T: Float> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Vector3<T> {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

// overrides vec1 + vec2
impl<T: Float> Add for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Self) -> Vector3<T> {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// overrides point + vec
impl<T: Float> Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;
    fn add(self, rhs: Vector3<T>) -> Point3<T> {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// overrides vec1 - vec2
impl<T: Float> Sub for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Self) -> Vector3<T> {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// overrides point1 - point2
impl<T: Float> Sub for Point3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Self) -> Vector3<T> {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// overrides point - vec
impl<T: Float> Sub<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;
    fn sub(self, rhs: Vector3<T>) -> Point3<T> {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// overrides vec * s
impl<T: Float> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// overrides vec / s
impl<T: Float> Div<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Vector3<T> {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// overrides vec1 += vec2
impl<T: Float> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// overrides vec += point
impl<T: Float> AddAssign<Vector3<T>> for Point3<T> {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// overrides vec *= s
impl<T: Float> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// overrides vec /= s
impl<T: Float> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T> From<Point3<T>> for Vector3<T> {
    fn from(value: Point3<T>) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
