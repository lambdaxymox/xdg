extern crate xdg;

use std::env;


fn clear_xdg_config() -> xdg::Config {
    let old_config = xdg::get_config();
    
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_DIRS", "");
    env::set_var("XDG_CONFIG_DIRS", "");
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_RUNTIME_DIR", "");

    old_config
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

fn set_var_opt(key: &str, val: &Option<String>) {
    let unwrapped = match val {
        &Some(ref st) => st.clone(),
        &None => String::from(""),
    };

    env::set_var(key, unwrapped);
}

fn reset_xdg_config(old_config: &xdg::Config) {
    set_var_opt("XDG_DATA_HOME", &old_config.xdg_data_home);
    set_var_opt("XDG_CONFIG_HOME", &old_config.xdg_config_home);
    set_var_opt("XDG_DATA_DIRS", &old_config.xdg_data_dirs);
    set_var_opt("XDG_CONFIG_DIRS", &old_config.xdg_config_dirs);
    set_var_opt("XDG_CACHE_HOME", &old_config.xdg_cache_home);
    set_var_opt("XDG_RUNTIME_DIR", &old_config.xdg_runtime_dir);
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
    let old_config = clear_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = default_config();
    reset_xdg_config(&old_config);
    
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
