use std::error::Error;

use crate::{
    day1::calorie::{find_max_calories, find_sum_of_top_3_calories},
    day2::rpc::{find_total_score_part1, find_total_score_part2},
    day3::rucksack::{sum_of_priorities_of_each_group_part2, sum_of_priorities_part1}, day4::campcleanup::idk,
};

use super::day_choice::DayChoice;

pub fn run_day(choice: DayChoice) -> Vec<(&'static str, Result<i32, Box<dyn Error>>)> {
    match choice {
        DayChoice::Day1 => day1(),
        DayChoice::Day2 => day2(),
        DayChoice::Day3 => day3(),
        DayChoice::Day4 => day4(),
        DayChoice::Day5 => todo!(),
    }
}

fn day1() -> Vec<(&'static str, Result<i32, Box<dyn Error>>)>{
    let path = "src/day1/input.txt";
    vec![
        ("The max elf is: ", find_max_calories(path)),
        ("The sum of the top 3 elfs are:  ", find_sum_of_top_3_calories(path)),
        
    ]
}

fn day2() -> Vec<(&'static str, Result<i32, Box<dyn Error>>)>{
    let path = "src/day2/input.txt";
    vec![
        ("The total score of part 1 is: ", find_total_score_part1(path)),
        ("The total score of part 2 is:  ", find_total_score_part2(path)),
    ]
}

fn day3() -> Vec<(&'static str, Result<i32, Box<dyn Error>>)>{
    let path = "src/day3/input.txt";
    vec![
        ("The sum of priorities part 2 is: ", sum_of_priorities_part1(path)),
        ("The sum of priorities part 2 is:  ", sum_of_priorities_of_each_group_part2(path)),
    ]
}

fn day4() -> Vec<(&'static str, Result<i32, Box<dyn Error>>)>{
    let path = "src/day3/test-input.txt";
    vec![
        ("The sum of priorities part 2 is: ", idk(path)),
    ]
}