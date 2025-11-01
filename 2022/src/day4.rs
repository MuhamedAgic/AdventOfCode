use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

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

pub fn get_sections(input_line: &str) -> ( (i32, i32), (i32, i32) )
{
    let sections: Vec<&str> = input_line.split(|c| c == ',' || c == '-').collect();
    if sections.len() == 4
    {
        return ( (i32::from_str(sections[0]).unwrap(), i32::from_str(sections[1]).unwrap()), 
                 (i32::from_str(sections[2]).unwrap(), i32::from_str(sections[3]).unwrap()) );
    }
    else 
    {
        println!("Niet 4 sections verkregen");
        return ((0, 0), (0, 0));
    }
}

pub fn fully_contains(pair1: &(i32, i32), pair2: &(i32, i32)) -> bool
{
    //pair2 bevat alles wat in pair 1 zit
    if pair1.0 >= pair2.0 && pair1.1 <= pair2.1
    {
        return true;
    }
    //pair1 bevat alles wat in pair 2 zit
    else if pair2.0 >= pair1.0 && pair2.1 <= pair1.1
    {
        return true;
    }
    else 
    {
        return false;
    }
}

pub fn day4_challenge1() -> i32
{
    let reader = get_file_reader_for("\\inputs\\day4.txt");

    let mut amount_fully_contains_other = 0;
    for line in reader.lines()
    {
        let line = line.unwrap(); // Ignore errors.

        let sections = get_sections(line.as_str());
        if fully_contains(&sections.0, &sections.1)
        {
            amount_fully_contains_other += 1;
        }
    }
    return amount_fully_contains_other;
}



// ====================== part 2 ==============================//


pub fn overlaps(pair1: &(i32, i32), pair2: &(i32, i32)) -> bool
{
    // de omgekeerde kant op, als waardes totaal niet overlappen, return false, anders true
    // 123......
    // ....567..
    // of
    // ......789
    // ..34.....
    if pair1.1 < pair2.0 || pair1.0 > pair2.1
    {
        return false;
    }
    else 
    {
        return true;
    }
}

pub fn day4_challenge2() -> i32
{
    let reader = get_file_reader_for("\\inputs\\day4.txt");

    let mut amount_overlapping = 0;
    for line in reader.lines()
    {
        let line = line.unwrap(); // Ignore errors.

        let sections = get_sections(line.as_str());
        if overlaps(&sections.0, &sections.1)
        {
            amount_overlapping += 1;
        }
    }
    return amount_overlapping;
}