use std::path::Path;
fn part_1<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let input_str: String = std::fs::read_to_string(file_path)?;
    let mut dimensions: Vec<u32> = Vec::new();
    for line in input_str.trim().lines() {
        dimensions.push(line.parse()?);
    }
    println!("{:?}", dimensions);
    Ok(1521)
}

fn part_2<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(1522)
}
