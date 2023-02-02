use json_parser::{Config, read_yaml, write_yaml};

fn main() {
    let config: Config = read_yaml();
    //config.update_frequency_sec = 69;
    //write_yaml(&config);
    println!("{:?}", config);
}
