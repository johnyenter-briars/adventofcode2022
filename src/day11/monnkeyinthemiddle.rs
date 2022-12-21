use std::{collections::HashMap, collections::VecDeque, error::Error, num, vec};

use crate::util::reading::read_lines;

use itertools::Itertools;
use phf::phf_map;
use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test: fn(u64) -> bool,
    if_true_throw_to: u64,
    if_false_throw_to: u64,
}

static OPERATION_MAP: phf::Map<&'static str, fn(u64) -> u64> = phf_map! {
    "new = old * 7" => |old: u64| { old * 7 },
    "new = old + 6" => | old: u64| { old + 6 },
    "new = old * old" => | old: u64| { old * old },
    "new = old + 3" => | old: u64| { old + 3 },
    "new = old * 19" => | old: u64| { old * 19 },
    "new = old + 8" => | old: u64| { old + 8 },
    "new = old * 13" => | old: u64| { old * 13 },
    "new = old + 7" => | old: u64| { old + 7 },
    "new = old + 2" => | old: u64| { old + 2 },
    "new = old + 1" => | old: u64| { old + 1 },
    "new = old + 4" => | old: u64| { old + 4 },
};

static TEST_MAP: phf::Map<&'static str, fn(u64) -> bool> = phf_map! {
    "divisible by 23" => |worried_lvl:u64| {  worried_lvl % 23 == 0 },
    "divisible by 19" => |worried_lvl:u64| {  worried_lvl % 19 == 0 },
    "divisible by 13" => |worried_lvl:u64| {  worried_lvl % 13 == 0 },
    "divisible by 17" => |worried_lvl:u64| {  worried_lvl % 17 == 0 },
    "divisible by 2" => |worried_lvl:u64| {  worried_lvl % 2 == 0 },
    "divisible by 5" => |worried_lvl:u64| {  worried_lvl % 5 == 0 },
    "divisible by 3" => |worried_lvl:u64| {  worried_lvl % 3 == 0 },
    "divisible by 7" => |worried_lvl:u64| {  worried_lvl % 7 == 0 },
    "divisible by 11" => |worried_lvl:u64| {  worried_lvl % 11 == 0 },
};

pub fn level_of_monkey_business(path: &str, rounds: u64) -> Result<String, Box<dyn Error>> {
    let mut monkeys = create_monkeys(path)?;

    let mut monkey_inspect_map: HashMap<u64, u64> = monkeys
        .iter()
        .map(|m| (m.id, 0))
        .collect::<Vec<(u64, u64)>>()
        .into_iter()
        .collect();

    //hard coded a universal divisor cause bro i'm so tired
    let mod_all = 9699690;

    for _ in 1..rounds + 1 {
        for monkey_index in 0..monkeys.len() {
            loop {
                let passes_test: (u64, u64) = {
                    let monkey = &mut monkeys[monkey_index];

                    if let Some(worry_level) = monkey.items.pop_front() {
                        let idk = monkey_inspect_map.get_mut(&monkey.id).unwrap();
                        *idk += 1;

                        //Worry level is applied operation.
                        let mut new_worry_level = (monkey.operation)(worry_level);

                        //Monkey gets bored with item. Worry level is divided by 3
                        if rounds == 20 {
                            new_worry_level = ((new_worry_level / 3) as f32).floor() as u64;
                        } else {
                            new_worry_level = new_worry_level % mod_all;

                        }

                        let passes_test = (monkey.test)(new_worry_level);

                        if passes_test {
                            let id = monkey.if_true_throw_to.clone();
                            (id, new_worry_level)
                        } else {
                            let id = monkey.if_false_throw_to.clone();
                            (id, new_worry_level)
                        }
                    } else {
                        break;
                    }
                };
                let monkey_to_move_item_to = &mut monkeys
                    .iter_mut()
                    .find_or_first(|m| m.id == passes_test.0)
                    .unwrap();

                monkey_to_move_item_to.items.push_back(passes_test.1);
            }
        }
    }

    let foo = monkey_inspect_map
        .values()
        .sorted()
        .rev()
        .take(2)
        .collect::<Vec<&u64>>();

    let idk: u64 = foo.into_iter().product();

    Ok(idk.to_string())
}

fn create_monkeys(path: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut creating_monkey: u64 = 999;
    let mut items: VecDeque<u64> = VecDeque::new();
    let mut if_true_throw_to = 999;
    let mut if_false_throw_to = 999;
    let mut operation: fn(u64) -> u64 = |_: u64| panic!("default operation");
    let mut test: fn(u64) -> bool = |_: u64| panic!("default operation");
    let mut creating_monkey_val = true;

    for line in read_lines(path, false)? {
        if line.contains("Monkey") {
            let (_, monkey_number) = line.split_once(' ').unwrap();
            creating_monkey = monkey_number.replace(":", "").parse::<u64>().unwrap();
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
                    .collect::<VecDeque<&str>>()
                    .iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<VecDeque<u64>>();
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
