use std::error::Error;
use std::path::Path;
use std::collections::HashSet;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    let instructions: String = std::fs::read_to_string(file_path)?;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let start: (i32, i32) = (0, 0);
    visited.insert(start);
    let mut current: (i32, i32) = (0, 0);
    for char in instructions.trim().chars() {
        match char {
            '^' => current = (current.0, current.1 - 1),
            'v' => current = (current.0, current.1 + 1),
            '<' => current = (current.0 - 1, current.1),
            '>' => current = (current.0 + 1, current.1),
            _ => Err("Invalid instruction {char}")?,
        }
        visited.insert(current);
    }

    Ok(visited.len() as u32)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    Ok(1)
}
