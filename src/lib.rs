#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2D linear algebra library suitable for `no_std`
//!
//! # Features
//!
//! * `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
//! * `serde`: implementations of `Serialize` and `Deserialize` from [serde](https://docs.rs/serde/1)
//! * `approx_v05`: implementations of [approx v0.5](https://docs.rs/approx/0.5) traits

mod interop {
    #[cfg(feature = "approx_v05")]
    mod approx_v05;
}

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Alias for `Vector<f32>`
pub type Vec2 = Vector2<f32>;

/// Alias for `Vector<i32>`
pub type IVec2 = Vector2<i32>;

/// Vector type, generic over its scalar type `T`
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl Vector2<f32> {
    pub const ZERO: Self = Self::new(0., 0.);
    pub const X: Self = Self::new(1., 0.);
    pub const Y: Self = Self::new(0., 1.);

    /// Cast scalar types to [`i32`]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_i32(self) -> Vector2<i32> {
        Vector2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Vector2<i32> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const X: Self = Self::new(1, 0);
    pub const Y: Self = Self::new(0, 1);

    /// Cast scalar types to [`f32`]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_f32(self) -> Vector2<f32> {
        Vector2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Vector2<T> {
    /// Create a vector where both `x` and `y` are the same `value`
    pub const fn splat(value: T) -> Self {
        Self { x: value, y: value }
    }
}

impl<T> From<[T; 2]> for Vector2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vector2<T>> for [T; 2] {
    fn from(Vector2 { x, y }: Vector2<T>) -> Self {
        [x, y]
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    fn from(Vector2 { x, y }: Vector2<T>) -> Self {
        (x, y)
    }
}

impl<A, B> AddAssign<Vector2<B>> for Vector2<A>
where
    A: AddAssign<B>,
{
    fn add_assign(&mut self, rhs: Vector2<B>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<A, B> Add<Vector2<B>> for Vector2<A>
where
    A: Add<B>,
{
    type Output = Vector2<A::Output>;
    fn add(self, rhs: Vector2<B>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<A, B> SubAssign<Vector2<B>> for Vector2<A>
where
    A: SubAssign<B>,
{
    fn sub_assign(&mut self, rhs: Vector2<B>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<A, B> Sub<Vector2<B>> for Vector2<A>
where
    A: Sub<B>,
{
    type Output = Vector2<A::Output>;
    fn sub(self, rhs: Vector2<B>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<A, B> MulAssign<B> for Vector2<A>
where
    A: MulAssign<B>,
    B: Copy,
{
    fn mul_assign(&mut self, rhs: B) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<A, B> Mul<B> for Vector2<A>
where
    A: Mul<B>,
    B: Copy,
{
    type Output = Vector2<A::Output>;
    fn mul(self, rhs: B) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<A, B> DivAssign<B> for Vector2<A>
where
    A: DivAssign<B>,
    B: Copy,
{
    fn div_assign(&mut self, rhs: B) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<A, B> Div<B> for Vector2<A>
where
    A: Div<B>,
    B: Copy,
{
    type Output = Vector2<A::Output>;
    fn div(self, rhs: B) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
