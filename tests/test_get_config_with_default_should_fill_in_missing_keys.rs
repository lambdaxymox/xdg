extern crate xdg;

use std::env;


fn empty_xdg_config() {
    env::remove_var("XDG_DATA_HOME");
    env::remove_var("XDG_CONFIG_HOME");
    env::remove_var("XDG_DATA_DIRS");
    env::remove_var("XDG_CONFIG_DIRS");
    env::remove_var("XDG_CACHE_HOME");
    env::remove_var("XDG_RUNTIME_DIR");
}


#[test]
fn test_get_config_or_default_should_fill_in_missing_keys() {
    empty_xdg_config();
    let result = xdg::get_config_with_defaults();
    let expected = xdg::default_config();

    assert_eq!(
        result, expected,
        "old config: \n{}\nnew config: \n{}\n", result, expected
    );
}
