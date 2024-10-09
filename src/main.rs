use mini_grep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect(); // collecting all arguments into a vector of
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {err}");
        process::exit(1);
    });

    println!("Searching for {:?}", config.query);
    println!("In file `{:?}`", config.file_path);
    println!("================");

    if let Err(e) = mini_grep::run(config) {
        println!("Application crashed {}", e);
        process::exit(1);
    }
}
