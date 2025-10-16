pub fn part_1(file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines: String = parse_input(file_path).unwrap();
    println!("{lines}");
    Ok(0)
}

fn parse_input(file_path: &str) -> Result<String, std::io::Error> {
    let content: String = std::fs::read_to_string(file_path)?;
    Ok(content)
}
