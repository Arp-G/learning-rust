use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
    If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. However,
    if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as
    an argument to unwrap_or_else.

    unwrap_or_else will pass the inner value of the Err to the closure
    */
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);
    /*
        Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return
        the unwrapped value because it would only be (). So we use 'if let' here !
    */
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
