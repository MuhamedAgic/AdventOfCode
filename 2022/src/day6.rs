use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use itertools::Itertools;

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

pub fn day6_part1() -> i32
{
    let reader = get_file_reader_for("\\inputs\\day6.txt");

    let mut char_count = 0;
    let mut chunk_of_14: Vec<char> = Vec::new();

    for line in reader.lines() // Ignore errors.
    {
        let line = line.unwrap();    
        for i in 0..=line.len()
        {
            if chunk_of_14.len() == 14
            {
                if chunk_of_14.clone()
                              .into_iter()
                              .unique()
                              .collect_vec()
                              .len() == 14
                {
                    break;
                }
                chunk_of_14.remove(0);
            }
            chunk_of_14.push(line.bytes().nth(i).unwrap() as char);
            char_count += 1;
        }
    }
    return char_count;
}
