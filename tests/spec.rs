use aline::{IVec2, Vec2};
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
