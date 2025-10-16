use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(file_path: &str) -> Result<String, std::io::Error> {
    let file: File = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line: String = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}

pub fn part_1(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {

    let mut line: String = parse_input(file_path).unwrap();

    let mut count: i32 = 0;

    for (i, c) in line.trim().chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => return Err(format!("Invalid character '{c}' at position {i}").into()),
        }
    }
    Ok(count)
}

pub fn part_2(file_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    reader.read_line(&mut line)?;

    let mut count: i32 = 0;
    for (i, c) in line.trim().chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => return Err(format!("Invalid character '{c}' at position {i}").into()),
        }
        if count == -1 {
            let answer:usize = i + 1;
            return Ok(answer);
        }
    }
    Err("No answer found".into())
}