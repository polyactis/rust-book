use std::env;
use std::process;
use c12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}. {} arguments, but need 2.",
            args.len()-1);
        process::exit(1);
    });

    if let Err(e) = c12_minigrep::run(config) {
        println!("Problem in searching through the file .. {e}.");
        process::exit(2);
    }


}