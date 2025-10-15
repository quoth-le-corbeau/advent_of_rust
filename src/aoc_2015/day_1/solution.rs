use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn part_1(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for line in lines {
        println!("{}", line);
    }
    Ok(0)


}