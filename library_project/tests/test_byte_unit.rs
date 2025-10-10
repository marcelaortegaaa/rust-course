use library_project::byte_unit::ByteUnit;

#[test] // Fails
fn test_from_str_whitespace() {
    let x: &str = "    ";
    let result = ByteUnit::from_str(x);
    assert_ne!(result, None, "{:#?} should be Bytes", result)
}

#[test] // Passes
fn test_from_str_weirdunit() {
    let x: &str = "kilometres";
    let result = ByteUnit::from_str(x);
    assert_eq!(result, None, "{:#?} should be None", result)
}
