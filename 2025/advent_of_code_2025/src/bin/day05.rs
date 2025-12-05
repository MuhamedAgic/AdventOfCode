use std::time::SystemTime;
use advent_of_code_2025::utils;

fn part_one(input: &str) -> i64 {
    let mut fresh_items = 0;
    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut numbers = vec![];
    let mut collecting_ranges = true;
    for line in input.lines() {
        if line.is_empty() {
            collecting_ranges = false;
            continue;
        }
        if collecting_ranges {
            if let Some(range) = line.split_once('-') {
                let int_range = (range.0.parse::<i64>().unwrap(), range.1.parse::<i64>().unwrap()) ;
                ranges.push(int_range);
            }
        } else {
            numbers.push(line.parse::<i64>().unwrap());
        }
    }

    'outer: for number in numbers.iter() {
        'inner: for range in ranges.iter() {
            if *number >= range.0 && *number <= range.1 {
                fresh_items += 1;
                continue 'outer;
            }
        }
    }

    fresh_items

}

fn part_two(input: &str) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input_from_path("C:/git/magic/AdventOfCode/2025/advent_of_code_2025/input/day05.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part two: {}", part_two(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}