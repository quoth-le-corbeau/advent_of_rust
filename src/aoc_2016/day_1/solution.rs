use std::collections::HashSet;
use std::path::Path;

struct Position {
    unit_vector: (i32, i32),
    position: (i32, i32),
}
pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = parse_input(file_path)?;
    let mut state = Position {
        unit_vector: (0, -1),
        position: (0, 0),
    };
    for instruction in instructions {
        state = move_per_instruction(instruction, state)?;
    }
    let manhattan: i32 = state.position.0.abs() + state.position.1.abs();
    Ok(manhattan as u32)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = parse_input(file_path)?;
    let mut state = Position{unit_vector: (0, 0), position: (0, 0)};
    let mut visited : HashSet<(i32, i32)> = HashSet::new();
    for instruction in instructions {
        state = move_per_instruction(instruction, state)?;
        if visited.contains(&state.position) {
            let manhattan: i32 = state.position.0.abs() + state.position.1.abs();
            return Ok(manhattan as u32)
        }
        else {
            visited.insert(state.position);
        }
    }
    Err("No solution found".into())

}

fn move_per_instruction(
    instruction: String,
    mut state: Position,
) -> Result<Position, Box<dyn std::error::Error>> {
    let direction = &instruction[0..1];
    let distance: i32 = instruction[1..].parse()?;
    if direction == "R" {
        state.unit_vector = (-state.unit_vector.1, state.unit_vector.0);
    } else if direction == "L" {
        state.unit_vector = (state.unit_vector.1, -state.unit_vector.0);
    } else {
        Err("Invalid direction: {direction}")?;
    }
    state.position = (
        state.position.0 + state.unit_vector.0 * distance,
        state.position.1 + state.unit_vector.1 * distance,
    );
    Ok(state)
}

fn parse_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, std::io::Error> {
    let content: String = std::fs::read_to_string(file_path)?;
    let parts: Vec<String> = content.trim().split(", ").map(|s| s.to_string()).collect();
    Ok(parts)
}
