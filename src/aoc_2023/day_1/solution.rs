use std::collections::HashMap;
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
        total += first * 10 + last;
    }
    Ok(total)
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const MAX_DIGIT_LEN: usize = 5;

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let lines: Vec<String> = std::fs::read_to_string(file_path)?
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut total: u32 = 0;

    for line in lines {
        let mut digit_by_index: HashMap<usize, u32> = HashMap::new();

        for i in 0..line.len() {
            // Check if current character is an ASCII digit
            if let Some(digit) = line.chars().nth(i).unwrap().to_digit(10) {
                digit_by_index.insert(i, digit);
                continue;
            }

            // Check for word digits (up to MAX_DIGIT_LEN characters)
            for len in 1..=MAX_DIGIT_LEN.min(line.len() - i) {
                let substring = &line[i..i + len];

                // Check if this substring matches any digit word
                if let Some(pos) = DIGITS.iter().position(|&d| d == substring) {
                    digit_by_index.insert(i, (pos + 1) as u32);
                    break; // Found a match, no need to check longer substrings
                }
            }
        }

        // now loop through the hashmap:
        // get value of min key in digits_by_index and store in variable called tens_digit
        let tens_digit = digit_by_index
            .iter()
            .min_by_key(|&(k, _)| k)
            .map(|(_, &v)| v)
            .unwrap_or(0);

        // get value of max key in digits_by_index and store in variable called units_digit
        let units_digit = digit_by_index
            .iter()
            .max_by_key(|&(k, _)| k)
            .map(|(_, &v)| v)
            .unwrap_or(0);

        total += tens_digit * 10 + units_digit;
    }

    Ok(total)
}
