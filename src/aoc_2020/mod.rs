pub mod day_1;

pub fn twenty_twenty () {
    println!("-------------");
    println!("Advent of Code 2020");
    println!("-------------");
    match part_one_eg() {
        Ok(value) => println!("Example: Day 1 part 1 value: {value}"),
        Err(e) => println!("Example: Day 1 part 1 Error: {e}")
    }
    println!("-------------");
    match part_one() {
        Ok(value) => println!("Day1 part 1 value: {value}"),
        Err(e) => println!("Day 1 part 1 Error: {e}")
    }
    println!("-------------");
    match part_two_eg() {
        Ok(value) => println!("Example: Day 1 part 2 value: {value}"),
        Err(e) => println!("Example: Day 1 part 2 Error: {e}")
    }
    println!("-------------");
    match part_two() {
        Ok(value) => println!("Day1 part 2 value: {value}"),
        Err(e) => println!("Day 1 part 2 Error: {e}")
    }
    println!("-------------");

}


fn part_one_eg() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_one("src/aoc_2020/day_1/example.txt", 2020)
}
fn part_one() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_one("src/aoc_2020/day_1/input.txt", 2020)
}

fn part_two_eg() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_two("src/aoc_2020/day_1/example.txt", 2020)
}
fn part_two() -> Result<i32, Box<dyn std::error::Error>> {
    day_1::solution::part_two("src/aoc_2020/day_1/input.txt", 2020)
}