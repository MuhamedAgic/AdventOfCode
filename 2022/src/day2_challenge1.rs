use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
pub enum GameResult 
{
    Win,
    Draw,
    Lose
}

pub fn is_valid_char(choice: &char, valid_chars: &Vec<char>) -> bool
{
    return valid_chars.contains(&choice);
}

// dit slaat op jou, niet de tegenstander
pub fn get_score_for_decision(my_choice: char) -> i32
{
    match my_choice 
    {
       'X' => 1,
       'Y' => 2,
       'Z' => 3,
       _ => 0 // ik heb even geen zin om dit correct met options af te handelen
    }
}

// dit slaat op jou, niet de tegenstander
pub fn get_score_for_game_result(game_result: GameResult) -> i32
{    
    match game_result 
    {
        GameResult::Win => 6,
        GameResult::Draw => 3,        
        GameResult::Lose => 0,
    }
}

// case win, draw_ en lose slaan op jou, niet de tegenstander
pub fn handle_current_game_part_1(opponents_choice: &char, my_choice: &char) -> Option<i32>
{
    match opponents_choice
    {
        'A' => match my_choice 
        {
            'X' => Some( get_score_for_decision('X') + get_score_for_game_result(GameResult::Draw) ),
            'Y' => Some( get_score_for_decision('Y') + get_score_for_game_result(GameResult::Win) ),
            'Z' => Some( get_score_for_decision('Z') + get_score_for_game_result(GameResult::Lose) ),
            _ => None
        },
        'B' => match my_choice 
        {
            'X' => Some( get_score_for_decision('X') + get_score_for_game_result(GameResult::Lose) ),
            'Y' => Some( get_score_for_decision('Y') + get_score_for_game_result(GameResult::Draw) ),
            'Z' => Some( get_score_for_decision('Z') + get_score_for_game_result(GameResult::Win) ),
            _ => None
        },
        'C' => match my_choice 
        {
            'X' => Some( get_score_for_decision('X') + get_score_for_game_result(GameResult::Win) ),
            'Y' => Some( get_score_for_decision('Y') + get_score_for_game_result(GameResult::Lose) ),
            'Z' => Some( get_score_for_decision('Z') + get_score_for_game_result(GameResult::Draw) ),
            _ => None
        },
        _ => None
    }

}

pub fn get_score_from_round(opponents_choice: &char, my_choice: &char) -> Option<i32>
{
    let valid_chars_for_me = vec!['X', 'Y', 'Z'];
    let valid_chars_for_opponent = vec!['A', 'B', 'C'];

    if !is_valid_char(&opponents_choice, &valid_chars_for_opponent)
    {
        return None;
    }

    if !is_valid_char(&my_choice, &valid_chars_for_me)
    {
        return None;
    }

    // PART 1 EN 2 AAN OF UIT ZETTEN
    //return handle_current_game_part_1(&opponents_choice, &my_choice);
    return handle_current_game_part_2(&opponents_choice, &my_choice);
}

pub fn day2_challenge1() -> i32
{
    // Lees bestand uit
    let current_dir = std::env::current_dir().unwrap()
                                                     .to_str()
                                                     .unwrap()
                                                     .to_owned();

    let path_to_input = current_dir + "\\inputs\\day2_challenge1.txt";

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path_to_input).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for (index, line) in reader.lines().enumerate() 
    {
        let line = line.unwrap(); // Ignore errors.

        let opponents_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();

        total_score += get_score_from_round(&opponents_choice, &my_choice).unwrap();
    }
    return total_score;
}




//=============================== deel 2 =======================================//
pub fn get_game_result_to_be(game_result: &char) -> Option<GameResult>
{    
    match game_result 
    {
        'X' => Some(GameResult::Lose),
        'Y' => Some(GameResult::Draw),        
        'Z' => Some(GameResult::Win),
        _ => None
    }
}

// case win, draw_ en lose slaan op jou, niet de tegenstander
// Dit is een smerige hack, ik zou het mooier willen doen met enums, die XYZ, maar het werkt :)
pub fn handle_current_game_part_2(opponents_choice: &char, game_result_to_be: &char) -> Option<i32>
{
    let game_result_to_attain = get_game_result_to_be(game_result_to_be).unwrap();
    match opponents_choice
    {
        'A' => match game_result_to_attain 
        {
            GameResult::Win => Some( get_score_for_decision('Y') + get_score_for_game_result(GameResult::Win) ),
            GameResult::Draw => Some( get_score_for_decision('X') + get_score_for_game_result(GameResult::Draw) ),
            GameResult::Lose => Some( get_score_for_decision('Z') + get_score_for_game_result(GameResult::Lose) ),
            _ => None
        },
        'B' => match game_result_to_attain 
        {
            GameResult::Win => Some( get_score_for_decision('Z') + get_score_for_game_result(GameResult::Win) ),
            GameResult::Draw => Some( get_score_for_decision('Y') + get_score_for_game_result(GameResult::Draw) ),
            GameResult::Lose => Some( get_score_for_decision('X') + get_score_for_game_result(GameResult::Lose) ),
            _ => None
        },
        'C' => match game_result_to_attain 
        {
            GameResult::Win => Some( get_score_for_decision('X') + get_score_for_game_result(GameResult::Win) ),
            GameResult::Draw => Some( get_score_for_decision('Z') + get_score_for_game_result(GameResult::Draw) ),
            GameResult::Lose => Some( get_score_for_decision('Y') + get_score_for_game_result(GameResult::Lose) ),
            _ => None
        },
        _ => None
    }
}
