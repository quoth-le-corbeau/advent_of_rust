use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let (mut list_1, mut list_2): (Vec<u32>, Vec<u32>) = parse_location_ids(file_path)?;
    list_1.sort();
    list_2.sort();
    // println!("{:?}", list_1);
    // println!("{:?}", list_2);
    let mut delta_total: u32 = 0;
    for (i, &n) in list_1.iter().enumerate() {
        let delta: u32 = list_2[i].abs_diff(n);
        delta_total += delta;
    }
    Ok(delta_total)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let (left_list, right_list): (Vec<u32>, Vec<u32>) = parse_location_ids(file_path)?;
    // {n: N} n = right_list elem N = count for n i
    let right_list_counter: HashMap<u32, u32> = count_occurrences(right_list)?;
    // println!("{:?}", right_list_counter);
    let mut total: u32 = 0;
    for n in left_list {
        match right_list_counter.get(&n) {
            Some(count) => total += count * &n,
            None => continue,
        };
    }
    Ok(total)
}

fn count_occurrences(list: Vec<u32>) -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>> {
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for &n in list.iter() {
        match counter.get(&n) {
            Some(count) => counter.insert(n, count + 1),
            None => counter.insert(n, 1),
        };
    }
    Ok(counter)
}

fn parse_location_ids<P: AsRef<Path>>(
    file_path: P,
) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();
    let file: File = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let ok_line = line?;
        let split_lines: Vec<String> = ok_line.split("   ").map(|x| x.to_string()).collect();
        let n1: u32 = split_lines[0].parse()?;
        let n2: u32 = split_lines[1].parse()?;
        list_1.push(n1);
        list_2.push(n2);
    }

    Ok((list_1.clone(), list_2.clone()))
}
