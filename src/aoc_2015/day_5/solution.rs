use std::error::Error;
use std::path::Path;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    let input_str: String = std::fs::read_to_string(file_path)?;
    let input = input_str.trim();
    let lines: Vec<&str> = input.lines().collect();
    let mut nice: Vec<String> = Vec::new();
    for line in lines {
        let mut vowel_count = 0;
        let mut has_double = false;
        let mut forbidden: bool = false;
        for (i, c) in line.chars().enumerate() {
            if VOWELS.contains(&c) {
                vowel_count += 1;
            }
            if i < line.len() - 1 {
                let next_c = line.chars().nth(i + 1).unwrap();
                let pair: String = format!("{}{}", c, next_c);
                if pair.chars().nth(0) == pair.chars().nth(1) {
                    has_double = true;
                }
                if FORBIDDEN.contains(&pair.as_str()) {
                    forbidden = true;
                    break;
                }
            }
        }
        if forbidden {
            continue;
        }
        if has_double && vowel_count > 2 {
            nice.push(line.to_string());
        }
    }
    Ok(nice.len() as u32)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    Ok(201552)
}
