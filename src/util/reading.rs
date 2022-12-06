use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read_lines(path: &str, f: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;

    let filtered = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        // .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    if f {
        Ok(filtered
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>())
    } else {
        Ok(filtered)
    }
}
