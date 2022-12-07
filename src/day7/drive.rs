use std::{collections::HashMap, error::Error, vec};

use regex::Regex;

use crate::util::reading::read_lines;

pub fn sum_of_dirs_with_total_size_at_most_part1(
    path: &str,
    most_size: i32,
) -> Result<String, Box<dyn Error>> {
    let mut file_size: HashMap<String, i32> = HashMap::new();
    let mut dir_contains: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_contains: HashMap<String, Vec<String>> = HashMap::new();

    let mut dirs_values: HashMap<String, i32> = HashMap::new();

    let mut current_dir = String::new();
    let mut listing_contents = false;
    for line in read_lines(path, true)? {
        let cd_regex = Regex::new(r"^\$ cd (.*)$").unwrap();
        let ls_regex = Regex::new(r"^\$ ..$").unwrap();
        let dir_regex = Regex::new(r"^dir (.*)$").unwrap();
        let file_regex = Regex::new(r"^([0-9]*) (.*)$").unwrap();

        if let Some(caps) = cd_regex.captures(&line) {
            let target_dir = caps.get(1).unwrap().as_str().to_string();
            if &target_dir != ".." && !&target_dir.is_empty() {
                dirs_values.insert(target_dir.clone(), 0);
                if target_dir == "cfgg" {
                    let idk = 10;
                }
            }
            current_dir = target_dir;
            listing_contents = false;
        }

        if let Some(caps) = ls_regex.captures(&line) {
            listing_contents = true;
        }

        if let Some(caps) = dir_regex.captures(&line) {
            let new_dir = caps.get(1).unwrap().as_str().to_string();
            match dir_contains.get_mut(&current_dir) {
                Some(v) => v.push(new_dir),
                None => {
                    dir_contains.insert(current_dir.clone(), vec![new_dir.clone()]);
                    dirs_values.insert(new_dir.clone(), 0);
                }
            };
        }

        if let Some(caps) = file_regex.captures(&line) {
            let size: i32 = caps.get(1).unwrap().as_str().to_string().parse().unwrap();
            let new_file = caps.get(2).unwrap().as_str().to_string();

            file_size.insert(new_file.clone(), size);
            match file_contains.get_mut(&current_dir) {
                Some(v) => v.push(new_file),
                None => {
                    file_contains.insert(current_dir.clone(), vec![new_file]);
                }
            };
        }
    }

    println!("{:?}", file_size);
    println!("{:?}", dir_contains);
    println!("{:?}", file_contains);
    println!("{:?}", dirs_values);
    println!("i'm sad {:?}", dirs_values.get("cfgg"));

    let mut dirs: Vec<String> = dirs_values.keys().cloned().collect();

    let mut idk_value = "";
    while !dirs.is_empty() {
        let target_dir = dirs.pop().unwrap();

        // if contains_f.len() == 0 {
        //     continue;
        // }

        let test_dir = "cfgg".to_string();
        println!("Remaining dirs: {}", dirs.len());
        // if dirs.len() <= 43 {
        if target_dir == test_dir  {
            let empty_vec: Vec<String> = vec![];
            let contains_f = file_contains.get(&target_dir).unwrap_or(&empty_vec);
            let contains_dirs = dir_contains.get(&target_dir).unwrap_or(&empty_vec);

            println!("files: {:?}", contains_f);
            println!("dirs: {:?}", contains_dirs);
            for dir in contains_dirs {
                println!("{}, {:?}", dir, dirs_values.get(dir));
            }
        }

        //remove dirs which contain no files

        let mut dir_value = 0;
        if let Some(contains_dirs) = dir_contains.clone().get_mut(&target_dir) {
            let ready_to_sum = loop {
                if let Some(dir) = contains_dirs.pop() {
                    if dirs_values.get(&dir).unwrap() > &0 {
                        dir_value += dirs_values.get(&dir).unwrap();
                    } else {
                        dirs.insert(0, target_dir.clone());
                        break false;
                    }
                } else {
                    break true;
                }
            };

            if !ready_to_sum {
                continue;
            }
        }

        if let Some(contains_files) = file_contains.get(&target_dir) {
            for file in contains_files {
                dir_value += file_size.get(file).unwrap();
            }
        }

        let v = dirs_values.get_mut(&target_dir).unwrap();
        *v += dir_value;
    }

    let mut sum = 0;
    for dir in dirs_values {
        if dir.1 < most_size {
            sum += dir.1;
        }
    }

    Ok(sum.to_string())
}

fn print_value(
    target_dir: &String,
    file_contains: &HashMap<String, Vec<String>>,
    dir_contains: &HashMap<String, Vec<String>>,
) {
}
