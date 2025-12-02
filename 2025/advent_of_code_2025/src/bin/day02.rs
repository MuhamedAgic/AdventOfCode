use std::time::SystemTime;
use advent_of_code_2025::utils;

fn part_one(input: &str) -> u64 {
    let mut invalid_ids: Vec<u64> = vec![];
    for range in input.split(',') {
        let first_and_second: Vec<u64> = range
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        // println!("{:?}", first_and_second);
        for number in first_and_second[0]..=first_and_second[1] {
            let number_str = number.to_string();
            // println!("{:?}", number_str);
            if number_str.len() % 2 == 0 {
                let (first_half, second_half) = number_str.split_at(number_str.len() / 2);
                // println!("{:?}", first_half);
                // println!("{:?}", second_half);
                if first_half == second_half {
                    invalid_ids.push(number_str.parse::<u64>().unwrap());
                }
            }
        }
    }
    invalid_ids.iter().sum()
}


fn part_two(input: &str) -> i32 {
    todo!()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = utils::read_input(1);
    let input = utils::read_input_from_path("C:/git/magic/AdventOfCode/2025/advent_of_code_2025/input/day02.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    // let now = SystemTime::now();
    // println!("Part two: {}", part_two(&input));
    // println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
    //          now.elapsed()?.as_secs(),
    //          now.elapsed()?.as_millis(),
    //          now.elapsed()?.as_micros());

    Ok(())
}