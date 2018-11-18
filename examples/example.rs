extern crate xdg;


fn main() {
    let config_with_defaults = xdg::get_config_or_default();
    println!("CONFIGURATION USED FOR INSTALLATION:");
    println!("{}", config_with_defaults);

    let config_without_defaults = xdg::get_config();
    println!("DEFAULT VALUES OF XDG VARIABLES");
    println!("{}", config_without_defaults);
}
