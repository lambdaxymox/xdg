extern crate xdg;


#[test]
fn test_foo() {
    println!("{:?}", xdg::get_config());
    assert!(false);
}