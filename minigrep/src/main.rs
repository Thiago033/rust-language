use std::{
    env, 
    process, 
};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments:\n {}", err);
        process::exit(1);
    });

    println!("Search for: {:?}", config.query);
    println!("In File: {:?}", config.file_name);

    if let Err( e ) = minigrep::run(config) {
        eprintln!("Application Error:\n {}", e);
        process::exit(1);
    }
}
