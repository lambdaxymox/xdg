extern crate xdg;

use std::env;


fn blank_xdg_config() -> xdg::Config {    
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_DIRS", "");
    env::set_var("XDG_CONFIG_DIRS", "");
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_RUNTIME_DIR", "");

    xdg::get_config()
}

#[test]
fn test_get_config_or_default_should_generate_default_values_if_blank() {
    let old_config = xdg::get_config();
    let new_config = blank_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = xdg::default_config();

    assert_eq!(
        result, expected,
        "old config: \n{}\nnew config: \n{}\n", old_config, new_config
    );
}
