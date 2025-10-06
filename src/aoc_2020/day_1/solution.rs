use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn part_one() -> Result<i32, Box<dyn std::error::Error>> {
    let file_path = "src/aoc/2020/day_1/example.txt";
    let file = File::open(file_path)?;
    let reader:BufReader<File> = BufReader::new(file);
    let lines = BufReader::new(file).lines();
    for line in lines {
        println!("{}", line?);
    }
    // let mut seen: HashSet = HashSet::new();
    Ok(3)
}
pub fn part_two() -> i32 {128}



pub fn part_one() -> Result<(u32), Box<dyn std::error::Error>> {
    let file = File::open("src/day_1/example.txt")?;
    let reader:BufReader<File> = BufReader::new(file);

    let numbers: Vec<u32> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut seen = HashSet::new();

    for &number in &numbers {
        let target: u32 = 2020 - number;
        if seen.contains(&target) {
            return Ok((number*target));
        }
        seen.insert(number);
    }

    Err("No pair found".into())
}