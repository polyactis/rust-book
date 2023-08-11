use std::env;
use std::process;
use c12_minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}. Need 2.");
        process::exit(1);
    });
    

    if let Err(e) = c12_minigrep::run(config) {
        eprintln!("Problem in searching through the file .. {e}.");
        process::exit(2);
    }


}