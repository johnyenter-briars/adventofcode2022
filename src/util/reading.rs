use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;

    let filtered = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        // .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    Ok(filtered)
}
