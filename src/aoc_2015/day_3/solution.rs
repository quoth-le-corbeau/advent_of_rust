use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    let instructions: String = std::fs::read_to_string(file_path)?;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.extend(deliver_presents(instructions.trim(), 0, 1));
    Ok(visited.len() as u32)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    let instructions: String = std::fs::read_to_string(file_path)?;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.extend(deliver_presents(instructions.trim(), 0, 2));
    visited.extend(deliver_presents(instructions.trim(), 1, 2));

    Ok(visited.len() as u32)
}

fn deliver_presents(instructions: &str, mut index: i32, increment: i32) -> HashSet<(i32, i32)> {
    let index_range: i32 = instructions.len() as i32;
    let mut current: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(current);
    while index < index_range {
        let char: char = instructions.chars().nth(index as usize).unwrap();
        match char {
            '^' => current = (current.0, current.1 - 1),
            'v' => current = (current.0, current.1 + 1),
            '<' => current = (current.0 - 1, current.1),
            '>' => current = (current.0 + 1, current.1),
            _ => println!("Invalid instruction {char}"),
        }
        visited.insert(current);
        index += increment;
    }
    visited
}
