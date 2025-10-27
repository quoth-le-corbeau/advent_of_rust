use std::path::Path;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let input_str = std::fs::read_to_string(file_path)?;
    // println!("{}", input_str);
    let lines: Vec<&str> = input_str.trim().lines().collect();
    // println!("{:?}", lines);
    let mut line_digits: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut digits: Vec<u32> = Vec::new();
        for char in line.chars() {
            if char.is_ascii_digit() {
                digits.push(char.to_digit(10).unwrap());
                //println!("{char} pushed to digits");
                //println!("{:?}", digits);
            }
        }
        line_digits.push(digits);
    }
    // println!("{:?}", line_digits);
    let mut total: u32 = 0;
    for digits in line_digits {
        //println!("{:?}", digits);
        if digits.len() == 0 {
            continue;
        }
        let first = digits[0];
        let last = digits[digits.len() - 1];
        total += (first*10 + last);
    }
    Ok(total)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(2312)
}
