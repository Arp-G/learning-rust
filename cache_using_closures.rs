// Chapter 13.1, Memoization  using a hashmap as a cache
// A closure is used to compute some value which we have considered that it takes some time (thread::sleep(Duration::from_secs(2));)
// So, we avoid recomputing values which the closure has already computed, by storing them in a cache implemented as a hashmap

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // Here if the value present is already in teh cache we just return it
    // otherwise we invoke the closure and compute the value add it to the cache(hash map) and then return it
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v, // Here v is a &u32, 8(&u32) gives uas a u32 and since its a primitive type it gets copied, so the HashMap owns the original value and we return a copy
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    call_with_different_values()
}

fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
