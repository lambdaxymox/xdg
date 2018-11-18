extern crate xdg;


fn main() {
    // Query the XDG environment variables for software installation.
    // If some values are missing, we will get the defaults specified in the
    // XDG Base Directory standard.
    let config_with_defaults = xdg::get_config_or_default();
    println!("XDG CONFIGURATION USED FOR INSTALLATION:");
    println!("{}", config_with_defaults);

    // Query the XDG environment variables as they are. 
    let config_without_defaults = xdg::get_config();
    println!("CURRENT VALUES OF XDG VARIABLES");
    println!("{}", config_without_defaults);
}
