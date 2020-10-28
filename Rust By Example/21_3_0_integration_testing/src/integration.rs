extern crate integration_testing;

mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(integration_testing::add(3, 2), 5);
}