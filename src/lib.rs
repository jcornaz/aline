#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2D linear algebra library suitable for `no_std`
//!
//! # Features
//!
//! * `std`: *(enabled by default)* enable use of the standard library. It must be disabled for `no_std` crates.
//! * `libm`: use [libm](https://crates.io/crates/libm) as an alternative core math library. This is required for some methods (i.e. `magnitude`) to be available on `no_std`
//! * `serde`: implementations of `Serialize` and `Deserialize` from [serde](https://docs.rs/serde/1)
//! * `approx_v05`: implementations of [approx v0.5](https://docs.rs/approx/0.5) traits

pub use aline_v1::*;
