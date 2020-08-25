use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    /*
    Next, we’re adding two lines in the middle. The rand::thread_rng function will give us the
    particular random number generator that we’re going to use: one that is local to the current
    thread of execution and seeded by the operating system. Then we call the gen_range method on
    the random number generator. This method is defined by the Rng trait that we brought
    into scope with the use rand::Rng statement.

    */
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
            .expect("Failed to read line"); // If this instance of io::Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it.

        println!("You guessed: {}", guess);

        // parse() returns a Result enum and we pattern match over it using "match"
        // Here we annotate the guess varaible to contain unsigned 32-bit number, so rust knows its data type
        // Knowing its type helps us to make comparisons later on using the cmp().
        // A few number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others.
        // Rust defaults to an i32, which is the type of secret_number unless you add type information 
        // elsewhere that would cause Rust to infer a different numerical type. This is seen when using cmp()

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Since guess is of type u32 (given by theannotation) the comparison with secret_number means 
        // that Rust will infer that secret_number should be a u32 as well. 
        // So now the comparison will be between two values of the same type!
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
