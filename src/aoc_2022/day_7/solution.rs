use std::collections::HashMap;
use std::path::Path;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let lines: HashMap<String, String> = parse_input(file_path)?;
    Ok(-999)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    println!("AoC Day 7 Part2");
    Ok(-999)
}

fn parse_input<P: AsRef<Path>>(
    file_path: P,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let file_content: String = std::fs::read_to_string(file_path)?;
    let parts: Vec<String> = file_content
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let tree: HashMap<String, String> = build_tree(parts)?;
    Ok(tree)

}


fn build_tree(lines: Vec<String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let tree: HashMap<String, String> = HashMap::new();
    for part in lines {
        println!("{}", part);
    }
    Ok(tree)
}