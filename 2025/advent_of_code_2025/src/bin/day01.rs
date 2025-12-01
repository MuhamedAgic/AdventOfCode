use std::time::SystemTime;
use advent_of_code_2025::utils;

fn part_one(input: &str) -> i32 {
    let max_value = 99;
    let min_value = 0;
    let mut current_dial_value = 50;
    let mut amount_times_zero = 0;

    for line in input.lines() {
        let (rotation_side, rotation_amount) = line.split_at(1);
        let rotation_amount = rotation_amount.parse::<i32>().unwrap();

        if rotation_side == "L" {
            current_dial_value -= rotation_amount;
        } else if rotation_side == "R" {
            current_dial_value += rotation_amount;
        }

        while current_dial_value > max_value {
            current_dial_value -= (max_value + 1);
        }
        while current_dial_value < min_value {
            current_dial_value += (max_value + 1);
        }

        if current_dial_value == 0 {
            amount_times_zero += 1;
        }
    }
    amount_times_zero
}


fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = utils::read_input(1);
    let input = utils::read_input_from_path("C:/git/magic/AdventOfCode/2025/advent_of_code_2025/input/day01.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    // let now = SystemTime::now();
    // println!("Part One V2: {}", part_two(&input));
    // println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
    //          now.elapsed()?.as_secs(),
    //          now.elapsed()?.as_millis(),
    //          now.elapsed()?.as_micros());

    Ok(())
}