use std::{error::Error, vec};

use crate::util::reading::read_lines;

const PRIORITY_TABLE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub fn sum_of_priorities_of_each_group_part2(path: &str) -> Result<String, Box<dyn Error>> {
    let mut group: Vec<String> = vec![];
    let sum: i32 = read_lines(path)?
        .iter()
        .map(|s| s.clone())
        .filter(|s| !s.is_empty())
        .map(|line| {
            group.push(line);
            if group.len() == 3 {
                let dup = get_item_in_all_three_sacks(&group[0], &group[1], &group[2])
                    .expect("No dup found in all 3 sacks?");

                let priority = get_priority_value(dup);

                group.clear();

                priority as i32
            } else {
                0
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    Ok(sum.to_string())
}

pub fn sum_of_priorities_part1(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(read_lines(path)?
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                return Some(0);
            }

            let (left_sack, right_sack) = line.split_at(line.len() / 2);

            let matching_item = get_item_in_both_sacks(left_sack, right_sack)
                .expect("No matching item found in both sacks??");

            let priority = get_priority_value(matching_item);

            Some(priority)
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>().to_string())
}

fn get_item_in_both_sacks(left_sack: &str, right_sack: &str) -> Option<char> {
    let duplicates = left_sack
        .chars()
        .filter(|c| right_sack.contains(*c))
        .collect::<Vec<char>>();

    duplicates.first().copied()
}

fn get_item_in_all_three_sacks(left_sack: &str, mid_sack: &str, right_sack: &str) -> Option<char> {
    let dups = left_sack
        .chars()
        .filter(|c| mid_sack.contains(*c))
        .filter(|c| right_sack.contains(*c))
        .collect::<Vec<char>>();

    dups.first().copied()
}

fn get_priority_value(matching_item: char) -> usize {
    PRIORITY_TABLE
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .position(|c| *c == matching_item)
        .expect(&format!(
            "Value: {} is not found in the priority table!",
            matching_item
        ))
        + 1
}
