pub fn part_1(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let line: String = parse_input(file_path).unwrap();
    let count: i32 = count_floors(&line)?;
    Ok(count)
}

pub fn part_2(file_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let line: String = parse_input(file_path)?;
    count_floors_until_target(&line, -1)
}

fn count_floors(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
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

fn count_floors_until_target(line: &str, target: i32) -> Result<usize, Box<dyn std::error::Error>> {
    let mut count: i32 = 0;
    for (i, c) in line.trim().chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => return Err(format!("Invalid character '{c}' at position {i}").into()),
        }
        if count == target {
            let answer: usize = i + 1;
            return Ok(answer);
        }
    }
    Err("No answer found".into())
}
fn parse_input(file_path: &str) -> Result<String, std::io::Error> {
    let contents: String = std::fs::read_to_string(file_path)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use crate::aoc_2015::day_1::solution::count_floors;
    use crate::aoc_2015::day_1::solution::count_floors_until_target;
    use parameterized::parameterized;

    #[test]
    fn test_count_floors() {
        let test_cases = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            (")))", -3),
            ("))(", -1),
            (")())())", -3),
        ];
        for (input, expected) in test_cases {
            let result = count_floors(input);
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[parameterized(input={"()())", ")"}, expected={5, 1})]
    fn test_count_floors_until_target(input: &str, expected: usize) {
        assert_eq!(count_floors_until_target(input, -1).unwrap(), expected);
    }
}
