use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn part_one(file_path: &str, target_sum: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let numbers: Vec<i32> = read_numbers_from_file(file_path)?;
    let (a, b) = find_target_number(&numbers, target_sum)
        .ok_or_else(|| "No solution found")?;
    Ok(a * b)
}
pub fn part_two(file_path: &str, target_sum: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let numbers: Vec<i32> = read_numbers_from_file(file_path)?;
    for &number in &numbers {
       let sub_target: i32 = target_sum - number;
       if let Some((a, b)) = find_target_number(&numbers, sub_target) {
           if a != number && b != number {
               println!("{} -> {} -> {}", number, a, b);
               return Ok(a * b * number);
           }
       }
    }
    Err("No solution found".into())

}

fn read_numbers_from_file(file_path: &str) -> Result<Vec<i32>, std::io::Error> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let numbers: Vec<i32> = reader
        .lines()
        .flatten()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    Ok(numbers)
}

fn find_target_number(numbers: &Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    let mut seen: HashSet<i32> = HashSet::new();
    for &number in numbers {
        let target: i32 = target_sum - number;
        if seen.contains(&target) {
            println!("{} -> {}", number, target);
            return Some((target, number));
        }
        seen.insert(number);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_number_found() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_target_number(&numbers, 2020);

        assert!(result.is_some());
        let (a, b) = result.unwrap();
        assert_eq!(a + b, 2020);
        assert_eq!(a * b, 514579);
    }
}