use std::time::SystemTime;
use advent_of_code_2025::utils;

fn part_one(input: &str) -> i64 {
    let mut nums: Vec<Vec<i64>> = vec![];
    let mut operators: Vec<char> = vec![];

    for line in input.lines() {
        let mut line_nums: Vec<i64> = vec![];
        let line_elements = line.split_whitespace().collect::<Vec<&str>>();
        for element in line_elements.iter() {
            match element.parse::<i64>() {
                Ok(num) => {
                    line_nums.push(num);
                },
                Err(e) => {
                    operators.push(element.chars().next().unwrap());
                }
            }
        }
        if !line_nums.is_empty() {
            nums.push(line_nums);
        }
    }

    let mut ans = 0;
    for i in 0..nums[0].len() {
        let mut sub_ans = 0;
        if operators[i] == '*' {
            sub_ans = 1;
        }
        for j in 0..nums.len() {
            match operators[i] {
                '+' => sub_ans += nums[j][i],
                '*' => sub_ans *= nums[j][i],
                _ => panic!("Don't know what to do! (+-/*)?")
            }
        }
        ans += sub_ans;
    }
    ans
}

fn part_two(input: &str) -> u64 {
    // collect
    let operators: Vec<&str> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .rev()
        .collect();

    let mut lines: Vec<String> = input
        .lines()
        .map(|line| line.to_owned())
        .collect();

    // make all lines even length, spacing and alignment stuff...
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap();
    for line in lines.iter_mut() {
        while line.len() != max_line_length {
            line.push(' ');
        }
    }

    for line in lines.iter() {
        assert_eq!(line.len(), max_line_length);
    }

    lines = lines
        .iter()
        .map(|line| line.chars().rev().collect::<String>())
        .collect();

    // setup
    let mut nums_current_batch = vec![];
    let mut ans = 0;
    let mut curr_operator_to_use = 0;

    // process
    for i in 0..lines[0].len() { // col
        let mut num = String::from("");
        for j in 0..lines.len() { // row
            if let Some(char) = lines[j].chars().nth(i) {
                match char.is_digit(10) {
                    true => num.push(char),
                    false => continue
                }
            }
        }
        if num != "" {
            nums_current_batch.push(num.parse::<u64>().unwrap());
            if i != lines[0].len() - 1 { // so we don't forget to add the last one!
                continue;
            }
        }
        match operators[curr_operator_to_use] {
            "+" => ans += nums_current_batch.iter().fold(0, |acc, x| acc + x),
            "*" => ans += nums_current_batch.iter().fold(1, |acc, x| acc * x),
            _ => panic!("Don't know what to do! (+-/*)?")
        }
        curr_operator_to_use += 1;
        nums_current_batch.clear();
    }
    ans
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input_from_path("C:/git/magic/AdventOfCode/2025/advent_of_code_2025/input/day06.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    // release 263 microseconds
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    let now = SystemTime::now();
    println!("Part two: {}", part_two(&input));
    // 10149093027477 too low
    // release 3001 microseconds
    println!("Elapsed time as:\n    Seconds: {} \n    Milliseconds: {}\n    Microseconds: {}\n",
             now.elapsed()?.as_secs(),
             now.elapsed()?.as_millis(),
             now.elapsed()?.as_micros());

    Ok(())
}