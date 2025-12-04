use std::time::SystemTime;
use advent_of_code_2025::utils;


fn check_target_north(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row < target.len() - 1 {
        return false; // can't have target, need minimum space of target.len() chars
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row - i][col]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_south(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row > grid.len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row + i][col]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_east(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if col > grid[row].len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row][col + i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_west(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if col < target.len() - 1{
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row][col - i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_north_east(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row < target.len() - 1 || col > grid[row].len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row - i][col + i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_north_west(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row < target.len() - 1 || col < target.len() - 1 {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row - i][col - i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_south_east(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row > grid.len() - target.len() || col > grid[row].len() - target.len() {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row + i][col + i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}

fn check_target_south_west(row: usize, col: usize, grid: &Vec<Vec<char>>, target: &str) -> bool {
    if row > grid[row].len() - target.len() || col < target.len() - 1 {
        return false;
    }

    let mut found_instance = String::from("");
    for i in 0..target.len() {
        found_instance.push(grid[row + i][col - i]);
    }

    if found_instance == target {
        return true;
    }
    return false;
}


fn part_one(input: &str) -> i32 {
    let search_functions = vec![
        check_target_north,
        check_target_south,
        check_target_east,
        check_target_west,
        check_target_north_east,
        check_target_north_west,
        check_target_south_east,
        check_target_south_west
    ];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut found_paper_rolls = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if grid[i][j] != '@' {
                continue;
            }

            let found = search_functions
                .iter()
                .map(|function| function(i, j, &grid, "@@"))
                .filter(|search_result| *search_result)
                .count();

            if found < 4 {
                found_paper_rolls += 1;
            }
        }
    }
    found_paper_rolls
}

fn part_two(input: &str) -> u32 {
5
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = utils::read_input(1);
    let input = utils::read_input_from_path("C:/git/magic/AdventOfCode/2025/advent_of_code_2025/input/day04.txt");

    let now = SystemTime::now();
    println!("Part One: {}", part_one(&input));
    // 20000 too high, of course it is when you use input of day 3...
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