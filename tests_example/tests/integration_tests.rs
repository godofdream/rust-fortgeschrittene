mod common;
#[test]
fn test_add_and_multiply() {
    // using common code.
    common::setup();

    assert_eq!(tests_example::add(2, 3), 5);
    assert_eq!(tests_example::multiply(2, 3), 6);
}
