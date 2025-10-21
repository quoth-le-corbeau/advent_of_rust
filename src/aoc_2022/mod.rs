mod day_1;
mod day_7;

pub fn twenty_twenty_two() {
    println!("-----------------------------------------------------");
    println!("\n=========== *** Advent of Code 2022 *** =============");
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

    println!("------------------- >>> Day 7 <<< -------------------");
    match day_7_part_1_eg() {
        Ok(value) => println!("Example: Day 7 part 1 value: {value}"),
        Err(e) => println!("Example: Day 7 part 1 Error: {e}"),
    }
    println!("-----------------------------------------------------");
    match day_7_part_1() {
        Ok(value) => println!("Day1 part 1 value: {value}"),
        Err(e) => println!("Day 7 part 1 Error: {e}"),
    }
    println!("-----------------------------------------------------");
    match day_7_part_2_eg() {
        Ok(value) => println!("Example: Day 7 part 2 value: {value}"),
        Err(e) => println!("Example: Day 7 part 2 Error: {e}"),
    }
    println!("-----------------------------------------------------");
    match day_7_part_2() {
        Ok(value) => println!("Day 7 part 2 value: {value}"),
        Err(e) => println!("Day 7 part 2 Error: {e}"),
    }
    println!("-----------------------------------------------------");
}

// Day 1
fn day_1_part_1_eg() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2022/day_1/example.txt")
}
fn day_1_part_1() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2022/day_1/input.txt")
}

fn day_1_part_2_eg() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2022/day_1/example.txt")
}
fn day_1_part_2() -> Result<u32, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2022/day_1/input.txt")
}

// Day 7
fn day_7_part_1_eg() -> Result<i64, Box<dyn std::error::Error>> {
    day_7::solution::part_1("src/aoc_2022/day_7/example.txt")
}
fn day_7_part_1() -> Result<i64, Box<dyn std::error::Error>> {
    day_7::solution::part_1("src/aoc_2022/day_7/input.txt")
}

fn day_7_part_2_eg() -> Result<i64, Box<dyn std::error::Error>> {
    day_7::solution::part_2("src/aoc_2022/day_7/example.txt")
}
fn day_7_part_2() -> Result<i64, Box<dyn std::error::Error>> {
    day_7::solution::part_2("src/aoc_2022/day_7/input.txt")
}
