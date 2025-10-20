use std::collections::{HashMap, HashSet};
use std::path::Path;

const PART_ONE_SIZE_LIMIT: i64 = 100000;
const TOTAL_DISK_SPACE: i64 = 70000000;
const SPACE_REQUIRED_FOR_UPDATE: i64 = 30000000;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i64, Box<dyn std::error::Error>> {
    let (_root_dir_size, size_map) = get_file_system_size(file_path)?;
    let mut total: i64 = 0;
    for (_dir_name, dir_size) in &size_map {
        if *dir_size <= PART_ONE_SIZE_LIMIT {
            total += *dir_size
        }
    }
    Ok(total)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i64, Box<dyn std::error::Error>> {
    let (root_dir_size, size_map) = get_file_system_size(file_path)?;
    let free_disk_space: i64 = TOTAL_DISK_SPACE - root_dir_size;
    let minimum_space_to_free: i64 = SPACE_REQUIRED_FOR_UPDATE - free_disk_space;
    let mut candidates: Vec<i64> = Vec::new();
    for (_dir_name, dir_size) in &size_map {
        if *dir_size >= minimum_space_to_free {
            candidates.push(*dir_size);
        }
    }
    candidates.sort();
    let best_candidate: i64 = *candidates.first().ok_or_else(|| "no solution")?;
    Ok(best_candidate)
}

fn get_file_system_size<P: AsRef<Path>>(
    file_path: P,
) -> Result<(i64, HashMap<String, i64>), Box<dyn std::error::Error>> {
    let tree: HashMap<String, HashSet<String>> = parse_input(file_path)?;
    let mut dir_size_by_dir_name: HashMap<String, i64> = HashMap::new();
    for (dir_name, _) in &tree {
        dir_size_by_dir_name.insert(dir_name.to_string(), 0);
    }
    let (root_dir_size, size_map) = dfs_sum(&tree, &mut dir_size_by_dir_name, &"/".to_string());
    // println!("{}", root_dir_size);
    // println!("{:?}", size_map);
    Ok((root_dir_size, size_map))
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

fn build_tree(
    lines: Vec<String>,
) -> Result<HashMap<String, HashSet<String>>, Box<dyn std::error::Error>> {
    let mut tree: HashMap<String, HashSet<String>> = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    let mut current_dir = "/".to_string(); // Tracks the current dir for inserting entries
    for line in lines {
        if line == "$ ls" {
            continue;
        } else if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd ") {
            let dir_name: String = line.split(" ").last().unwrap().to_string();
            if dir_name == "/" {
                path = vec!["".to_string()];
            } else {
                path.push(dir_name);
            }
            let dir_name = path.join("/").replace("//", "/");
            current_dir = if dir_name.is_empty() {
                "/".to_string()
            } else {
                dir_name
            };
        } else {
            tree.entry(current_dir.clone())
                .or_insert_with(HashSet::new)
                .insert(line);
        }
    }
    Ok(tree)
}

fn dfs_sum(
    tree: &HashMap<String, HashSet<String>>,
    size_map: &mut HashMap<String, i64>,
    node: &str,
) -> (i64, HashMap<String, i64>) {
    let mut dir_size: i64 = 0;
    for child in tree.get(node).unwrap() {
        if child.starts_with("dir ") {
            let mut parts = child.split(' ');
            parts.next(); // Skip "dir"

            if let Some(dir_name) = parts.next() {
                let trimmed_node = node.trim_end_matches('/');

                let new_node = format!("{}/{}", trimmed_node, dir_name.trim());

                let (s, _) = dfs_sum(&tree, size_map, &new_node);
                dir_size += s;
            }
        } else {
            let parts: Vec<&str> = child.split(' ').collect();

            let file_size: usize = match parts.get(0).and_then(|s| s.trim().parse().ok()) {
                Some(size) => size,
                None => panic!("Invalid file: {}", child),
            };

            dir_size += file_size as i64;
        }
    }
    size_map.insert(node.to_string(), dir_size);
    (dir_size, size_map.clone())
}
