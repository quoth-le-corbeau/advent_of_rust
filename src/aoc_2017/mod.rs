pub mod day_1;


pub fn twenty_seventeen() {
    println!("-----------------------------------------------------");
    println!("\n=========== *** Advent of Code 2017 *** =============");
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
    crate::aoc_2017::day_1::solution::part_1("src/aoc_2017/day_1/example.txt")
}
fn day_1_part_1() -> Result<i32, Box<dyn std::error::Error>> {
    crate::aoc_2017::day_1::solution::part_1("src/aoc_2017/day_1/input.txt")
}

fn day_1_part_2_eg() -> Result<i32, Box<dyn std::error::Error>> {
    crate::aoc_2017::day_1::solution::part_2("src/aoc_2017/day_1/example.txt")
}
fn day_1_part_2() -> Result<i32, Box<dyn std::error::Error>> {
    crate::aoc_2017::day_1::solution::part_2("src/aoc_2017/day_1/input.txt")
}
