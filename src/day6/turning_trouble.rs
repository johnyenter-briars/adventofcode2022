use std::{collections::VecDeque, error::Error};

use crate::util::reading::read_lines;

pub fn find_start_of_message_marker(path: &str, message_length: usize) -> Result<String, Box<dyn Error>> {
    let mut current_characters: VecDeque<char> = VecDeque::new();
    let binding = read_lines(path, true)?.pop().unwrap();
    let mut input = binding.chars().into_iter();

    //yea i know this is stupid, but i really wanted to use the `break <value>` syntax
    let mut i = 1;
    let marker = loop {
        if let Some(c) = input.next() {
            current_characters.push_back(c);
            if current_characters.len() > message_length {
                current_characters.pop_front();
            }

            if !any_overlap(&current_characters) && current_characters.len() == message_length {
                break Some(i);
            }
        } else {
            break None;
        }
        i += 1;
    };

    match marker {
        Some(m) => Ok(m.to_string()),
        None => panic!("No marker found??"),
    }
}

// this could be a big O(n) algorithm (using a map) if I cared to actually write it out
fn any_overlap(current_characters: &VecDeque<char>) -> bool {
    let foo = current_characters
        .clone()
        .iter()
        .map(|c| current_characters.iter().filter(|c1| c == *c1).count())
        .any(|num| num > 1);

    foo
}
