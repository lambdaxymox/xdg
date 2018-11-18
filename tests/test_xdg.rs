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

fn default_config() -> xdg::Config {
    let home = env::var("HOME").unwrap();

    xdg::Config::new(
        Some(format!("{}/{}", home, ".local/share")),
        Some(format!("{}/{}", home, ".config")),
        Some(format!("{}", "/usr/local/share/:/usr/share/")),
        Some(format!("{}", "/etc/xdg")),
        Some(format!("{}/{}", home, ".cache")),
        Some(format!("{}", ""))
    )
}

#[test]
fn test_get_config_or_default_should_generate_missing_values() {
    clear_xdg_config();
    let result = xdg::get_config_or_default();
    let expected = default_config();
    
    assert_eq!(result, expected);
}