pub fn part_1(file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = parse_input(file_path).unwrap();
    let mut unit_vector = (0, -1);
    let mut position: (i32, i32) = (0, 0);
    for instruction in instructions {
       position = move_per_instruction(instruction, &unit_vector, &position);
    }
    let manhattan: i32 = position.0.abs() + position.1.abs();
    Ok(manhattan as u32)
}

fn move_per_instruction(instruction: String, unit_vector: (i32, i32), position: (i32, i32)) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let direction = &instruction[0..1];
    let distance: i32 = instruction[1..].parse()?;
    if instruction == "R" {
        let mut unit_vector = (-unit_vector.1, -unit_vector.0);
    }
    else if instruction == "L" {
        let mut unit_vector = (unit_vector.1, -unit_vector.0);
    }
    else {
        Err("Invalid direction: {direction}")?;
    }
    position = (
        position.0 + unit_vector.0 * distance as i32,
        position.1 + unit_vector.1 * distance as i32,
        );
    Ok(position);
}

fn parse_input(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let content: String = std::fs::read_to_string(file_path)?;
    let parts: Vec<String> = content.trim().split(", ").map(|s| s.to_string()).collect();
    Ok(parts)
}
