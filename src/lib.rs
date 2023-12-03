#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2d linear algebra library suitable for `no_std`
//!
//! # Features
//!
//! `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
