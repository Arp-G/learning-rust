use std::error::Error;
use std::fs; // For the trait object Box<dyn Error>
use std::env; // For working with environment varaibles to distinguish between case sensitive and insensitive grep

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#the-trade-offs-of-using-clone
        let query = args[1].clone();
        let filename = args[2].clone();

        /* If 'CASE_INSENSITIVE' is not set 'env::var("CASE_INSENSITIVE")' returns an Result::Err and is_err() returns true, thus case_sensitive = true
        
         Here we don’t care about the value of the "CASE_INSENSITIVE" environment variable, just whether it’s set or unset, so we’re checking is_err rather 
         than using unwrap, expect, or any of the other methods we’ve seen on Result.
        */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); 

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/*
Here using lifetime 'a, we tell Rust that the data returned by the search function will live as long as the
data passed into the search function in the contents argument.

This is needed since we returns a Vector of references, without the lifetime we will get an error like..
"this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
