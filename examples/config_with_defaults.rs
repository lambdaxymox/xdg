extern crate xdg;


fn main() {
    // Query the XDG environment variables for software installation.
    // If some values are missing, we will get the defaults specified in the
    // XDG Base Directory standard.
    let config_with_defaults = xdg::get_config_with_defaults();
    println!("XDG CONFIGURATION USED FOR INSTALLATION:");
    println!("{}", config_with_defaults);
}
