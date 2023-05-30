use c11_test;

mod common;

#[test]
fn integration_adds_two(){
    common::setup();
    assert_eq!(4, c11_test::add_two(2));
}