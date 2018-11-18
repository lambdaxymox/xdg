use std::env;
use std::ffi::OsStr;
use std::fmt;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub xdg_data_home: Option<String>,
    pub xdg_config_home: Option<String>,
    pub xdg_data_dirs: Option<String>,
    pub xdg_config_dirs: Option<String>,
    pub xdg_cache_home: Option<String>,
    pub xdg_runtime_dir: Option<String>,
}

impl Config {
    pub fn new(
        xdg_data_home: Option<String>, xdg_config_home: Option<String>, 
        xdg_data_dirs: Option<String>, xdg_config_dirs: Option<String>, 
        xdg_cache_home: Option<String>, xdg_runtime_dir: Option<String>) -> Config {
        
        Config {
            xdg_data_home: xdg_data_home,
            xdg_config_home: xdg_config_home,
            xdg_data_dirs: xdg_data_dirs,
            xdg_config_dirs: xdg_config_dirs,
            xdg_cache_home: xdg_cache_home,
            xdg_runtime_dir: xdg_runtime_dir,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "XDG_DATA_HOME={}", 
            self.xdg_data_home.as_ref().unwrap_or(&"".to_string())
        ).unwrap();
        writeln!(f, "XDG_CONFIG_HOME={}", 
            self.xdg_config_home.as_ref().unwrap_or(&"".to_string())
        ).unwrap();
        writeln!(f, "XDG_DATA_DIRS={}",
            self.xdg_data_dirs.as_ref().unwrap_or(&"".to_string())
        ).unwrap();
        writeln!(f, "XDG_CONFIG_DIRS={}", 
            self.xdg_config_dirs.as_ref().unwrap_or(&"".to_string())
        ).unwrap();
        writeln!(f, "XDG_CACHE_HOME={}",
            self.xdg_cache_home.as_ref().unwrap_or(&"".to_string())
        ).unwrap();
        writeln!(f, "XDG_RUNTIME_DIR={}",
            self.xdg_runtime_dir.as_ref().unwrap_or(&"".to_string())
        )
    }
}

fn get_env<K: AsRef<OsStr>>(key: K) -> Option<String> {    
    match env::var(key) {
        Ok(ref st) if st != "" => Some(st.clone()),
        _ => None,
    }
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

fn get_env_or_default<K: AsRef<OsStr>>(key: K, default: &str) -> String {
    let var = match env::var(key) {
        Ok(ref st) if st == "" => default.into(),
        Err(_) => default.into(),
        Ok(st) => st,
    };

    var
}

pub fn get_config_or_default() -> Config {
    let home = get_env("HOME").unwrap();
    
    let xdg_data_home_default = format!("{}/{}", home, ".local/share");
    let xdg_data_home = Some(
        get_env_or_default("XDG_DATA_HOME", &xdg_data_home_default)
    );
    let xdg_config_home_default = format!("{}/{}", home, ".config");
    let xdg_config_home = Some(
        get_env_or_default("XDG_CONFIG_HOME", &xdg_config_home_default)
    );
    let xdg_data_dirs_default = format!("{}", "/usr/local/share/:/usr/share/");
    let xdg_data_dirs = Some(
        get_env_or_default("XDG_DATA_DIRS", &xdg_data_dirs_default)
    );
    let xdg_config_dirs_default = format!("{}", "/etc/xdg");
    let xdg_config_dirs = Some(
        get_env_or_default("XDG_CONFIG_DIRS", &xdg_config_dirs_default)
    );
    let xdg_cache_home_default = format!("{}/{}", home, ".cache");
    let xdg_cache_home = Some(
        get_env_or_default("XDG_CACHE_HOME", &xdg_cache_home_default)
    );
    let xdg_runtime_dir_default = format!("{}", "");
    let xdg_runtime_dir = Some(
        get_env_or_default("XDG_RUNTIME_DIR", &xdg_runtime_dir_default)
    );

    Config::new(
        xdg_data_home, xdg_config_home, xdg_data_dirs,
        xdg_config_dirs, xdg_cache_home, xdg_runtime_dir
    )
}
