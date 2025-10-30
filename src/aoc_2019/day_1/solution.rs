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
    let input_lines: Vec<i32> = std::fs::read_to_string(file_path)?
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut total = 0;
    for line in input_lines {
        total += cumulative_fuel_sum(line, 0);
    }
    Ok(total)
}

fn cumulative_fuel_sum(n: i32, mut cumulative_sum: i32) -> i32 {
    if n <= 0 {
        return cumulative_sum;
    }
    let new_n: i32 = (n / 3) - 2;
    if new_n <= 0 {
        return cumulative_sum;
    }
    cumulative_sum += new_n;
    cumulative_fuel_sum(new_n, cumulative_sum)
}
