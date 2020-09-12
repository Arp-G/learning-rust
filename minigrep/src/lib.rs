use std::error::Error;
use std::fs; // For the trait object Box<dyn Error>

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#the-trade-offs-of-using-clone
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

/*
Box<dyn Error> means the function will return a type that implements the Error trait,The dyn keyword is short for “dynamic.”
This gives us flexibility to return error values that may be of different types in different error cases.
*/

/*
We’ve declared the run function’s success type as () in the signature, which means we need to wrap the unit type value in the Ok value.
This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run
for its side effects only; it doesn’t return a value we need.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
