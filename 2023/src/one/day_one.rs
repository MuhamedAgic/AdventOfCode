use crate::utilities::filereader;
use itertools::izip;

fn min_index_of_vectors(vector1: &Vec<(i32, i32)>, vector2: &Vec<(i32, i32)>) -> i32 
{
    let zipped_vectors = izip!(vector1, vector2);
    let min_index = zipped_vectors.min_by_key(|(x, _)| x.1).unwrap().0;
    return min_index.1;
}

fn max_index_of_vectors(vector1: &Vec<(i32, i32)>, vector2: &Vec<(i32, i32)>) -> i32 
{
    let zipped_vectors = izip!(vector1, vector2);
    let max_index = zipped_vectors.max_by_key(|(x, _)| x.1).unwrap().0;
    return max_index.1;
}

fn get_vector_with_min_idx(vector1: &Vec<(i32, i32)>, vector2: &Vec<(i32, i32)>) -> (i32, i32) 
{
    let min_index1 = vector1.iter().min_by_key(|&(_, index)| index).unwrap().1;
    let min_index2 = vector2.iter().min_by_key(|&(_, index)| index).unwrap().1;
    
    if min_index1 < min_index2 
    {
        (0, min_index1)
    }
    else 
    {
        (1, min_index2)
    }
}

fn get_vector_with_max_idx(vector1: &Vec<(i32, i32)>, vector2: &Vec<(i32, i32)>) -> (i32, i32) 
{
    let max_index1 = vector1.iter().max_by_key(|&(_, index)| index).unwrap().1;
    let max_index2 = vector2.iter().max_by_key(|&(_, index)| index).unwrap().1;
    
    if max_index1 > max_index2 
    {
        (0, max_index1)
    }
    else 
    {
        (1, max_index2)
    }
}

fn get_num_from_line(line: &str) -> Option<i32>
{
    // loop chars, eerste getal en laatste combineren bij één getal, die dupliceren
    // Alle chars die ints zijn in één lijst
    let mut nums: Vec<char> = Vec::new();
    
    for c in line.chars()
    {
        if c.is_digit(10) // 10 based digit
        {
            nums.push(c);
        }
    }

    // duplicate single numbers
    if nums.is_empty()
    {
        return None;
    }
    else if nums.len() == 1
    {
        nums.push(nums.get(0).unwrap().to_owned());
    }

    // take first and last number
    let first = &nums.get(0).unwrap().to_string();
    let last = &nums.get(nums.len() - 1).unwrap().to_string();

    let number = (first.to_owned() + (*last).as_str()).parse::<i32>().unwrap();

    return Some(number);
}

fn string_to_int(s: &str) -> Option<i32>
{
    if s == "one" { return Some(1); }
    if s == "two" { return Some(2); }
    if s == "three" { return Some(3); }
    if s == "four" { return Some(4); }
    if s == "five" { return Some(5); }
    if s == "six" { return Some(6); }
    if s == "seven" { return Some(7); }
    if s == "eight" { return Some(8); }
    if s == "nine" { return Some(9); }
    else { return None; }
}

fn get_str_digits(line: &str, list_nums: &mut Vec<(i32, i32)>) -> ()
{
    // check digits
    for (idx, c) in line.char_indices()
    {
        if c.is_digit(10) // 10 based digit
        {
            list_nums.push((c.to_digit(10).unwrap() as i32, idx as i32));
        }
    }
}

fn get_number_substrings(line: &str, list_nums: &mut Vec<(i32, i32)>, str_numbers: [&str;9]) -> ()
{
    for str_num in str_numbers
    {
        let found_indexes: Vec<usize> = line.match_indices(str_num)
                                            .map(|(index, _)| index)
                                            .collect();
        for found_idx in found_indexes
        {
            list_nums.push((string_to_int(str_num).unwrap(), found_idx as i32))
        }
    } 
}

fn get_num_from_line_v2(line: &str) -> Option<i32>
{    
    let str_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut int_nums: Vec<(i32, i32)> = Vec::new();
    let mut string_nums_as_int: Vec<(i32, i32)> = Vec::new();

    get_str_digits(line, &mut int_nums);
    get_number_substrings(line, &mut string_nums_as_int, str_numbers);

    // gosh this feels so ugly...

    // duplicate single numbers
    if int_nums.is_empty() && string_nums_as_int.is_empty()
    {
        return None;
    }
    else if int_nums.len() == 1 && string_nums_as_int.is_empty()
    {
        let num_str = int_nums.get(0).unwrap().0.to_owned();
        let number = format!("{}{}", num_str, num_str).parse::<i32>().unwrap();
        return Some(number);
    }
    else if int_nums.is_empty() && string_nums_as_int.len() == 1
    {
        let num = string_nums_as_int.get(0).unwrap().0.to_owned();
        let number = format!("{}{}", num.to_string(), num).parse::<i32>().unwrap();
        return Some(number);
    }
    // when one is empty and the other contains more than one
    else if int_nums.is_empty() && string_nums_as_int.len() > 1
    {
        let first = &string_nums_as_int.get(0).unwrap().0.to_string();
        let last = &string_nums_as_int.get(string_nums_as_int.len() - 1).unwrap().0.to_string();

        let number = (first.to_owned() + (*last).as_str()).parse::<i32>().unwrap();

        return Some(number);
    }
    else if string_nums_as_int.is_empty() && int_nums.len() > 1
    {
        let first = &int_nums.get(0).unwrap().0.to_string();
        let last = &int_nums.get(int_nums.len() - 1).unwrap().0.to_string();

        let number = (first.to_owned() + (*last).as_str()).parse::<i32>().unwrap();

        return Some(number);
    }

    // take min and max index from lists
    let mut lists_with_nums_combined = Vec::new();
    lists_with_nums_combined.push(&int_nums);
    lists_with_nums_combined.push(&string_nums_as_int);
    
    let listnr_min_and_idx = get_vector_with_min_idx(&int_nums, &string_nums_as_int);
    let listnr_max_and_idx = get_vector_with_max_idx(&int_nums, &string_nums_as_int);

    // from the chosen vec, get the max/min tuple pair on index
    let first = &lists_with_nums_combined.get(listnr_min_and_idx.0 as usize)
                                               .unwrap()// De lijst met het eerste getal
                                               .iter()// de waarde die hoort bij de eerste index
                                               .min_by_key(|x| x.1)
                                               .unwrap().0;// .0, want (waarde, index)
    let last = &lists_with_nums_combined.get(listnr_max_and_idx.0 as usize)
                                              .unwrap()// De lijst met het laaste getal
                                              .iter()// de waarde die hoort bij de laatste index
                                              .max_by_key(|x| x.1)
                                              .unwrap().0;// .0, want (waarde, index)

    let number = (first.to_string() + last.to_string().as_str()).parse::<i32>().unwrap();

    return Some(number);
}

pub fn day_one_part_one(filename: &str) -> i32
{
    let lines = filereader::read_lines(filename).unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for line in lines
    {
        let s = line.as_ref().unwrap().as_str();
        let num = get_num_from_line(s);

        if let Some(value) = num
        {
            nums.push(value);
        }
        else 
        {
            println!("No numbers in this line!");
        }
    }
    
    return nums.into_iter().sum();
}

pub fn day_one_part_two(filename: &str) -> i32
{
    let lines = filereader::read_lines(filename).unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for line in lines
    {
        let s = line.as_ref().unwrap().as_str();
        let num = get_num_from_line_v2(s);

        if let Some(value) = num
        {
            nums.push(value);
        }
        else 
        {
            println!("No numbers in this line!");
        }
    }
    
    return nums.into_iter().sum();
}