#![cfg(feature = "approx_v05")]

use aline::Vec2;
use approx_v05::{
    assert_abs_diff_eq, assert_abs_diff_ne, assert_relative_eq, assert_relative_ne, assert_ulps_eq,
    assert_ulps_ne,
};
use rstest::rstest;

#[rstest]
#[case(Vec2::ZERO, Vec2::ZERO)]
#[case((1.2, 3.4), (1.2, 3.4))]
fn should_equal(#[case] a: impl Into<Vec2>, #[case] b: impl Into<Vec2>) {
    let a = a.into();
    let b = b.into();
    assert_abs_diff_eq!(a, b);
    assert_relative_eq!(a, b);
    assert_ulps_eq!(a, b);
}

#[rstest]
#[case(Vec2::ZERO, Vec2::X)]
#[case(Vec2::X, Vec2::Y)]
#[case((1.2, 3.4), (3.4, 1.2))]
fn should_not_equal(#[case] a: impl Into<Vec2>, #[case] b: impl Into<Vec2>) {
    let a = a.into();
    let b = b.into();
    assert_abs_diff_ne!(a, b);
    assert_relative_ne!(a, b);
    assert_ulps_ne!(a, b);
}
