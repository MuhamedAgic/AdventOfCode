use std::time::SystemTime;
use advent_of_code_2025::utils;


fn part_one(input: &str) -> u32 {
    let mut biggest_nums = vec![];
    for line in input.lines() {
        let mut biggest = 0;
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                let mut num_str = String::from("");
                num_str.push(line.chars().nth(i).unwrap());
                num_str.push(line.chars().nth(j).unwrap());
                let num: u32 = num_str.parse().unwrap();
                if num > biggest {
                    biggest = num;
                }
            }
        }
        biggest_nums.push(biggest);
    }

    biggest_nums.iter().sum()
}

fn part_two(input: &str) -> u32 {
5

}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = utils::read_input(1);
    let input = utils::read_input_from_path("C:/git/magic/AdventOfCode/2025/advent_of_code_2025/input/day03.txt");

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