use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn part_one(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file: File = File::open(file_path)?;
    let reader:BufReader<File> = BufReader::new(file);
    for line in reader.lines().flatten() {
        println!("{}", line);
    };
    // let mut seen: HashSet = HashSet::new();
    let x: i32 = 2020;
    if x == 2020 {
        Ok(x)
    }
    else {
        return Err("Something went wrong".into());
    }

}
pub fn part_two(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let x:i32 = 128;
    if x == 128 {
        Ok(x)
    }
    else { Err("Not yet implemented".into()) }
}



pub fn part_one_ref() -> Result<u32, Box<dyn std::error::Error>> {
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
            return Ok(number*target);
        }
        seen.insert(number);
    }

    Err("No pair found".into())
}