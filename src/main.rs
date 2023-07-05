use std::env;
use std::process;

use grep_off::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        print!("Problem passing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = grep_off::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
