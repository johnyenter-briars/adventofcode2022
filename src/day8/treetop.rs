use std::{error::Error};

use crate::util::reading::read_lines;

// TODO: all this code should use `enumerate` rather than indexes, but i'm like 4 days behind and gotta go fast
pub fn find_visible_trees(path: &str) -> Result<String, Box<dyn Error>> {
    let mut forest: Vec<Vec<u32>> = vec![];
    for line in read_lines(path, true)? {
        let chars: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        forest.push(chars);
    }

    let mut num_visible = 0;
    for target_y in 0..forest.len() {
        for target_x in 0..forest[target_y].len() {
            let target_value = forest[target_y][target_x];

            let is_visible = check_visible_from_top(target_value, target_x, target_y, &forest)
                || check_visible_from_bottom(target_value, target_x, target_y, &forest)
                || check_visible_from_left(target_value, target_x, target_y, &forest)
                || check_visible_from_right(target_value, target_x, target_y, &forest);

            if is_visible {
                num_visible += 1;
            }
        }
    }

    Ok(num_visible.to_string())
}

pub fn find_max_scenic_score(path: &str) -> Result<String, Box<dyn Error>> {
    let mut forest: Vec<Vec<u32>> = vec![];
    for line in read_lines(path, true)? {
        let chars: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        forest.push(chars);
    }

    let mut scenic_scores: Vec<i32> = vec![];

    for target_y in 0..forest.len() {
        for target_x in 0..forest[target_y].len() {
            let target_value = forest[target_y][target_x];
            let foo = scenic_score_from_top(target_value, target_x, target_y, &forest)
                * scenic_score_from_bottom(target_value, target_x, target_y, &forest)
                * scenic_score_from_left(target_value, target_x, target_y, &forest)
                * scenic_score_from_right(target_value, target_x, target_y, &forest);

            scenic_scores.push(foo);
        }
    }

    Ok(scenic_scores.iter().max().unwrap().to_string())
}

fn check_visible_from_top(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> bool {
    let mut sum_eq_greater_trees_ = 0;
    for y in 0..target_y {
        let value = forest[y][target_x];
        if value >= target_value {
            sum_eq_greater_trees_ += 1;
        }
    }
    sum_eq_greater_trees_ == 0
}

fn check_visible_from_bottom(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> bool {
    let mut sum_eq_greater_trees_ = 0;
    for y in target_y + 1..forest.len() {
        let value = forest[y][target_x];
        if value >= target_value {
            sum_eq_greater_trees_ += 1;
        }
    }
    sum_eq_greater_trees_ == 0
}

fn check_visible_from_left(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> bool {
    let mut sum_eq_greater_trees_ = 0;
    for x in 0..target_x {
        let value = forest[target_y][x];
        if value >= target_value {
            sum_eq_greater_trees_ += 1;
        }
    }
    sum_eq_greater_trees_ == 0
}

fn check_visible_from_right(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> bool {
    let mut sum_eq_greater_trees_ = 0;
    for x in target_x + 1..forest[target_y].len() {
        let value = forest[target_y][x];
        if value >= target_value {
            sum_eq_greater_trees_ += 1;
        }
    }
    sum_eq_greater_trees_ == 0
}

fn scenic_score_from_top(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> i32 {
    let mut num_trees = 0;
    for y in (0..target_y).rev() {
        let value = forest[y][target_x];
        num_trees += 1;
        if value >= target_value {
            return num_trees;
        }
    }
    num_trees
}

fn scenic_score_from_bottom(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> i32 {
    let mut num_trees = 0;
    for y in target_y + 1..forest.len() {
        let value = forest[y][target_x];
        num_trees += 1;
        if value >= target_value {
            return num_trees;
        }
    }
    num_trees
}

fn scenic_score_from_left(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> i32 {
    let mut num_trees = 0;
    for x in (0..target_x).rev() {
        let value = forest[target_y][x];
        num_trees += 1;
        if value >= target_value {
            return num_trees;
        }
    }
    num_trees
}

fn scenic_score_from_right(
    target_value: u32,
    target_x: usize,
    target_y: usize,
    forest: &Vec<Vec<u32>>,
) -> i32 {
    let mut num_trees = 0;
    for x in target_x + 1..forest[target_y].len() {
        let value = forest[target_y][x];
        num_trees += 1;
        if value >= target_value {
            return num_trees;
        }
    }
    num_trees
}
