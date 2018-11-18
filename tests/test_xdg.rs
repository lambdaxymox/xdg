extern crate xdg;


#[test]
fn test_foo() {
    println!("{}", xdg::get_config());
    println!("{}", xdg::get_config_or_default());
    assert!(false);
}