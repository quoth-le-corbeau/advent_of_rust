mod day_1;
mod day_2;

pub fn twenty_fifteen() {
    println!("-----------------------------------------------------");
    println!("\n============ *** Advent of Code 2015 *** ============");
    println!("-----------------------------------------------------");
    println!("------------------- >>> Day 1 <<< -------------------");
    match day_1_part_1_eg() {
        Ok(value) => println!("Example: Day 1 part 1 value: {value}"),
        Err(error) => println!("Example Day 1 Part 1 Error: {error}"),
    }
    println!("-----------------------------------------------------");
    match day_1_part_1() {
        Ok(value) => println!("Day 1 part 1 value: {value}"),
        Err(error) => println!("Day 1 Part 1 Error: {error}"),
    }
    println!("-----------------------------------------------------");
    match day_1_part_2_eg() {
        Ok(value) => println!("Example Day 1 part 2 value: {value}"),
        Err(error) => println!("Example Day 1 Part 2 Error: {error}"),
    }
    println!("-----------------------------------------------------");
    match day_1_part_2() {
        Ok(value) => println!("Day 1 part 2 value: {value}"),
        Err(error) => println!("Day 1 Part 2 Error: {error}"),
    }
}

// Day 1
fn day_1_part_1_eg() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2015/day_1/example.txt")
}

fn day_1_part_1() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2015/day_1/input.txt")
}

fn day_1_part_2_eg() -> Result<usize, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2015/day_1/example.txt")
}

fn day_1_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2015/day_1/input.txt")
}
// Day 2
fn day_2_part_1_eg() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2015/day_2/example.txt")
}

fn day_2_part_1() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_1("src/aoc_2015/day_2/input.txt")
}

fn day_2_part_2_eg() -> Result<usize, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2015/day_2/example.txt")
}

fn day_2_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    day_1::solution::part_2("src/aoc_2015/day_2/input.txt")
}
