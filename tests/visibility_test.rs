#[test]
fn test_types_are_public() {
    // had an issue where the Error type was not accessible outside the crate
    const TEST_MESSAGE: &str = "test message";
    let err = dlprotoc::Error::from_string(TEST_MESSAGE.to_string());
    assert_eq!(TEST_MESSAGE, err.to_string());
}
