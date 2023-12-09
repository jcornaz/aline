# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Added

* `Vector::<T>::splat` where `T: Copy` to easily create vector with the same value for `x` and `y`


## [0.1.1] - 2023-12-03

### Added

* implement `Default` for `Vector<T>`
* `IVec2::as_f32` and `Vec2::as_i32` methods to cast from one scalar type to the other


### Documentation

* Better document required feature flags on docs.rs
* Fix formatting of the cargo feature list on docs.rs
* Fix typos in readme


## [0.1.0] - 2023-12-03

* `Vector2<T>` types with `Vec2` and `IVec2` aliases
* Operators `+`, `+=`, `-`, `-=`, `*`, `*=`, `/` and `/=`
* Support `no_std` (by disabling the `std` feature flag)
* `approx_v05` feature: Implementations of [approx v0.5](https://docs.rs/approx/0.5) traits
* `serde` feature: Implementations of `Serialize` and `Deserialize` from [serde](https://docs.rs/serde/1)


[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/aline/compare/...v0.1.0
