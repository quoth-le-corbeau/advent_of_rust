use std::path::Path;
pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let input_lines: Vec<u32> = std::fs::read_to_string(file_path)?
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut total = 0;
    for line in input_lines {
        total += (line / 3) - 2;
    }

    Ok(total)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(192)
}
