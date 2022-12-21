use std::{collections::HashMap, error::Error, vec};

use crate::util::reading::read_lines;

use itertools::Itertools;
use phf::phf_map;
use regex::Regex;

struct Monkey {
    id: u32,
    items: Vec<u32>,
    operation: fn(u32) -> u32,
    test: fn(u32) -> bool,
    if_true_throw_to: u32,
    if_false_throw_to: u32,
}

static OPERATION_MAP: phf::Map<&'static str, fn(u32) -> u32> = phf_map! {
    "new = old * 7" => |old: u32| { old * 7 },
    "new = old + 6" => | old: u32| { old + 6 },
    "new = old * old" => | old: u32| { old * old },
    "new = old + 3" => | old: u32| { old + 3 },
    "new = old * 19" => | old: u32| { old * 19 },
};

static TEST_MAP: phf::Map<&'static str, fn(u32) -> bool> = phf_map! {
    "divisible by 23" => |worried_lvl:u32| {  worried_lvl % 23 == 0 },
    "divisible by 19" => |worried_lvl:u32| {  worried_lvl % 19 == 0 },
    "divisible by 13" => |worried_lvl:u32| {  worried_lvl % 13 == 0 },
    "divisible by 17" => |worried_lvl:u32| {  worried_lvl % 17 == 0 },
};

pub fn level_of_monkey_business(path: &str, rounds: u32) -> Result<String, Box<dyn Error>> {
    let monkeys = create_monkeys(path)?;

    Ok("Idk".to_string())
}

// fn capture_values() -> i32 {
//     let caps = Regex::new(r"^addx (.*)$").unwrap().captures(&line).unwrap();
//     let number: i32 = caps.get(1).unwrap().as_str().parse()?;
//     10
// }

fn create_monkeys(path: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut creating_monkey: u32 = 999;
    let mut items: Vec<u32> = vec![];
    let mut if_true_throw_to = 999;
    let mut if_false_throw_to = 999;
    let mut operation: fn(u32) -> u32 = |_: u32| panic!("default operation");
    let mut test: fn(u32) -> bool = |_: u32| panic!("default operation");
    let mut creating_monkey_val = true;

    for line in read_lines(path, false)? {
        if line.contains("Monkey") {
            let (_, monkey_number) = line.split_once(' ').unwrap();
            creating_monkey = monkey_number.replace(":", "").parse::<u32>().unwrap();
            creating_monkey_val = true;
        }

        if line.len() == 0 {
            creating_monkey_val = false;
        }

        if creating_monkey != 999 {
            if line.contains("Starting") {
                let (_, starting_items) = line.split_once(": ").unwrap();
                items = starting_items
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
            } else if line.contains("Operation") {
                let (_, op) = line.split_once(": ").unwrap();
                operation = *OPERATION_MAP.get(op).unwrap();
            } else if line.contains("Test") {
                let (_, t) = line.split_once(": ").unwrap();
                test = *TEST_MAP.get(t).unwrap();
            } else if line.contains("If true") {
                let (_, monkey_num) = line.split_once(": throw to monkey ").unwrap();
                if_true_throw_to = monkey_num.parse().unwrap();
            } else if line.contains("If false") {
                let (_, monkey_num) = line.split_once(": throw to monkey ").unwrap();
                if_false_throw_to = monkey_num.parse().unwrap();
            }
        }

        if !creating_monkey_val {
            let new_monkey = Monkey {
                id: creating_monkey,
                items: items.clone(),
                operation,
                test,
                if_true_throw_to,
                if_false_throw_to,
            };

            monkeys.push(new_monkey);
        }
    }

    // i literally dont care how shit this code is
    let new_monkey = Monkey {
        id: creating_monkey,
        items: items.clone(),
        operation,
        test,
        if_true_throw_to,
        if_false_throw_to,
    };

    monkeys.push(new_monkey);

    Ok(monkeys)
}
