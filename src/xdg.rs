use std::env;
use std::ffi::OsStr;


#[derive(Clone, Debug)]
pub struct Config {
    pub data_home: Option<String>,
    pub config_home: Option<String>,
    pub data_dirs: Option<String>,
    pub config_dirs: Option<String>,
    pub cache_home: Option<String>,
    pub runtime_dir: Option<String>,
}

impl Config {
    pub fn new(
        xdg_data_home: Option<String>, xdg_config_home: Option<String>, 
        xdg_data_dirs: Option<String>, xdg_config_dirs: Option<String>, 
        xdg_cache_home: Option<String>, xdg_runtime_dir: Option<String>) -> Config {
        
        Config {
            data_home: xdg_data_home,
            config_home: xdg_config_home,
            data_dirs: xdg_data_dirs,
            config_dirs: xdg_config_dirs,
            cache_home: xdg_cache_home,
            runtime_dir: xdg_runtime_dir,
        }
    }
}

fn get_env<K: AsRef<OsStr>>(key: K) -> Option<String> {    
    env::var(key).ok()
}

pub fn get_config() -> Config {
    let xdg_data_home = get_env("XDG_DATA_HOME");
    let xdg_config_home = get_env("XDG_CONFIG_HOME");
    let xdg_data_dirs = get_env("XDG_DATA_DIRS");
    let xdg_config_dirs = get_env("XDG_CONFIG_DIRS");
    let xdg_cache_home = get_env("XDG_CACHE_HOME");
    let xdg_runtime_dir = get_env("XDG_RUNTIME_DIR");

    Config::new(
        xdg_data_home, xdg_config_home, xdg_data_dirs,
        xdg_config_dirs, xdg_cache_home, xdg_runtime_dir
    )
}
