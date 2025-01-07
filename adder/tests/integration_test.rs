use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}

// using 'cargo test --test integration_test' will only run this file and
// ignore other integration test files
