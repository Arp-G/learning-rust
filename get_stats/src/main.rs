use std::collections::HashMap;
use std::io;

fn mean(list: &Vec<i32>) -> f32 {
    list.iter().sum::<i32>() as f32 / list.len() as f32
}

fn median(list: &mut Vec<i32>) -> i32 {
    list.sort(); // To sort we need a mutable refernce of the vector, since we need to change it
    list[list.len() / 2]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();

    for num in list.into_iter() {
        let count = hash.entry(num).or_insert(0);
        *count += 1;
    }

    struct Mode {
        num: i32,
        count: u32,
    }

    let mut mode = Mode { num: 0, count: 0 };

    for (num, count) in hash.into_iter() {
        if count > mode.count {
            mode = Mode {
                num: *num,
                count: count,
            }
        }
    }

    mode.num
}

fn take_input() -> Vec<i32> {
    println!("Enter the numbers separated by space");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input");
    input
        .trim()
        .split(' ')
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>()
}

fn main() {
    let mut list = take_input();

    let mean = mean(&list);

    let median = median(&mut list); // Need to pass mutable reference to sort

    let mode = mode(&list); // We give up ownership of hash here

    println!("====== STATS ======");
    println!("MEAN = {}", mean);
    println!("MEDIAN = {}", median);
    println!("MODE = {}", mode);
}

// fn take_input() -> (Vec<i32>, HashMap<i32, u32>) {
//     println!("Enter the size of the list");

//     let mut input = String::new();

//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line"); //panic! with this message if Result::Err<E>

//     let len = input.trim().parse().expect("Incorrect size !");

//     let mut list: Vec<i32> = Vec::new();
//     let mut hash = HashMap::new();

//     for i in 0..len {
//         let mut input = String::new();

//         println!("Enter number {}: ", (i + 1));

//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let num: i32 = input.trim().parse().expect("Not a valid number !");

//         list.push(num);
//         let count = hash.entry(num).or_insert(0);
//         *count += 1;
//     }

//     (list, hash)
// }
