use std::fs;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

pub fn day1_challenge1() -> i32
{
    // Lees bestand uit
    let current_dir = std::env::current_dir().unwrap()
                                                     .to_str()
                                                     .unwrap()
                                                     .to_owned();

    let path_to_input = current_dir + "\\inputs\\day1_challenge1.txt";

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path_to_input).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut biggest_sum: i32 = 0;
    let mut current_sum: i32 = 0;

    // elke lijn langs
    for (index, line) in reader.lines().enumerate() 
    {
        let line = line.unwrap(); // Ignore errors.

        // als er een lege lijn is, kijk dan of het huidige totaal aantal calorien van deze elf
        // groter is dan de tot nu toe grootste gevonden
        if line == ""
        {
            if current_sum > biggest_sum
            {
                biggest_sum = current_sum;
            }
            current_sum = 0;
        }

        // als lijn getal bevat, tel ze steeds bij elkaar op
        else 
        {
            current_sum += line.parse::<i32>().unwrap();
        }
    }
    return biggest_sum;
}

pub fn day1_challenge2() -> i32
{
    // Lees bestand uit
    let current_dir = std::env::current_dir().unwrap()
                                                     .to_str()
                                                     .unwrap()
                                                     .to_owned();

    let path_to_input = current_dir + "\\inputs\\day1_challenge1.txt";

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path_to_input).unwrap();
    let reader = BufReader::new(file);

    let mut vector_of_sums : Vec<i32> = Vec::new();  
    let mut current_sum: i32 = 0;

    // elke lijn langs
    for (index, line) in reader.lines().enumerate() 
    {
        let line = line.unwrap(); // Ignore errors.

        // als er een lege lijn is, kijk dan of het huidige totaal aantal calorien van deze elf
        // groter is dan de tot nu toe grootste gevonden
        if line == ""
        {
            vector_of_sums.push(current_sum.clone());
            current_sum = 0;
        }

        // als lijn getal bevat, tel ze steeds bij elkaar op
        else 
        {
            current_sum += line.parse::<i32>().unwrap();
        }
    }

    // Ga de vector langs en kijk welke de 3 grootste zijn en tel ze bij elkaar op
    vector_of_sums.sort();

    let biggest_three_values_summed =  vector_of_sums.index(vector_of_sums.len() - 1) + vector_of_sums.index(vector_of_sums.len() - 2) +  vector_of_sums.index(vector_of_sums.len() - 3);

    return biggest_three_values_summed;
}