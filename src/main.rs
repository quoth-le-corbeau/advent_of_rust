use std::collections::HashMap;

mod aoc_2015;
mod aoc_2016;
mod aoc_2017;
mod aoc_2018;
mod aoc_2019;
mod aoc_2020;
mod aoc_2022;

fn main() {
    aoc_2015::twenty_fifteen();
    aoc_2016::twenty_sixteen();
    aoc_2017::twenty_seventeen();
    aoc_2018::twenty_eighteen();
    aoc_2019::twenty_nineteen();
    aoc_2020::twenty_twenty();
    aoc_2022::twenty_twenty_two();
    println!("Fibonacci!");
    let mut cache = HashMap::new();
    //let fib_n: u64 = fibonacci(500, &mut cache); <- panic integer overflow
    let fib_n: u64 = fibonacci(91, &mut cache);
    println!("Fibonacci(n=5): {}", fib_n);
}
pub fn fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }

    if let Some(&value) = cache.get(&n) {
        return value;
    }

    let result = fibonacci(n - 1, cache) + fibonacci(n - 2, cache);
    cache.insert(n, result);
    result
}
