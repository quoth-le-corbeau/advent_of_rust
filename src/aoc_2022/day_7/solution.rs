use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let lines: HashMap<String, HashSet<String>> = parse_input(file_path)?;
    println!("lines: {:?}", lines);
    Ok(-999)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    println!("AoC Day 7 Part2");
    Ok(-999)
}

fn parse_input<P: AsRef<Path>>(
    file_path: P,
) -> Result<HashMap<String, HashSet<String>>, Box<dyn std::error::Error>> {
    let file_content: String = std::fs::read_to_string(file_path)?;
    let lines: Vec<String> = file_content
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let tree: HashMap<String, HashSet<String>> = build_tree(lines)?;
    Ok(tree)

}

fn build_tree(lines: Vec<String>) -> Result<HashMap<String, HashSet<String>>, Box<dyn std::error::Error>> {
    let mut tree: HashMap<String, HashSet<String>> = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    let mut current_dir = "/".to_string(); // Tracks the current dir for inserting entries
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line == "$ ls" {
            continue;
        }
        else if line == "$ cd .." {
           path.pop();
        }
        else if line.starts_with("$ cd ") {
            let dir_name: String = line.split(" ").last().unwrap().to_string();
            if dir_name == "/" {
                path = vec!["".to_string()];
            }
            else {
                path.push(dir_name);
            }
            let dir_name = path.join("/").replace("//", "/");
            current_dir  = if dir_name.is_empty() {"/".to_string() } else {dir_name};
        }
        else {
            tree.entry(current_dir.clone())
                .or_insert_with(HashSet::new)
                .insert(line);
        }
    }
    Ok(tree)
}