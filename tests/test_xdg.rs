extern crate xdg;

use std::env;


fn clear_xdg_config() {
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_DIRS", "");
    env::set_var("XDG_CONFIG_DIRS", "");
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_RUNTIME_DIR", "");
}

fn empty_xdg_config() {
    env::remove_var("XDG_DATA_HOME");
    env::remove_var("XDG_CONFIG_HOME");
    env::remove_var("XDG_DATA_DIRS");
    env::remove_var("XDG_CONFIG_DIRS");
    env::remove_var("XDG_CACHE_HOME");
    env::remove_var("XDG_RUNTIME_DIR");
}

fn no_xdg_runtime_dir_config() {
    env::remove_var("XDG_RUNTIME_DIR");
}

fn default_config() -> xdg::Config {
    let home = env::var("HOME").unwrap();

    xdg::Config::new(
        Some(format!("{}/{}", home, ".local/share")),
        Some(format!("{}/{}", home, ".config")),
        Some(format!("{}", "/usr/local/share/:/usr/share/")),
        Some(format!("{}", "/etc/xdg")),
        Some(format!("{}/{}", home, ".cache")),
        None
    )
}

#[test]
fn test_get_config_or_default_should_generate_default_values_if_blank() {
    clear_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = default_config();
    
    assert_eq!(result, expected);
}

#[test]
fn test_get_config_or_default_should_fill_in_missing_keys() {
    empty_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = default_config();

    assert_eq!(result, expected);
}

#[test]
fn test_get_config_or_default_should_not_generate_missing_xdg_runtime_dir() {
    no_xdg_runtime_dir_config();
    let config = xdg::get_config_or_default();
    let result = config.xdg_runtime_dir;
    let expected = None;

    assert_eq!(result, expected);
}
