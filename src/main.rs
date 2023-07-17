use std::env;
use std::process;

use grep_off::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_off::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
