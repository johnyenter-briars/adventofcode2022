use adventofcode2022::util::{day_choice::DayChoice, runnng::run_day};

fn main() {
    let choice = DayChoice::Day10;
    let use_test_data = false;
    println!("Running for day: {:?}, using test data: {}", choice, use_test_data);
    for part in run_day(choice, use_test_data) {
        match part.1 {
            Ok(result) => println!("{} {:?}", part.0, result),
            Err(e) => println!("There was an error! {:?}", e),
        }
    }
}
