use aline::IVec2;
use rstest::rstest;

#[rstest]
#[case(IVec2::ZERO, IVec2::ZERO, IVec2::ZERO)]
#[case(IVec2::ZERO, IVec2::X, IVec2::X)]
#[case((1, 2), [3, 4], IVec2::new(4, 6))]
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
