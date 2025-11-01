use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
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

fn is_string_numeric(str: &String) -> bool 
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


pub fn update_dir_structure(dir_structure: &mut HashMap<String, Vec<String>>, 
                            current_dir: &String,
                            output_ls_command: &Vec<String>) -> ()
{
    for line in output_ls_command.iter()
    {
        if is_string_numeric(&line.split_whitespace().nth(0).unwrap().to_owned())
        {
            // dan heb je een file met een bepaalde size
        }
        else if line.split_whitespace().nth(0).unwrap().starts_with("dir")
        {
            //dir_structure.index(&current_dir).push(line.split_whitespace().nth(1).unwrap().to_owned())
        }
    }

    todo!();
}


pub fn get_total_size_of_directory() -> i32
{
 todo!();
}


pub fn day7_part1() -> i32
{
    /*
    //let reader = get_file_reader_for("\\inputs\\day7.txt");

    //let mut directory_structure: HashMap<String, Vec<String>> = HashMap::new();



    //let mut start_grabbing_command_output = false;

    // regel voor regel
    //let mut output_lines_from_command = Vec::new();

    for line in reader.lines() // Ignore errors.
    {

        if !&line.unwrap().clone().starts_with("$")
        {
            // dan is dit de output van een commando
            output_lines_from_command.push(line.unwrap().clone());
            continue;
        }
        else
        {
            // eerst output analyseren, dan legen
            output_lines_from_command.clear();
            if line.unwrap().contains("cd")
            {
                // kijk welke dir het is en sla die op, daarna moeten ook
            }
            else if line.unwrap().contains("ls")
            {
                // zelfde voor ls
    
            }

        }
        

    }
*/

    todo!();
}