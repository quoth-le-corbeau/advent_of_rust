pub fn part_1(file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = parse_input(file_path).unwrap();
    let mut unit_vector = (0, -1);
    let mut position: (i32, i32) = (0, 0);
    for instruction in instructions {
        let direction = &instruction[0..1];
        let distance: u32 = instruction[1..].parse()?;
        if direction == "R" {
            unit_vector = (-unit_vector.1, unit_vector.0);
        } else if direction == "L" {
            unit_vector = (unit_vector.1, -unit_vector.0);
        } else {
            Err("Invalid direction")?;
        }
        // Move distance steps in the unit_vector direction
        position = (
            position.0 + unit_vector.0 * distance as i32,
            position.1 + unit_vector.1 * distance as i32,
        );
    }
    let manhattan: i32 = position.0.abs() + position.1.abs();
    Ok(manhattan as u32)
}

fn parse_input(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let content: String = std::fs::read_to_string(file_path)?;
    let parts: Vec<String> = content.trim().split(", ").map(|s| s.to_string()).collect();
    Ok(parts)
}
