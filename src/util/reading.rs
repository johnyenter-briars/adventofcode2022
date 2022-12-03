use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};
pub fn read_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(io::BufReader::new(file).lines())
}
