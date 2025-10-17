use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone)]
struct Walker {
    unit_vector: (i32, i32),
    position: (i32, i32),
    distance: i32,
    visited: HashSet<(i32, i32)>,
}

impl Walker {
    fn new() -> Self {
        Walker{
            unit_vector: (0, -1), // initialize facing north
            position: (0, 0),
            distance: 0,
            visited: HashSet::new(),
        }
    }

    fn turn_right(&mut self) {
        self.unit_vector = (-self.unit_vector.1, -self.unit_vector.0);
    }

    fn turn_left(&mut self) {
        self.unit_vector = (self.unit_vector.1, -self.unit_vector.0);
    }

    fn manhattan_to_origin(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn move_one_step(&mut self) {
        self.position.0 += self.unit_vector.0;
        self.position.1 += self.unit_vector.1;
        self.visited.insert(self.position);
    }

    fn move_distance(&mut self) {
        self.position.0 += self.unit_vector.0 * self.distance;
        self.position.1 += self.unit_vector.1 * self.distance;
        self.visited.insert(self.position);
    }

    fn parse_instruction(&mut self, instruction: String) -> Result<(), Box<dyn std::error::Error>> {
        let direction = &instruction[0..1];
        let distance = instruction[1..].parse::<i32>()?;

        match direction {
            "R" => self.turn_right(),
            "L" => self.turn_left(),
            _ => return Err(format!("Unknown direction: {direction}").into())
        }
        self.distance = distance;
        Ok(())
    }



}
pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = parse_input(file_path)?;
    let mut walker = Walker::new();
    for instruction in instructions {
        walker.parse_instruction(instruction)?;
        walker.move_distance();
    }
    let result: i32 = walker.manhattan_to_origin();
    Ok(result as u32)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = parse_input(file_path)?;
    Ok(0)
}



fn parse_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, std::io::Error> {
    let content: String = std::fs::read_to_string(file_path)?;
    let parts: Vec<String> = content.trim().split(", ").map(|s| s.to_string()).collect();
    Ok(parts)
}
