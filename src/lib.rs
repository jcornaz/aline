#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2d linear algebra library suitable for `no_std`
//!
//! # Features
//!
//! `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.

/// Alias for `Vector<f32>`
pub type Vec2 = Vector2<f32>;

/// Alias for `Vector<i32>`
pub type IVec2 = Vector2<i32>;

/// Vector type, generic over its scalar type `T`
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
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

impl Vector2<f32> {
    pub const ZERO: Self = Self::new(0., 0.);
    pub const X: Self = Self::new(1., 0.);
    pub const Y: Self = Self::new(0., 1.);
}

impl Vector2<i32> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const X: Self = Self::new(1, 0);
    pub const Y: Self = Self::new(0, 1);
}
