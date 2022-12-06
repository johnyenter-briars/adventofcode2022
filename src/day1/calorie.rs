use std::{
    error::Error,
};

use crate::util::reading::read_lines;

pub fn find_max_calories(path: &str) -> Result<String, Box<dyn Error>> {
    let mut elfs: Vec<i32> = vec![];

    let mut current_elf_value = 0;
    for line in read_lines(path)? {
        if line.eq("") {
            elfs.push(current_elf_value);
            current_elf_value = 0;
        } else {
            let value = line.parse::<i32>()?;
            current_elf_value += value;
        }
    }

    let max_elf = elfs.iter().max().expect("No biggest");

    Ok(max_elf.to_string())
}

pub fn find_sum_of_top_3_calories(path: &str) -> Result<String, Box<dyn Error>> {
    let mut elfs: Vec<i32> = vec![];

    let mut current_elf_value = 0;
    for line in read_lines(path)? {
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

    Ok(sum.to_string())
}
