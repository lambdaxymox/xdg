extern crate xdg;

use std::env;


fn no_xdg_runtime_dir_config() {
    env::remove_var("XDG_RUNTIME_DIR");
}

#[test]
fn test_get_config_or_default_should_not_generate_missing_xdg_runtime_dir() {
    let old_config = xdg::get_config();
    no_xdg_runtime_dir_config();
    let new_config = xdg::get_config_or_default();
    let result = new_config.xdg_runtime_dir.clone();
    let expected = xdg::Value::NotPresent;

    assert_eq!(
        result, expected,
        "old config: \n{}\nnew config: \n{}\n", old_config, new_config
    );
}
