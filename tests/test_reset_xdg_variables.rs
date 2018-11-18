extern crate xdg;

use std::env;


fn reset_var(key: &str, value: &xdg::Value) {
    match value {
        xdg::Value::Occupied(st) => {
            env::set_var(key, st)
        }
        xdg::Value::NotPresent => {
            env::remove_var(key);
        }
        xdg::Value::Empty => {
            env::set_var(key, "");
        }
    }
}

fn reset_xdg_config(old_config: &xdg::Config) {
    reset_var("XDG_DATA_HOME", &old_config.xdg_data_home);
    reset_var("XDG_CONFIG_HOME", &old_config.xdg_config_home);
    reset_var("XDG_DATA_DIRS", &old_config.xdg_data_dirs);
    reset_var("XDG_CONFIG_DIRS", &old_config.xdg_config_dirs);
    reset_var("XDG_CACHE_HOME", &old_config.xdg_cache_home);
    reset_var("XDG_RUNTIME_DIR", &old_config.xdg_runtime_dir);
}

fn clear_xdg_config() {    
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_DIRS", "");
    env::set_var("XDG_CONFIG_DIRS", "");
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_RUNTIME_DIR", "");
}

#[test]
fn test_reset_xdg_config_returns_to_old_values() {
    let expected = xdg::get_config();
    clear_xdg_config();
    reset_xdg_config(&expected);
    let result = xdg::get_config();

    assert_eq!(result, expected);
}
