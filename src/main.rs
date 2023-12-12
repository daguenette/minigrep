mod libs;

use std::env;
use std::process;
use libs::Config;

fn main() {

    // GET ARGS
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
       println!("Problem parsing arguments: {err}");
       process::exit(1);
    });

    // Run.
    if let Err(e) = libs::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}
