use std::{error::Error, ops::Index};

use crate::util::reading::read_lines;

const PRIORITY_TABLE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn sum_of_priorities_part1(path: &str) -> Result<i32, Box<dyn Error>> {
    Ok(read_lines(path)?
        .filter_map(|line| {
            let line = line.expect("Unable to create string");

            if line.is_empty() {
                return Some(0);
            }

            let (left_sack, right_sack) = line.split_at(line.len() / 2);

            let matching_item = get_item_in_both_sacks(left_sack, right_sack)
                .expect("No matching item found in both sacks??");

            let priority = make_priority_table()
                .iter()
                .position(|c| *c == matching_item)
                .expect(&format!(
                    "Value: {} is not found in the priority table!",
                    matching_item
                ))
                + 1;

            Some(priority)
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>() as i32)
}

fn get_item_in_both_sacks(left_sack: &str, right_sack: &str) -> Option<char> {
    let idk = left_sack
        .chars()
        .filter(|c| right_sack.contains(*c))
        .collect::<Vec<char>>();

    idk.first().copied()
}

fn make_priority_table() -> Vec<char> {
    PRIORITY_TABLE.chars().collect()
}
