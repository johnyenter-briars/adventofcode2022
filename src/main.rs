use adventofcode2022::util::{day_choice::DayChoice, runnng::run_day};

fn main() {
    let choice = DayChoice::Day5;
    println!("Runing for day: {:?}", choice);
    for part in run_day(choice) {
        match part.1 {
            Ok(result) => println!("{} {:?}", part.0, result),
            Err(e) => println!("There was an error! {:?}", e),
        }
    }
}
