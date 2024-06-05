use aline::{IVec2, Vec2};
use approx_v05::assert_abs_diff_eq;
use rstest::rstest;

#[rstest]
#[case(IVec2::ZERO, IVec2::ZERO, IVec2::ZERO)]
#[case(IVec2::ZERO, IVec2::X, IVec2::X)]
#[case((1, 2), [3, 4], IVec2::new(4, 6))]
#[case(IVec2::splat(1), IVec2::splat(2), IVec2::splat(3))]
fn test_add_sub(
    #[case] a: impl Into<IVec2>,
    #[case] b: impl Into<IVec2>,
    #[case] sum: impl Into<IVec2>,
) {
    let a: IVec2 = a.into();
    let b: IVec2 = b.into();
    let expected_sum: IVec2 = sum.into();
    assert_eq!(a + b, expected_sum);
    assert_eq!(expected_sum - a, b);
    assert_eq!(expected_sum - b, a);
    let mut actual_sum = a;
    actual_sum += b;
    assert_eq!(actual_sum, expected_sum);
    let mut rest = expected_sum;
    rest -= a;
    assert_eq!(rest, b);
}

#[rstest]
#[case(IVec2::ZERO, 0, IVec2::ZERO)]
#[case((3, 4), 0, (0, 0))]
#[case((3, 4), 1, (3, 4))]
#[case((3, 4), 2, (6, 8))]
fn test_mul(
    #[case] a: impl Into<IVec2>,
    #[case] b: i32,
    #[case] expected_product: impl Into<IVec2>,
) {
    let a = a.into();
    let expected_product = expected_product.into();
    assert_eq!(a * b, expected_product);
    let mut actual_product = a;
    actual_product *= b;
    assert_eq!(actual_product, expected_product);
}

#[rstest]
#[case(IVec2::ZERO, 1, IVec2::ZERO)]
#[case((3, 4), 1, (3, 4))]
#[case((4, 6), 2, (2, 3))]
fn test_div(#[case] a: impl Into<IVec2>, #[case] b: i32, #[case] expected_ratio: impl Into<IVec2>) {
    let a = a.into();
    let expected_ratio = expected_ratio.into();
    assert_eq!(a / b, expected_ratio);
    let mut actual_ratio = a;
    actual_ratio /= b;
    assert_eq!(actual_ratio, expected_ratio);
}

#[rstest]
#[case([1, 2], (1., 2.))]
fn test_scalar_cast(#[case] integer: impl Into<IVec2>, #[case] float: impl Into<Vec2>) {
    let integer = integer.into();
    let float = float.into();
    assert_eq!(float.as_i32(), integer);
    #[cfg(feature = "approx_v05")]
    assert_eq!(integer.as_f32(), float);
}

#[rstest]
#[case(IVec2::ZERO, IVec2::ZERO)]
#[case([1, -2], [-1, 2])]
#[case(IVec2::X, -IVec2::X)]
#[case(IVec2::Y, -IVec2::Y)]
fn test_neg(#[case] vector: impl Into<IVec2>, #[case] expected: impl Into<IVec2>) {
    let vector = vector.into();
    assert_eq!(-vector, expected.into());
    assert_eq!(-(-vector), vector);
}

#[rstest]
#[case(IVec2::ZERO, IVec2::ZERO, 0)]
#[case(IVec2::X, IVec2::ZERO, 0)]
#[case(IVec2::X, IVec2::X, 1)]
#[case(IVec2::X, IVec2::new(1, 1), 1)]
#[case(IVec2::X, IVec2::Y, 0)]
#[case(IVec2::X, -IVec2::X, -1)]
#[case(IVec2::new(2, 3), IVec2::new(4, 5), 23)]
fn test_dot_product(#[case] v1: IVec2, #[case] v2: IVec2, #[case] expected: i32) {
    assert_eq!(v1.dot(v2), expected);
}

#[rstest]
#[case(Vec2::ZERO, 0.0)]
#[case(Vec2::X, 1.0)]
#[case(Vec2::Y, 1.0)]
#[case(Vec2::Y * 2., 2.0)]
#[case(Vec2::new(3., 4.), 5.)]
fn test_magnitude(#[case] vector: Vec2, #[case] expected: f32) {
    assert_abs_diff_eq!(vector.magnitude_squared(), expected * expected);
    #[cfg(any(feature = "std", feature = "libm"))]
    assert_abs_diff_eq!(vector.magnitude(), expected);
}

#[rstest]
#[cfg(any(feature = "std", feature = "libm"))]
fn test_normalize_zero_vec() {
    assert_eq!(Vec2::ZERO.normalize(), None);
}

#[rstest]
#[cfg(any(feature = "std", feature = "libm"))]
fn test_normalize_normal(#[values(Vec2::X, Vec2::Y, -Vec2::X, -Vec2::Y)] vector: Vec2) {
    let normalized = vector.normalize().unwrap();
    assert_abs_diff_eq!(normalized.x, vector.x);
    assert_abs_diff_eq!(normalized.y, vector.y);
}

#[rstest]
#[cfg(any(feature = "std", feature = "libm"))]
fn test_normalize() {
    let normalized = Vec2::new(3., 4.).normalize().unwrap();
    assert_abs_diff_eq!(normalized.x, 3. / 5.);
    assert_abs_diff_eq!(normalized.y, 4. / 5.);
    assert_abs_diff_eq!(normalized.magnitude(), 1.0);
}
