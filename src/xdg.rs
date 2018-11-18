use std::env;
use std::ffi::OsStr;
use std::fmt;


/// The possible values of a XDG environment variable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// Environment variable is not set.
    NotPresent,
    /// Environment variable is blank.
    Empty,
    /// Environment variable is set to a non-blank value.
    Occupied(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::NotPresent => write!(f, ""),
            Value::Empty => write!(f, ""),
            Value::Occupied(st) => write!(f, "{}", st),
        }
    }
}

/// The XDG Base Directory configuration. This contains all the information
/// needed to install an XDG complying software application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub xdg_data_home: Value,
    pub xdg_config_home: Value,
    pub xdg_data_dirs: Value,
    pub xdg_config_dirs: Value,
    pub xdg_cache_home: Value,
    pub xdg_runtime_dir: Value,
}

impl Config {
    /// Construct a new set of environment variables.
    pub fn new(
        xdg_data_home: Value, xdg_config_home: Value, 
        xdg_data_dirs: Value, xdg_config_dirs: Value, 
        xdg_cache_home: Value, xdg_runtime_dir: Value) -> Config {
        
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
        writeln!(f, "XDG_DATA_HOME={}", self.xdg_data_home).unwrap();
        writeln!(f, "XDG_CONFIG_HOME={}", self.xdg_config_home).unwrap();
        writeln!(f, "XDG_DATA_DIRS={}", self.xdg_data_dirs).unwrap();
        writeln!(f, "XDG_CONFIG_DIRS={}", self.xdg_config_dirs).unwrap();
        writeln!(f, "XDG_CACHE_HOME={}", self.xdg_cache_home).unwrap();
        writeln!(f, "XDG_RUNTIME_DIR={}", self.xdg_runtime_dir)
    }
}

/// An XDG Base Directory Specification configuration using only the default values
/// from the specification if no XDG environment variables are set.
/// ```ignore
/// XDG_DATA_HOME=$HOME/.local/share
/// XDG_CONFIG_HOME=$HOME/.config
/// XDG_DATA_DIRS=/usr/local/share/:/usr/share/
/// XDG_CONFIG_DIRS=/etc/xdg
/// XDG_CACHE_HOME=$HOME/.cache
/// XDG_RUNTIME_DIR=<No Default Given. This Is System Dependent>
/// ```
pub fn default_config() -> Config {
    let home = env::var("HOME").unwrap();

    Config::new(
        Value::Occupied(format!("{}/{}", home, ".local/share")),
        Value::Occupied(format!("{}/{}", home, ".config")),
        Value::Occupied(format!("{}", "/usr/local/share/:/usr/share/")),
        Value::Occupied(format!("{}", "/etc/xdg")),
        Value::Occupied(format!("{}/{}", home, ".cache")),
        Value::NotPresent,
    )
}

fn get_env<K: AsRef<OsStr>>(key: K) -> Value {    
    let value = env::var(key);
    match value {
        Ok(ref val) if val != "" => Value::Occupied(val.clone()),
        Ok(_) => Value::Empty,
        Err(_) => Value::NotPresent,
    }
}

/// Get the XDG environment variables as they are set in the current running process.
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

fn get_env_or_default<K: AsRef<OsStr>>(key: K, default: Value) -> Value {
    let var = match env::var(key) {
        Ok(ref st) if st == "" => default,
        Ok(st) => Value::Occupied(st),
        Err(_) => default,
    };

    var
}

/// Get the XDG environment variables needed for installing a piece of software,
/// or for the software to find  its data and configuration files on the system.
/// The default values are
/// ```ignore
/// XDG_DATA_HOME=\$HOME/.local/share
/// XDG_CONFIG_HOME=\$HOME/.config
/// XDG_DATA_DIRS=/usr/local/share/:/usr/share/
/// XDG_CONFIG_DIRS=/etc/xdg
/// XDG_CACHE_HOME=\$HOME/.cache
/// XDG_RUNTIME_DIR=<No Default Given. This Is System Dependent>
/// ```
pub fn get_config_with_defaults() -> Config {
    let home = match get_env("HOME") {
        Value::Occupied(st) => st,
        _ => panic!(),
    };
    
    let xdg_data_home_default = Value::Occupied(
        format!("{}/{}", home, ".local/share")
    );
    let xdg_data_home = get_env_or_default(
        "XDG_DATA_HOME", xdg_data_home_default
    );
    let xdg_config_home_default = Value::Occupied(
        format!("{}/{}", home, ".config")
    );
    let xdg_config_home = get_env_or_default(
        "XDG_CONFIG_HOME", xdg_config_home_default
    );
    let xdg_data_dirs_default = Value::Occupied(
        format!("{}", "/usr/local/share/:/usr/share/")
    );
    let xdg_data_dirs = get_env_or_default(
        "XDG_DATA_DIRS", xdg_data_dirs_default
    );
    let xdg_config_dirs_default = Value::Occupied(
        format!("{}", "/etc/xdg")
    );
    let xdg_config_dirs = get_env_or_default(
        "XDG_CONFIG_DIRS", xdg_config_dirs_default
    );
    let xdg_cache_home_default = Value::Occupied(
        format!("{}/{}", home, ".cache")
    );
    let xdg_cache_home = get_env_or_default(
        "XDG_CACHE_HOME", xdg_cache_home_default
    );

    let xdg_runtime_dir = get_env_or_default("XDG_RUNTIME_DIR", Value::NotPresent);

    Config::new(
        xdg_data_home, xdg_config_home, xdg_data_dirs,
        xdg_config_dirs, xdg_cache_home, xdg_runtime_dir
    )
}
