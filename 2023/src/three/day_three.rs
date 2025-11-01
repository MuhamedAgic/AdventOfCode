use std::{fs::File, io::{BufReader, BufRead}, path::Path};

use crate::filereader;


fn is_adjacent(line: &str, start_idx: i32, number_length: i32) -> bool
{
    let min_bound = start_idx - 1;
    let max_bound = start_idx + number_length;
    for (idx, c) in line.char_indices()
    {
        if c != '.'  && !c.is_digit(10) 
        {
            // ...|*....|...
            // ...|.123(|...
            // ...|....&|...
            // Alles tussen de lijnen geldig
            if idx as i32 >= min_bound && idx as i32 <= max_bound // misschien + 1
            {
                //println!("Special char! {}", c);
                return true;
            }
        }
    }
    return false;
}

fn is_symbol_around(lines: &mut Vec<String>, line_cnt: i32, line_nr: i32, start_idx: i32, number_str: &str) -> bool
{
    let number_length = number_str.len();

    //println!("BGD LINECOUNT {}", line_cnt);
    //println!("BGD lnnr {}", line_nr);

    // regel 4: 777 niet gepakt

    if line_nr == 0
    {
        //print!("{}, ", line_nr);
        // check only below
        let current_line = lines.get(line_nr as usize).clone().unwrap();
        let line_below = lines.get(line_nr as usize + 1).clone().unwrap();
        return is_adjacent(&current_line, start_idx, number_length as i32) || is_adjacent(&line_below, start_idx, number_length as i32)
    }
    else if line_nr == line_cnt - 1
    {
        //print!("{}, ", line_nr);
        // check only above
        let line_above = lines.get(line_nr as usize - 1).unwrap();
        let current_line = lines.get(line_nr as usize).clone().unwrap();
        return is_adjacent(&line_above, start_idx, number_length as i32) || is_adjacent(&current_line, start_idx, number_length as i32);
    }
    else if line_nr > 0 && line_nr < line_cnt
    {
        //print!("{}, ", line_nr);
        // check under and above
        let line_above = lines.get(line_nr as usize - 1).unwrap();
        let current_line = lines.get(line_nr as usize).unwrap();
        let line_below = lines.get(line_nr as usize + 1).unwrap();

        return is_adjacent(line_below, start_idx, number_length as i32)
        || is_adjacent(current_line, start_idx, number_length as i32)
        || is_adjacent(line_above, start_idx, number_length as i32);
    }
    else 
    {
        println!("Invalid linenr {}", line_nr);
        return false;
    }
}

fn find_part_numbers_w_idx(filename: &str) -> Vec<(i32, usize)>
{
    let mut lines_orig = filereader::read_lines(filename).unwrap();
    //let mut lines_ref = filereader::read_lines(filename).unwrap();
    //let mut lines_ref2 = filereader::read_lines(filename).unwrap();

    let file = File::open(Path::new(filename)).unwrap();
    let file2 = File::open(Path::new(filename)).unwrap();
    let reader = BufReader::new(&file);
    let reader2 = BufReader::new(&file2);
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut lines2: Vec<_> = reader2.lines().collect::<Result<_, _>>().unwrap();

    //println!("LINES {:?}", lines2);

    let line_count = lines2.len() as i32;

    //println!("BGD LINECOUNT {}", line_count);

    let mut result = Vec::new();
    let mut current_number_str = String::new();

    let mut current_line_nr: i32 = 0;

    let mut last_idx = 0;

    let mut line_cntr = 0;

    let mut result_sum = 0;
    for line in lines
    {
        line_cntr += 1;
        let mut result_line = Vec::new();
        for (idx, c) in line.char_indices()
        {
            if let Some(digit) = c.to_digit(10) 
            {
                // sowieso pushen
                current_number_str.push(c);
            } 
            else
            {
                if !current_number_str.is_empty() 
                {
                    //result.push((current_number_str.parse::<i32>().unwrap(), idx));
                    let current_number = current_number_str.parse::<i32>().unwrap();

                    // check around
                    
    	            let start_num_idx = idx as i32 - current_number_str.len() as i32;
                    //println!("n start idx: {}", start_num_idx);

                    // check per char eromheen
                    if is_symbol_around(&mut lines2, line_count, current_line_nr, start_num_idx, current_number_str.as_str())
                    {
                        result.push((current_number, start_num_idx as usize));
                        result_line.push((current_number, start_num_idx as usize));
                        result_sum += current_number;
                    }
                    current_number_str.clear();
                }
            }
            last_idx = idx;
        }
        
        println!("Line {} {:?}\n", line_cntr, result_line);
        // if !current_number_str.is_empty() 
        // {
        //     result.push((current_number_str.parse::<i32>().unwrap(), last_idx));
        // }
        current_line_nr += 1;
    }

    // 558036 te laag
    // 560570 te laag

    //println!("{:?}", result);
    println!("{:?}", result_sum);
    return result;

    
}

pub fn day_three_part_one(filename: &str) -> i32
{
    let output: Vec<(i32, usize)> = find_part_numbers_w_idx(filename);
    let answer = output.iter().map(|&(value, _)| value).sum();
    return answer;
}