use adventofcode2022::util::{day_choice::DayChoice, running::run_day};

fn main() {
    let choice = DayChoice::Day11;
    let use_test_data = true;
    println!("Running for day: {:?}, using test data: {}", choice, use_test_data);
    for part in run_day(choice, use_test_data) {
        match part.1 {
            Ok(result) => println!("{} {:?}", part.0, result),
            Err(e) => println!("There was an error! {:?}", e),
        }
    }
}
