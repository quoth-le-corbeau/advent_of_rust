mod day_1;
mod day_10;

pub fn twenty_twenty_three() {
    println!("-----------------------------------------------------");
    println!("\n=========== *** Advent of Code 2023 *** =============");
    println!("-----------------------------------------------------");
    println!("------------------- >>> Day 1 <<< -------------------");
    match day_1_part_1_eg() {
        Ok(value) => println!("Example: Day 1 part 1 value: {value}"),
        Err(e) => println!("Example: Day 1 part 1 Error: {e}"),
    }
    println!("-----------------------------------------------------");
    match day_1_part_1() {
        Ok(value) => println!("Day1 part 1 value: {value}"),
        Err(e) => println!("Day 1 part 1 Error: {e}"),
    }
    println!("-----------------------------------------------------");
    match day_1_part_2_eg() {
        Ok(value) => println!("Example: Day 1 part 2 value: {value}"),
        Err(e) => println!("Example: Day 1 part 2 Error: {e}"),
    }
    println!("-----------------------------------------------------");
    match day_1_part_2() {
        Ok(value) => println!("Day 1 part 2 value: {value}"),
        Err(e) => println!("Day 1 part 2 Error: {e}"),
    }
    println!("-----------------------------------------------------");
}

// Day 1
fn day_1_part_1_eg() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2023/day_1/example.txt")
}
fn day_1_part_1() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2023/day_1/input.txt")
}

fn day_1_part_2_eg() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2023/day_1/example.txt")
}
fn day_1_part_2() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2023/day_1/input.txt")
}
