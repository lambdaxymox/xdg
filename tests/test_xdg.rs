extern crate xdg;

use std::env;


fn clear_xdg_config() -> xdg::Config {    
    env::set_var("XDG_DATA_HOME", "");
    env::set_var("XDG_CONFIG_HOME", "");
    env::set_var("XDG_DATA_DIRS", "");
    env::set_var("XDG_CONFIG_DIRS", "");
    env::set_var("XDG_CACHE_HOME", "");
    env::set_var("XDG_RUNTIME_DIR", "");

    xdg::get_config()
}

fn empty_xdg_config() -> xdg::Config {
    env::remove_var("XDG_DATA_HOME");
    env::remove_var("XDG_CONFIG_HOME");
    env::remove_var("XDG_DATA_DIRS");
    env::remove_var("XDG_CONFIG_DIRS");
    env::remove_var("XDG_CACHE_HOME");
    env::remove_var("XDG_RUNTIME_DIR");

    xdg::get_config()
}

fn no_xdg_runtime_dir_config() -> xdg::Config {
    env::remove_var("XDG_RUNTIME_DIR");

    xdg::get_config()
}

fn set_var_opt(key: &str, val: &xdg::Value) {
    let unwrapped = match val {
        &xdg::Value::Occupied(ref st) => st.clone(),
        _ => String::from(""),
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
        xdg::Value::Occupied(format!("{}/{}", home, ".local/share")),
        xdg::Value::Occupied(format!("{}/{}", home, ".config")),
        xdg::Value::Occupied(format!("{}", "/usr/local/share/:/usr/share/")),
        xdg::Value::Occupied(format!("{}", "/etc/xdg")),
        xdg::Value::Occupied(format!("{}/{}", home, ".cache")),
        xdg::Value::NotPresent,
    )
}

#[ignore]
#[test]
fn test_get_config_or_default_should_generate_default_values_if_blank() {
    let old_config = xdg::get_config();
    let new_config = clear_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = default_config();
    reset_xdg_config(&old_config);

    assert_eq!(
        result, expected,
        "old config: \n{}\nnew config: \n{}\n", old_config, new_config
    );
}

#[ignore]
#[test]
fn test_get_config_or_default_should_fill_in_missing_keys() {
    let old_config = xdg::get_config();
    let new_config = empty_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = default_config();
    reset_xdg_config(&old_config);

    assert_eq!(
        result, expected,
        "old config: \n{}\nnew config: \n{}\n", old_config, new_config
    );
}

#[test]
fn test_get_config_or_default_should_not_generate_missing_xdg_runtime_dir() {
    let old_config = xdg::get_config();
    println!("XDG_RUNTIME_DIR={:?}", env::var("XDG_RUNTIME_DIR"));
    env::remove_var("XDG_RUNTIME_DIR");
    println!("XDG_RUNTIME_DIR={:?}", env::var("XDG_RUNTIME_DIR"));
    let new_config = no_xdg_runtime_dir_config();
    let config = xdg::get_config_or_default();
    let result = config.xdg_runtime_dir;
    let expected = xdg::Value::NotPresent;
    reset_xdg_config(&old_config);

    assert_eq!(
        result, expected,
        "old config: \n{}\nnew config: \n{}\n", old_config, new_config
    );
}
