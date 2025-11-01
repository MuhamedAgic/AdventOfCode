use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn get_file_reader_for(filename: &str) -> BufReader<File>
{
    // Lees bestand uit
    let current_dir = std::env::current_dir().unwrap()
                                             .to_str()
                                             .unwrap()
                                             .to_owned();

    let path_to_input = current_dir + filename;

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path_to_input).unwrap();
    let reader = BufReader::new(file);

    return reader;
}

pub fn handle_going_left(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32), steps: i32, distinct_t_positions: &mut HashSet<(i32, i32)>) -> ()
{
    for i in 1..=steps
    {
        // als t onder h
        if t_pos.1 < h_pos.1 
        {
            // als rechts van h, volg h diagonaal omhoog naar links
            if t_pos.0 > h_pos.0
            {
                t_pos.0 -= 1;
                t_pos.1 += 1;
            }
        }
        // als t zelfde rij als h
        else if t_pos.1 == h_pos.1 
        {
            // als t rechts van h, volg h
            if t_pos.0 > h_pos.0
            {
                t_pos.0 -= 1;
            }
        }
        // als t boven h
        else if t_pos.1 > h_pos.1 
        {
            // als rechts van h, volg h diagonaal omlaag
            if t_pos.0 > h_pos.0
            {
                t_pos.0 -= 1;
                t_pos.1 -= 1;
            }
        }

        h_pos.0 -= 1;   // stap naar links
        distinct_t_positions.insert(t_pos.clone()); // het is een hashset, daar zitten toch alleen unieke in :)
    }

}

pub fn handle_going_right(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32), steps: i32, distinct_t_positions: &mut HashSet<(i32, i32)>) -> ()
{
    for i in 1..=steps
    {
        // als t onder h
        if t_pos.1 < h_pos.1 
        {
            // als links van h, volg h diagonaal omhoog naar rechts
            if t_pos.0 < h_pos.0
            {
                t_pos.0 += 1;
                t_pos.1 += 1;
            }
        }
        // als t zelfde rij als h
        else if t_pos.1 == h_pos.1 
        {
            // als t links van h, volg h
            if t_pos.0 < h_pos.0
            {
                t_pos.0 += 1;
            }
        }
        // als t boven h
        else if t_pos.1 > h_pos.1 
        {
            // als links van h, volg h diagonaal omlaag naar rechts
            if t_pos.0 < h_pos.0
            {
                t_pos.0 += 1;
                t_pos.1 -= 1;
            }
        }

        h_pos.0 += 1;   // stap naar rechts
        distinct_t_positions.insert(t_pos.clone()); // het is een hashset, daar zitten toch alleen unieke in :)
    }
}

pub fn handle_going_up(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32), steps: i32, distinct_t_positions: &mut HashSet<(i32, i32)>) -> ()
{
    for i in 1..=steps
    {
        // als t onder h
        if t_pos.1 < h_pos.1 
        {
            // als links van h, volg h diagonaal omhoog naar rechts
            if t_pos.0 < h_pos.0
            {
                t_pos.0 += 1;
            }
            // als rechts van h, volg h diagonaal omhoog naar links
            else if t_pos.0 > h_pos.0
            {
                t_pos.0 -= 1;
            }
            // ga in ieder geval een stap omhoog, geld voor alle gevallen
            t_pos.1 += 1;
        }

        h_pos.1 += 1;   // stap naar boven
        distinct_t_positions.insert(t_pos.clone()); // het is een hashset, daar zitten toch alleen unieke in :)
    }
}

pub fn handle_going_down(h_pos: &mut (i32, i32), t_pos: &mut (i32, i32), steps: i32, distinct_t_positions: &mut HashSet<(i32, i32)>) -> ()
{
    for i in 1..=steps
    {
        // als t boven h
        if t_pos.1 > h_pos.1 
        {
            // als links van h, volg h diagonaal omlaag naar rechts
            if t_pos.0 < h_pos.0
            {
                t_pos.0 += 1;
            }
            // als rechts van h, volg h diagonaal omlaag naar links
            else if t_pos.0 > h_pos.0
            {
                t_pos.0 -= 1;
            }
            // ga in ieder geval een stap omlaag, geld voor alle gevallen
            t_pos.1 -= 1;
        }

        h_pos.1 -= 1;   // stap omlaag
        distinct_t_positions.insert(t_pos.clone()); // het is een hashset, daar zitten toch alleen unieke in :)
    }
}

pub fn track_amount_distinct_t_positions(h_pos: &mut (i32, i32),
                                         t_pos: &mut (i32, i32),
                                         operation: char, 
                                         steps: i32, 
                                         distinct_t_positions: &mut HashSet<(i32, i32)>) -> ()
{
    match operation 
    {
        'L' => handle_going_left(h_pos, t_pos, steps, distinct_t_positions),
        'R' => handle_going_right(h_pos, t_pos, steps, distinct_t_positions),
        'U' => handle_going_up(h_pos, t_pos, steps, distinct_t_positions),
        'D' => handle_going_down(h_pos, t_pos, steps, distinct_t_positions),
        _ => ()
    }
}

pub fn day9_part1() -> u32
{
    let reader = get_file_reader_for("\\AOC2022\\inputs\\day9.txt");
    
    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);
    let mut distinct_t_positions: HashSet<(i32, i32)> = HashSet::new();

    for line in reader.lines() // Ignore errors.
    {
        let unsplit_operation_and_steps =  line.unwrap();
        let mut operation_and_steps = unsplit_operation_and_steps.split_whitespace();

        let operation = operation_and_steps.clone().nth(0).unwrap().chars().next().expect("Kon operation niet lezen");// to char
        let steps = operation_and_steps.nth(1).unwrap().parse::<i32>().unwrap();

        track_amount_distinct_t_positions(&mut h_pos, &mut t_pos, operation, steps, &mut distinct_t_positions);
    }

    return distinct_t_positions.len() as u32;
}
