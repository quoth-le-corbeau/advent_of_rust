use std::path::Path;
pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let depths: Vec<i32> = std::fs::read_to_string(file_path)?
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let mut increases: i32 = 0;
    for window in depths.windows(2) {
        let diff: i32 = window[1] - window[0];
        if diff > 0 {
            increases += 1;
        }
    }
    Ok(increases)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let depths: Vec<i32> = std::fs::read_to_string(file_path)?
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let mut increases: i32 = 0;
    let mut previous: i32 = 0;
    for window in depths.windows(3) {
        let window_sum: i32 = window.iter().sum();
        let diff: i32 = window_sum - previous;
        if diff > 0 {
            increases += 1
        }
        previous = window_sum;
    }
    Ok(increases - 1)
}
