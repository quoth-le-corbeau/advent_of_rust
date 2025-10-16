fn parse_input(file_path: &str) -> Result<String, std::io::Error> {
    let contents: String = std::fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn part_1(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut line: String = parse_input(file_path).unwrap();
    let mut count: i32 = 0;
    for (i, c) in line.trim().chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => return Err(format!("Invalid character '{c}' at position {i}").into()),
        }
    }
    Ok(count)
}

pub fn part_2(file_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let line: String = parse_input(file_path)?;
    let mut count: i32 = 0;
    for (i, c) in line.trim().chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => return Err(format!("Invalid character '{c}' at position {i}").into()),
        }
        if count == -1 {
            let answer:usize = i + 1;
            return Ok(answer);
        }
    }
    Err("No answer found".into())
}