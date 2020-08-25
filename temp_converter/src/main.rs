use std::io;

fn main() {
    loop {
        println!(
            "Enter 1 for fahrenheit to celsius conversation and 2 for celsius to fahrenheit converstion"
        );

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 2 {
                    println!("Incorrect choice try again !");
                    continue
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Incorrect choice try again !");
                continue
            }
        };

        let choice_str = if choice == 1 { "celsius" } else { "fahrenheit" };

        println!("Enter temperatur in {}", choice_str);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,

            Err(_) => {
                println!("Wrong temperature input, try again !");
                continue;
            }
        };

        if choice == 1 {
            let result: f32 = (input - 32.0) * 5.0 / 9.0;
            println!("{}F is {}°C !", input, result);
            break;
        } else {
            println!("{}°C is {}F !", input, ((input * 9.0 / 5.0) + 32.0));
            break;
        }
    }
}
