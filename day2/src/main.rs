use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    match find_total_score_part1() {
        Ok(total_score) => println!("The total score of part 1 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    }

    match find_total_score_part2() {
        Ok(total_score) => println!("The total score of part 2 is: {}", total_score),
        Err(e) => println!("There was an error: {}", e),
    }
}

fn find_total_score_part1() -> Result<i32, Box<dyn Error>> {
    let winning_combination: Vec<(&str, &str)> = vec![
        ("A", "Y"), //rock, paper
        ("B", "Z"), //paper, scissors
        ("C", "X"), //scissors, rock
    ];

    let drawing_combination: Vec<(&str, &str)> = vec![
        ("A", "X"), //rock, rock
        ("B", "Y"), //paper, paper
        ("C", "Z"), //scissors, scissors
    ];

    let score_of_choice: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let file = File::open("input.txt")?;

    let mut current_score = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() == 0 {
            break;
        }

        let (enemy_choice, my_choice) = line.split_at(1);

        let my_choice = my_choice.replace(" ", "");
        let my_choice = my_choice.as_str();

        if winning_combination.contains(&(enemy_choice, my_choice)) {
            current_score += 6;
        } else if drawing_combination.contains(&(enemy_choice, my_choice)) {
            current_score += 3;
        } else {
            current_score += 0;
        }

        current_score += score_of_choice[my_choice];
    }

    Ok(current_score)
}

fn find_total_score_part2() -> Result<i32, Box<dyn Error>> {
    let inverse_winning: HashMap<&str, &str> = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let inverse_drawing: HashMap<&str, &str> = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let inverse_losing: HashMap<&str, &str> = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);

    let score_of_choice: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let file = File::open("input.txt")?;

    let mut current_score = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() == 0 {
            break;
        }

        let (enemy_choice, objective) = line.split_at(1);
        let objective = objective.replace(" ", "");
        let objective = objective.as_str();

        let mut score_of_round = 0;
        let my_choice = if objective == "Z" {
            //want to win
            score_of_round += 6;
            inverse_winning[enemy_choice]
        } else if objective == "Y" {
            //want to draw
            score_of_round += 3;
            inverse_drawing[enemy_choice]
        } else if objective == "X" {
            //want to lose
            score_of_round += 0;
            inverse_losing[enemy_choice]
        } else {
            return Err(Box::from("Not a valid outcome!"));
        };

        score_of_round += score_of_choice[my_choice];

        current_score += score_of_round;
    }

    Ok(current_score)
}
