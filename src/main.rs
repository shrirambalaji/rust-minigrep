use minigrep::Config;
use std::{env, process};

fn main() {
    // env::args() returns an iterator.
    // .collect converts the iterator into a preferred collection.
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    let query = config.query;
    let filepath = config.filepath;

    println!("Searching for \"{}\"", query);
    println!("In file {}", filepath);

    // Box<dyn Error>
    // dyn -> indicates a dynamic dispatch. W.r.t generics we know rust does monomorphization.
    // using dyn allows rust to have different types of trait objects, that are evaluated during runtime.
    // here Box acts as a smart pointer, that would point to a table of methods to handle the trait object of type `Error`.
    if let Err(e) = minigrep::run(config) {
        eprintln!("An error occured {}", e);
        process::exit(1);
    };
}
