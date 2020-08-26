use std::io;

fn main() {
    loop {
        let mut n = String::new();

        println!("Will find nth fibonacci number, what is n?");

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num @ 1..=10000000) => num,
            _ => {
                eprintln!("Incorrect choice or out of bound value !");
                continue;
            }
        };

        let result: u32 = if n == 1 { 0 } else { fib(0, 1, 2, n) };

        println!("The {}th fibonacci number is {}", n, result);

        break;
    }
}

fn fib(x: u32, y: u32, curr_iter: u32, n: u32) -> u32 {
    if curr_iter == n {
        y
    } else {
        fib(y, x + y, curr_iter + 1, n)
    }
}
