use std::path::Path;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let groups: Vec<String> = parse_input(file_path)?;
    let mut max_cal: u32 = 0;
    for group in groups {
        let mut total: u32 = 0;
        for item in group.split_whitespace() {
            let calories: u32 = item.parse()?;
            total += calories;
        }
        if total > max_cal {
            max_cal = total;
        }
    }
    Ok(max_cal)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let groups: Vec<String> = parse_input(file_path)?;
    let mut totals: Vec<u32> = Vec::new();
    for group in groups {
        let mut total: u32 = 0;
        for item in group.split_whitespace() {
            let calories: u32 = item.parse()?;
            total += calories;
        }
        totals.push(total);
    }
    totals.sort();
    if totals.len() < 3 {
        return Err("Insufficient calorie groups".into());
    }
    let top_three: Vec<u32> = totals[totals.len() - 3..].to_vec();
    let top_three_sum: u32 = top_three.iter().sum();
    Ok(top_three_sum)
}

fn parse_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input_str: String = std::fs::read_to_string(file_path)?;
    let groups: Vec<String> = input_str.split("\n\n").map(|s| s.to_string()).collect();
    Ok(groups)
}
