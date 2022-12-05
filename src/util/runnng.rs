use crate::{
    day1::calorie::{find_max_calories, find_sum_of_top_3_calories},
    day2::rpc::{find_total_score_part1, find_total_score_part2},
    day3::rucksack::{sum_of_priorities_of_each_group_part2, sum_of_priorities_part1}, day4::campcleanup::idk,
};

use super::day_choice::DayChoice;

pub fn run_day(choice: DayChoice) {
    match choice {
        DayChoice::Day1 => day1(),
        DayChoice::Day2 => day2(),
        DayChoice::Day3 => day3(),
        DayChoice::Day4 => day4(),
        DayChoice::Day5 => todo!(),
    };
}

fn day1() {
    let path = "src/day1/input.txt";
    match find_max_calories(path) {
        Ok(total_calories) => println!("The max elf is: {}", total_calories),
        Err(e) => println!("There was an error: {}", e),
    }

    match find_sum_of_top_3_calories(path) {
        Ok(top_3) => println!("The sum of the top 3 elfs are: {:?}", top_3),
        Err(e) => println!("There was an error: {}", e),
    }
}

fn day2() {
    let path = "src/day2/input.txt";
    match find_total_score_part1(path) {
        Ok(total_score) => println!("The total score of part 1 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    }

    match find_total_score_part2(path) {
        Ok(total_score) => println!("The total score of part 2 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    }
}

fn day3() {
    let path = "src/day3/input.txt";
    match sum_of_priorities_part1(path) {
        Ok(total_score) => println!("The sum of priorities part 1 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    }

    match sum_of_priorities_of_each_group_part2(path) {
        Ok(total_score) => println!("The sum of priorities part 2 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    }
}

fn day4() {
    let path = "src/day3/test-input.txt";
    match idk(path) {
        Ok(total_score) => println!("The sum of priorities part 1 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    };
}