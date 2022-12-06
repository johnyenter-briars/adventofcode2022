use std::{error::Error, num, vec};

use regex::Regex;

use crate::util::reading::read_lines;

pub fn supply_stacks_part1(path: &str) -> Result<String, Box<dyn Error>> {
    let mut temp_stacks: Vec<Vec<char>> = vec![];

    let raw_lines = read_lines(path)?;
    let mut lines = raw_lines.iter().peekable();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();

        let mut chars_iter = line.chars().peekable();

        let mut index = 0;
        while chars_iter.peek().is_some() {
            let (one, two, _) = (
                chars_iter.next().unwrap(),
                chars_iter.next().unwrap(),
                chars_iter.next().unwrap(),
            );

            chars_iter.next(); //skip the space
            if temp_stacks.len() == index {
                temp_stacks.push(vec![]);
            }

            if one == '[' {
                temp_stacks[index].push(two);
            }

            index += 1;
        }

        if line.is_empty() {
            break;
        }
    }

    let mut stacks = temp_stacks
        .into_iter()
        .map(|mut v| {
            v.reverse();
            v.clone()
        })
        .collect::<Vec<Vec<char>>>();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let re = Regex::new(r"^move ([0-9]*) from ([0-9]*) to ([0-9]*)$").unwrap();
        let caps = re.captures(line).unwrap();
        let number_crates: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let destination: usize = caps.get(3).unwrap().as_str().parse().unwrap();

        for _ in 0..number_crates {
            let moving = stacks[from - 1].pop().unwrap();
            stacks[destination - 1].push(moving);
        }
    }

    let v = stacks
        .iter()
        .map(|s| s.last().unwrap().clone())
        .collect::<Vec<char>>();

    let s = String::from_iter(v);

    Ok(s)
}

pub fn supply_stacks_part2(path: &str) -> Result<String, Box<dyn Error>> {
    let mut temp_stacks: Vec<Vec<char>> = vec![];

    let raw_lines = read_lines(path)?;
    let mut lines = raw_lines.iter().peekable();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();

        let mut chars_iter = line.chars().peekable();

        let mut index = 0;
        while chars_iter.peek().is_some() {
            let (one, two, _) = (
                chars_iter.next().unwrap(),
                chars_iter.next().unwrap(),
                chars_iter.next().unwrap(),
            );

            chars_iter.next(); //skip the space
            if temp_stacks.len() == index {
                temp_stacks.push(vec![]);
            }

            if one == '[' {
                temp_stacks[index].push(two);
            }

            index += 1;
        }

        if line.is_empty() {
            break;
        }
    }

    let mut stacks = temp_stacks
        .into_iter()
        .map(|mut v| {
            v.reverse();
            v.clone()
        })
        .collect::<Vec<Vec<char>>>();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let re = Regex::new(r"^move ([0-9]*) from ([0-9]*) to ([0-9]*)$").unwrap();
        let caps = re.captures(line).unwrap();
        let number_crates: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let destination: usize = caps.get(3).unwrap().as_str().parse().unwrap();

        //this sucks and isnt rusty but i'm so tired dog
        let mut moving: Vec<char> = vec![];
        for _ in 0..number_crates {
            moving.push(stacks[from - 1].pop().unwrap());
        }
        moving.reverse();
        for c in moving {
            stacks[destination - 1].push(c);
        }
    }

    let v = stacks
        .iter()
        .map(|s| s.last().unwrap().clone())
        .collect::<Vec<char>>();

    let s = String::from_iter(v);

    Ok(s)
}

// i tried : (
// fn make_stacks<'a, i>(lines: &i) -> vec<vec<char>>
// where
//     i: iterator<item = &'a string>,
// {
//     let mut lines = lines.peekable();
//     let mut temp_stacks: vec<vec<char>> = vec![];

//     while lines.peek().is_some() {
//         let line = lines.next().unwrap();

//         let mut chars_iter = line.chars().peekable();

//         let mut index = 0;
//         while chars_iter.peek().is_some() {
//             let (one, two, _) = (
//                 chars_iter.next().unwrap(),
//                 chars_iter.next().unwrap(),
//                 chars_iter.next().unwrap(),
//             );

//             chars_iter.next(); //skip the space
//             if temp_stacks.len() == index {
//                 temp_stacks.push(vec![]);
//             }

//             if one == '[' {
//                 temp_stacks[index].push(two);
//             }

//             index += 1;
//         }

//         if line.is_empty() {
//             break;
//         }
//     }

//     let mut stacks = temp_stacks
//         .into_iter()
//         .map(|mut v| {
//             v.reverse();
//             v.clone()
//         })
//         .collect::<vec<vec<char>>>();

//     stacks
// }
