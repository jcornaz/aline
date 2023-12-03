#![cfg(feature = "serde")]

use aline::IVec2;
use rstest::rstest;

#[test]
fn should_deserialize_expected_form() {
    let v: IVec2 = serde_json::from_str(r#"{ "x": 1, "y": 2 }"#).unwrap();
    assert_eq!(v, IVec2::new(1, 2));
}

#[rstest]
fn should_deserialize_serialized_form(
    #[values(IVec2::ZERO, IVec2::X, IVec2::Y, IVec2::new(1, 2))] v: IVec2,
) {
    let encoded = serde_json::to_string(&v).unwrap();
    let decoded: IVec2 = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, v);
}
