use std::error::Error;

use crate::{
    day1::calorie::{find_max_calories, find_sum_of_top_3_calories},
    day2::rpc::{find_total_score_part1, find_total_score_part2},
    day3::rucksack::{sum_of_priorities_of_each_group_part2, sum_of_priorities_part1},
    day4::campcleanup::{overlapping_full_pairs_part1, overlapping_part_of_pairs_part2},
    day5::supply_stacks::{supply_stacks_part1, supply_stacks_part2},
    day6::turning_trouble::find_start_of_message_marker,
    day8::treetop::{find_max_scenic_score, find_visible_trees}, day9::headtail::{find_total_tail_positions, find_total_rope_locations}, day10::crt::sum_of_signal_strengths_at_cycles,
};

type DayResult = Vec<(&'static str, Result<String, Box<dyn Error>>)>;

use super::day_choice::DayChoice;

pub fn run_day(choice: DayChoice, use_test_data: bool) -> DayResult {
    match choice {
        DayChoice::Day1 => day1(use_test_data),
        DayChoice::Day2 => day2(use_test_data),
        DayChoice::Day3 => day3(use_test_data),
        DayChoice::Day4 => day4(use_test_data),
        DayChoice::Day5 => day5(use_test_data),
        DayChoice::Day6 => day6(use_test_data),
        DayChoice::Day7 => panic!("i failed at day 7 :-("),
        DayChoice::Day8 => day8(use_test_data),
        DayChoice::Day9 => day9(use_test_data),
        DayChoice::Day10 => day10(use_test_data),
        DayChoice::Day11 => todo!(),
        DayChoice::Day12 => todo!(),
        DayChoice::Day13 => todo!(),
        DayChoice::Day14 => todo!(),
        DayChoice::Day15 => todo!(),
        DayChoice::Day16 => todo!(),
        DayChoice::Day17 => todo!(),
        DayChoice::Day18 => todo!(),
        DayChoice::Day19 => todo!(),
        DayChoice::Day20 => todo!(),
        DayChoice::Day21 => todo!(),
        DayChoice::Day22 => todo!(),
        DayChoice::Day23 => todo!(),
        DayChoice::Day24 => todo!(),
        DayChoice::Day25 => todo!(),
    }
}

fn day1(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day1", use_test_data);
    vec![
        ("The max elf is: ", find_max_calories(path)),
        (
            "The sum of the top 3 elfs are:  ",
            find_sum_of_top_3_calories(path),
        ),
    ]
}

fn day2(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day2", use_test_data);
    vec![
        (
            "The total score of part 1 is: ",
            find_total_score_part1(path),
        ),
        (
            "The total score of part 2 is:  ",
            find_total_score_part2(path),
        ),
    ]
}

fn day3(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day3", use_test_data);
    vec![
        (
            "The sum of priorities part 2 is: ",
            sum_of_priorities_part1(path),
        ),
        (
            "The sum of priorities part 2 is:  ",
            sum_of_priorities_of_each_group_part2(path),
        ),
    ]
}

fn day4(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day4", use_test_data);
    vec![
        (
            "The number of pairs in which one contains the other is: ",
            overlapping_full_pairs_part1(path),
        ),
        (
            "The number of pairs in which any element overlaps is: ",
            overlapping_part_of_pairs_part2(path),
        ),
    ]
}
fn day5(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day5", use_test_data);
    vec![
        (
            "crates on the top in crane 9000 is: ",
            supply_stacks_part1(path),
        ),
        (
            "crates on the top in crane 9001 is: ",
            supply_stacks_part2(path),
        ),
    ]
}

fn day6(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day6", use_test_data);
    vec![
        (
            "chars before the first marker of size 4: ",
            find_start_of_message_marker(path, 4),
        ),
        (
            "chars before the first marker of size 14: ",
            find_start_of_message_marker(path, 14),
        ),
    ]
}

fn day8(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day8", use_test_data);
    vec![
        (
            "the number of trees visible are: ",
            find_visible_trees(path),
        ),
        ("the max scenic score is: ", find_max_scenic_score(path)),
    ]
}

fn day9(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day9", use_test_data);
    vec![(
        "the number locations visited by the tail is: ",
        find_total_tail_positions(path),
    ),(
        "the number of locations visited by the rope is: ",
        find_total_rope_locations(path),
    )]
}

fn day10(use_test_data: bool) -> DayResult {
    let path = &get_data_path("day10", use_test_data);
    vec![(
        "the sum of signal strengts is: ",
        sum_of_signal_strengths_at_cycles(path),
    )]
}

fn get_data_path(day: &str, use_test_data: bool) -> String {
    if use_test_data {
        format!("src/{}/test-input.txt", day)
    } else {
        format!("src/{}/input.txt", day)
    }
}
