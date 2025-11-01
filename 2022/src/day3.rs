use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

// handig: https://stackoverflow.com/questions/69922830/nested-for-loop-through-two-vectors 

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

// vb: ('a', 1), ('b', 2), ('A', 27)
pub fn get_priorities() -> Vec<(char, i32)>
{
    // https://ascii-tables.com/#:~:text=Standard%20ASCII%20table%20contains%20a%20table%20of%20127,table%20makes%20it%20a%20total%20of%20255%20codes. 
    let mut letters_with_priorities = Vec::new();
    for i in 97..=122   // a tm z
    {
        letters_with_priorities.push(((i as u8) as char, i - 96)); //beginnen bij 1 f*%$**%$!!!!!!!!
    }
    for i in 65..=90    // A tm Z
    {
        letters_with_priorities.push(((i as u8) as char, i - 38)) ;
    }
    return letters_with_priorities;
}

pub fn get_common_chars(input_string: &str) -> Option<HashSet<char>>
{
    if !input_string.len() % 2 == 0
    {
        return None;
    }

    // een geval gehad dat er 2 p's zijn, dus dan wordt ook 2x de waarde opgeteld
    // Dacht dat het zo moest...
    let mut chars_in_both_pockets_of_rucksack = HashSet::new();
    let split_input = input_string.split_at((input_string.len() / 2) as usize);

    for char in split_input.0.chars().into_iter()
    {
        if split_input.1.contains(char)
        {
            chars_in_both_pockets_of_rucksack.insert(char);
        }
    }
    return Some(chars_in_both_pockets_of_rucksack);
}

pub fn get_total_error_for_rucksack(common_chars: &Option<HashSet<char>>, letters_with_priorities: &Vec<(char, i32)>) -> i32
{
    if *common_chars == None
    {
        return 0;
    }
    // 'a' 'b' 'A' wordt 1, 2, 27
    let mut priority_values_of_chars = Vec::new();
    for common_char in common_chars.as_ref().unwrap().iter()
    {
        // vind bijbehorende waarde
        for (letter, corresponding_priority) in letters_with_priorities
        {
            if letter == common_char
            {
                priority_values_of_chars.push(corresponding_priority.clone());
            }
        }
    }
    return priority_values_of_chars.iter().sum();
}

pub fn day3_challenge1() -> i32
{
    let reader = get_file_reader_for("\\inputs\\day3.txt");

    let letters_and_priorities = get_priorities();
    let mut sum_of_priority_errors = 0;

    for line in reader.lines()
    {
        let line = line.unwrap(); // Ignore errors.

        let common_chars = get_common_chars(&line);
        sum_of_priority_errors += get_total_error_for_rucksack(&common_chars, &letters_and_priorities);
    }
    return sum_of_priority_errors;
}




// =========================== part 2 ============================//

pub fn get_common_char_in_group(line1: &String, line2: &String, line3: &String) -> Option<HashSet<char>>
{
    let mut common_char_in_group = HashSet::new();
    for char in line1.as_str().chars().into_iter()
    {
        if line2.contains(char) && line3.contains(char)
        {
            common_char_in_group.insert(char);
        }
    }
    return Some(common_char_in_group);
}


pub fn day3_challenge2() -> i32
{
    let reader = get_file_reader_for("\\inputs\\day3.txt");

    let letters_and_priorities = get_priorities();
    let mut sum_of_priority_errors = 0;

    let mut line1 = String::from("");
    let mut line2 = String::from("");
    let mut line3 = String::from("");

    for (index, line) in reader.lines().enumerate()
    {

        let line = line.unwrap(); // Ignore errors.

        if index == 0 || index == 1 || index == 2
        {
            match index
            {
                0 => line1 = line.clone(),
                1 => line2 = line.clone(),
                2 => line3 = line.clone(),
                _ => ()
            }

            if index == 2
            {
                let common_char_in_group = get_common_char_in_group(&line1, &line2, &line3);
                sum_of_priority_errors += get_total_error_for_rucksack(&common_char_in_group, &letters_and_priorities);
            }
        }
        else
        {
            match (index + 1) % 3
            {
                2 => line2 = line.clone(),
                1 => line1 = line.clone(),
                0 => line3 = line.clone(),
                _ => println!("iets is mis gegaan")
            }
    
            if (index + 1) % 3 == 0
            {
                // we hebben een groep
                let common_char_in_group = get_common_char_in_group(&line1, &line2, &line3);
                sum_of_priority_errors += get_total_error_for_rucksack(&common_char_in_group, &letters_and_priorities);
            }
        }
    }
    return sum_of_priority_errors;
}