use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

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

// begin is onderaan stack, eind is bovenaan stack
pub fn get_input_towers(towers: &str) -> Vec<Vec<&str>>
{
    // begin met lijn lezen, als we wat zien, kijk welke index
    // afhankelijk van welke index, hoort hij bij die toren
    for ch in towers.lines()
    {
        	
    }
todo!();
}


fn is_string_numeric(str: &str) -> bool 
{
    for c in str.chars() 
    {
        if !c.is_numeric() 
        {
            return false;
        }
    }
    return true;
}

pub fn process_line(line: &str, stacks: &Vec<&str>) -> ()
{
    let mut operation_to_perform = Vec::new(); 
    line.split_whitespace()
        .inspect(|element| 
            if is_string_numeric(element){
                operation_to_perform.push(element.parse::<i32>().unwrap());
            }
        );

    // eerste getal = hoeveel verplaatsen, tweede getal van, derde getal waar naartoe
    // begin is onderaan stack, eind is bovenaan stack
    for i in operation_to_perform[2]..=operation_to_perform[1]
    {
        
    }


    todo!();
} 

pub fn get_final_message(processed_stacks: &Vec<&str>) -> String
{
    //processed_stacks.iter().last()
    todo!();
}

pub fn day5_part1() -> String
{
    let reader = get_file_reader_for("\\inputs\\day5.txt");

    for line in reader.lines()
    {
        let line = line.unwrap(); // Ignore errors.


    }
    todo!();
}