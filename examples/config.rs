extern crate xdg;


fn main() {
    // Query the XDG environment variables as they are, without generating
    // default values for missing parameters.
    let config_without_defaults = xdg::get_config();
    println!("CURRENT VALUES OF XDG VARIABLES");
    println!("{}", config_without_defaults);
}
