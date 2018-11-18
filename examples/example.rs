extern crate xdg;


fn main() {
    let config = xdg::get_config_or_default();

    println!("{}", config); 
}
