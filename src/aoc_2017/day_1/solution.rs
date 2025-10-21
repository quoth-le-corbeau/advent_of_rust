use std::path::Path;
pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let nums: Vec<i32> = parse_input(file_path)?;
    let mut total: i32 = 0;
    for (i, num) in nums.iter().enumerate() {
        let n: i32 = nums[i];
        let mut next_: usize = i + 1;
        if i + 1 == nums.len() {
            next_ = 0;
        }
        let next_num: i32 = nums[next_];
        if n == next_num {
            total += num;
        }
    }

    Ok(total)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let input_str: String = std::fs::read_to_string(file_path)?;
    Ok(2)
}

fn parse_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let input_str: String = std::fs::read_to_string(file_path)?;
    let input_str = input_str.trim();
    let mut nums: Vec<i32> = Vec::new();
    for char in input_str.chars() {
        let n: i32 = char.to_string().parse()?;
        nums.push(n);
    }
    Ok(nums)
}
