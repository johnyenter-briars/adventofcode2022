use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};
pub fn read_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(path)?;

    let idk = io::BufReader::new(file).lines();

    Ok(idk)
}
