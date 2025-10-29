use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Dimensions {
    l: u32,
    w: u32,
    h: u32,
}

impl Dimensions {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<u32> = s
            .split('x')
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() != 3 {
            return Err("Expected exactly three dimensions (l, w, h)".into());
        }
        Ok(Self {
            l: parts[0],
            w: parts[1],
            h: parts[2],
        })
    }

    fn surface_area(&self) -> u32 {
        let s1 = self.l * self.w;
        let s2 = self.w * self.h;
        let s3 = self.h * self.l;
        let smallest = *[s1, s2, s3].iter().min().unwrap();
        2 * (s1 + s2 + s3) + smallest
    }

    fn volume_plus_smallest_perimeter(&self) -> u32 {
        let mut sides: Vec<u32> = vec![self.l, self.w, self.h];
        sides.sort();
        let smallest_perimeter = 2 * sides[0] + 2 * sides[1];
        let volume = self.l * self.w * self.h;
        smallest_perimeter + volume
    }
}

pub fn part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn Error>> {
    let input: String = fs::read_to_string(file_path)?;
    let total: u32 = input
        .trim()
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let dims = Dimensions::from_str(line)?;
            Ok(dims.surface_area())
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(total)
}

pub fn part_2<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let input: String = fs::read_to_string(file_path)?;
    let total: u32 = input
        .trim()
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let dims = Dimensions::from_str(line)?;
            Ok(dims.volume_plus_smallest_perimeter())
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    Ok(total)
}

fn _part_1<P: AsRef<Path>>(file_path: P) -> Result<u32, Box<dyn std::error::Error>> {
    let input_str: String = std::fs::read_to_string(file_path)?;
    let mut all_dimensions: Vec<HashMap<String, u32>> = Vec::new();
    for line in input_str.trim().lines() {
        let split_line: Vec<&str> = line.split("x").collect();
        let mut dimensions: HashMap<String, u32> = HashMap::new();
        dimensions.insert('l'.to_string(), split_line[0].parse::<u32>()?);
        dimensions.insert('w'.to_string(), split_line[1].parse::<u32>()?);
        dimensions.insert('h'.to_string(), split_line[2].parse::<u32>()?);
        all_dimensions.push(dimensions);
    }
    let mut total: u32 = 0;
    for dimensions in all_dimensions {
        let surface_area: u32 = _get_surface_area(&dimensions)?;
        total += surface_area;
    }
    // println!("{:?}", all_dimensions);
    Ok(total)
}
fn _get_surface_area(dimensions: &HashMap<String, u32>) -> Result<u32, Box<dyn std::error::Error>> {
    let l: &u32 = dimensions.get("l").unwrap();
    let w: &u32 = dimensions.get("w").unwrap();
    let h: &u32 = dimensions.get("h").unwrap();
    let s1: u32 = l * w;
    let s2: u32 = l * h;
    let s3: u32 = w * h;
    let mut sides: Vec<u32> = Vec::new();
    sides.push(s1);
    sides.push(s2);
    sides.push(s3);
    sides.sort();
    let smallest_side: u32 = sides[0];
    let surface_area = (s1 + s2 + s3) * 2;

    Ok(surface_area + smallest_side)
}
