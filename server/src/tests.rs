use shared::Channel;

#[test]
fn test_match() {
    assert!(Channel::from("test.test").matches(
        &Channel::from("test.test"),
    ));
    assert!(Channel::from("test.*").matches(&Channel::from("test.test")));
    assert!(Channel::from("*.test").matches(&Channel::from("test.test")));
    assert!(Channel::from("*").matches(&Channel::from("test.test")));

    assert!(!Channel::from("test.test.1").matches(
        &Channel::from("test.test"),
    ));
    assert!(!Channel::from("test.*.1").matches(
        &Channel::from("test.test"),
    ));
    assert!(!Channel::from("*.test.1").matches(
        &Channel::from("test.test"),
    ));
}
