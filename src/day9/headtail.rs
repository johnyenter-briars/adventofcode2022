use std::{
    collections::{HashMap, HashSet},
    error::Error,
    vec,
};

use itertools::Itertools;

use crate::util::reading::read_lines;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn find_total_tail_positions(path: &str) -> Result<String, Box<dyn Error>> {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut points_tail_touches = HashSet::new();
    points_tail_touches.insert(tail);
    for line in read_lines(path, true)? {
        let (direction, magnitude) = line.split_once(' ').unwrap();
        let num_to_move: i32 = magnitude.to_string().parse().unwrap();

        for _ in 0..num_to_move {
            move_point_by_x(&mut head, direction, 1);

            let difference_of_points = Point {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };

            let not_touching = difference_of_points.x.abs() > 1 || difference_of_points.y.abs() > 1;

            if not_touching {
                tail.x += difference_of_points.x.signum();
                tail.y += difference_of_points.y.signum();

                points_tail_touches.insert(tail);
            }
        }
    }

    Ok(points_tail_touches.len().to_string())
}

pub fn find_total_rope_locations(path: &str) -> Result<String, Box<dyn Error>> {
    let mut start_location = Point { x: 0, y: 0 };
    let mut rope = vec![start_location; 10];
    let mut points_tail_touches = HashSet::new();
    points_tail_touches.insert(start_location);

    for line in read_lines(path, true)? {
        let (direction, magnitude) = line.split_once(' ').unwrap();
        let num_to_move: i32 = magnitude.to_string().parse().unwrap();

        for _ in 0..num_to_move {
            move_point_by_x(&mut rope[0], direction, 1);

            for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                // determine if head and tail are touching
                let diff = Point {
                    x: rope[head_idx].x - rope[tail_idx].x,
                    y: rope[head_idx].y - rope[tail_idx].y,
                };
                let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;

                // update tail and insert it into the seen set if needed
                if not_touching {
                    rope[tail_idx].x += diff.x.signum();
                    rope[tail_idx].y += diff.y.signum();
                    if tail_idx == rope.len() - 1 {
                        points_tail_touches.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }

    Ok(points_tail_touches.len().to_string())
}

fn move_point_by_x(point: &mut Point, direction: &str, magnitude: i32) {
    match direction {
        "L" => point.x -= magnitude,
        "R" => point.x += magnitude,
        "U" => point.y += magnitude,
        "D" => point.y -= magnitude,
        _ => panic!("idk"),
    };
}
