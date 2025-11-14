use md5;
use std::path::Path;
pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    find_hash_with_n_zeros(file_path, 5)
}
pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    find_hash_with_n_zeros(file_path, 6)
}

fn find_hash_with_n_zeros<P: AsRef<Path>>(
    file_path: P,
    n: usize,
) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(file_path)?;
    let input_str = input.trim();
    for i in 0.. {
        let combined: String = format!("{}{}", input_str, i);
        let digest = md5::compute(combined.as_bytes());
        let hex = format!("{:x}", digest);
        if hex.starts_with(&"0".repeat(n)) {
            println!("Found {} -> {}", i, hex);
            return Ok(i);
        }
    }
    println!("Uh-oh! No result found");
    Ok(0)
}
