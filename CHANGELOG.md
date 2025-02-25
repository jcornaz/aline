# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [1.2.0] - 2024-12-11

### Added

* `UVec2` alias for `Vector2<usize>`
* implement `TryFrom<IVec2>` for `UVec2` and `TryFrom<UVec2>` for `IVec2`



## [1.1.0] - 2024-12-08


### Added

* `Vector2::<T>::perp` where `T: Neg` which returns the vector rotated by 90°


## [1.0.1] - 2024-12-07

### Dependencies

* Relax dependencies versions requirement
* Lower MSRV to 1.61


## [1.0.0] - 2024-09-09

### Documentation

* Typos fixed


## [0.1.4] - 2024-06-05


### Added

* `Vec2::normalize`


## [0.1.3] - 2023-12-09


### Added

* Implement `Neg` for `Vector2<T>` where `T: Neg`
* `dot` product
* `magnitude_squared` and `magnitude` (the latter requires either `std` or `libm`)


## [0.1.2] - 2023-12-09

### Added

* `Vector2::<T>::splat` where `T: Copy` to easily create vector with the same value for `x` and `y`


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


[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/jcornaz/beancount_parser_2/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/jcornaz/beancount_parser_2/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.4...v1.0.0
[0.1.4]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/aline/compare/...v0.1.0
