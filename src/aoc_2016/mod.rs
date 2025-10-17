pub mod day_1;

pub fn twenty_sixteen() {
    println!("-----------------------------------------------------");
    println!("\n============ *** Advent of Code 2016 *** ============\n");
    println!("-----------------------------------------------------");
    println!("------------------- >>> Day 1 <<< -------------------");
    match day_1_part_1_eg() {
        Ok(result) => println!("Example Day 1 Part 1: {}", result),
        Err(error) => println!("Example Day 1 Part 1 Error: {}", error),
    }
    println!("-----------------------------------------------------");
    match day_1_part_1() {
        Ok(result) => println!("Day 1 Part 1: {}", result),
        Err(error) => println!("Day 1 Part 1 Error: {}", error),
    }
    println!("-----------------------------------------------------");
    match day_1_part_2_eg() {
        Ok(result) => println!("Example Day 1 Part 2: {}", result),
        Err(error) => println!("Example Day 1 Part 2 Error: {}", error),
    }
    println!("-----------------------------------------------------");
    match day_1_part_2() {
        Ok(result) => println!("Day 1 Part 2: {}", result),
        Err(error) => println!("Day 1 Part 2 Error: {}", error),
    }
}

fn day_1_part_1_eg() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2016/day_1/example.txt")
}

fn day_1_part_1() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2016/day_1/input.txt")
}
fn day_1_part_2_eg() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2016/day_1/example.txt")
}

fn day_1_part_2() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2016/day_1/input.txt")
}
