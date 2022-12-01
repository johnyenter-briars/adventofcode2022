use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, Read},
};

fn main() {
    match find_max_calories() {
        Ok(total_calories) => println!("The max elf is: {}", total_calories),
        Err(e) => println!("There was an error: {}", e),
    }

    match find_sum_of_top_3_calories() {
        Ok(top_3) => println!("The sum of the top 3 elfs are: {:?}", top_3),
        Err(e) => println!("There was an error: {}", e),
    }
}

fn find_max_calories() -> Result<i32, Box<dyn Error>> {
    let mut elfs: Vec<i32> = vec![];

    let file = File::open("data.txt")?;

    let mut current_elf_value = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line?;

        if line.eq("") {
            elfs.push(current_elf_value);
            current_elf_value = 0;
        } else {
            let value = line.parse::<i32>()?;
            current_elf_value += value;
        }
    }

    let max_elf = elfs.iter().max().expect("No biggest");

    Ok(*max_elf)
}

fn find_sum_of_top_3_calories() -> Result<i32, Box<dyn Error>> {
    let mut elfs: Vec<i32> = vec![];

    let file = File::open("data.txt")?;

    let mut current_elf_value = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line?;

        if line.eq("") {
            elfs.push(current_elf_value);
            current_elf_value = 0;
        } else {
            let value = line.parse::<i32>()?;
            current_elf_value += value;
        }
    }

    let mut top_3: Vec<i32> = vec![];

    while top_3.len() != 3 {
        let max_elf = elfs.iter().max().expect("No biggest");

        top_3.push(*max_elf);

        let max_position = elfs.iter().position(|e| e == max_elf).unwrap();

        elfs.remove(max_position);
    }

    let sum = top_3[0] + top_3[1] + top_3[2];

    Ok(sum)
}
