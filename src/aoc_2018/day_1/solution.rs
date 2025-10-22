use std::fs;
use std::path::Path;
use std::collections::HashSet;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let changes: Vec<i32> = parse_input(file_path)?;
    let mut resulting_freq: i32 = 0;
    for change in changes {
        resulting_freq += change;
    }
    Ok(resulting_freq)
}

pub fn part_2<P: AsRef<std::path::Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let changes: Vec<i32> = parse_input(file_path)?;
    let mut seen = HashSet::new();
    let mut frequency = 0;
    // Amazing syntactic sugar!!
    for change in changes.iter().cycle() {
        if !seen.insert(frequency) {
            return Ok(frequency);
        }
        frequency += change;
    }

    // Technically unreachable
    Err("No solution found".into())
}

fn parse_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let input_str: String = fs::read_to_string(file_path)?;
    let changes: Vec<i32> = input_str
        .trim()
        .split('\n')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    Ok(changes)
}
