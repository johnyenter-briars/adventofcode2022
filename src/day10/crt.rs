use core::panic;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    vec,
};

use regex::Regex;

use crate::util::reading::read_lines;

const CYCLE_MARKERS: [u32; 6] = [20, 60, 100, 140, 180, 220];

pub fn sum_of_signal_strengths_at_cycles(path: &str) -> Result<String, Box<dyn Error>> {
    let mut cycle = 0;
    let mut x_register = 1;
    let mut signal_strength = 0;
    let cycle_nums: HashMap<String, u32> =
        HashMap::from([("noop".to_string(), 1), ("addx".to_string(), 2)]);

    for line in read_lines(path, true)? {
        if line.contains("noop") {
            update_cycles(&mut cycle, &mut signal_strength, x_register, "noop", &cycle_nums);
        } else if line.contains("addx") {
            update_cycles(&mut cycle, &mut signal_strength, x_register, "addx", &cycle_nums);
            let re = Regex::new(r"^addx (.*)$").unwrap();
            let caps = re.captures(&line).unwrap();
            let number: i32 = caps.get(1).unwrap().as_str().parse()?;
            x_register += number;
        } else {
            panic!("Unknown instruction!: {}", line);
        }
    }

    Ok(signal_strength.to_string())
}

fn check_for_signal_strength(cycle: u32, x_register: i32) -> i32 {
    println!("cycle: {}, register: {}", cycle, x_register);
    match CYCLE_MARKERS.contains(&cycle) {
        true => {
            let val = x_register * cycle as i32;
            println!("adding strenghth: {}", val);
            val
        },
        false => 0,
    }
}
fn update_cycles(cycle: &mut u32, signal_strength: &mut i32, x_register: i32, instruction: &str, cycle_nums: &HashMap<String, u32>) {
    let cycles_to_do = cycle_nums[instruction];
    for _ in 0..cycles_to_do {
        *cycle += 1;
        *signal_strength += check_for_signal_strength(cycle.clone(), x_register);
        println!("signal strength: {}", signal_strength);
    }
}
