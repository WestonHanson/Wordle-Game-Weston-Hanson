#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fs::File;
use std::fs;
use std::env;
use std::io::Read;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::str;
use std::fmt;
use rand::Rng;
use termion::color;

fn main(){

    let mut play_again = true;

    println!("\n\nDo you want to play Wordle (yes/no)?");

    while play_again {

        let mut user_play_again = String::new();

        io::stdin().read_line(&mut user_play_again).expect("Failed to read input");
    
        if user_play_again.trim().to_lowercase().as_str() == "yes" {
            println!("");
        } else if user_play_again.trim().to_lowercase().as_str() != "yes" && user_play_again.trim().to_lowercase().as_str() != "no"{
            println!("\nPlease try again.\n");
            continue; 
        } else {
            play_again = false;
            continue;
        }

        let file_name = "words.txt";
        let contents = fs::read_to_string(file_name).unwrap();
        let mut word_list = Vec::new();
        let mut random_word = String::new();

        for line in contents.lines() {
            let elements: Vec<String> = line.split(',').map(String::from).collect();
            word_list.extend(elements);
        }

        let mut range = rand::thread_rng();
        let random_num = range.gen_range(0..488);
        let Some(word) = word_list.get(random_num) else { todo!() };
        random_word = word.clone();

        let mut game = false;
        let mut word_match = false;
        let mut wordle_board: [[String; 5]; 6] = [
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        ];

        let mut wordle_board_counter = 0;

        while !game{

            print_board(&wordle_board);

            // Prompt the user for input
            println!("Guess a word:");

            // Create a mutable String to store the user input
            let mut user_input = String::new();

            // Read the user input without explicit error handling
            io::stdin().read_line(&mut user_input).expect("Failed to read input");
            
            user_input = user_input.to_lowercase();

            println!("");

            if binary_search(&word_list, &user_input) == true {
                fill_board(&mut wordle_board, &user_input, random_word.clone(), wordle_board_counter);
                wordle_board_counter += 1;
            } else{
                println!("Word is not in word list.\n");
                continue; 
            }

            if user_input.trim() == random_word.to_lowercase() || user_input.trim() == random_word {
                game = true;
                word_match = true;
            }

            if wordle_board_counter > 5 {
                game = true;
                word_match = false;
            }

        }

        print_board(&wordle_board);

        if word_match {
            println!("Good Job on guessing {}!\n", random_word);
        } else {
            println!("The correct word was {}.\n", random_word);
        }
        
        println!("\nDo you want to play again?");
    
    }
}

fn print_board(wordle_board: &[[String; 5]; 6]){
    for row in wordle_board {
        for element in row {
            if element.contains("!"){
                let new_element: String = (*element.clone()).to_string();
                print!("{}{}{} ", color::Bg(color::Green), new_element.to_string().remove(0), color::Bg(color::Reset));
            } else if element.contains("?"){
                let new_element: String = (*element.clone()).to_string();
                print!("{}{}{} ", color::Bg(color::Yellow), new_element.to_string().remove(0), color::Bg(color::Reset));
            } else if element.contains("#"){
                let new_element: String = (*element.clone()).to_string();
                print!("{}{}{} ", color::Bg(color::Blue), new_element.to_string().remove(0), color::Bg(color::Reset));
            } else {
                print!("{} ", element);
            }
        }
        println!("");
    }
    println!("");
}

fn binary_search(word_list: &Vec<String>, user_input: &String) -> bool {

    let user_input_trimmed = user_input.trim_end_matches(|c| c == '\r' || c == '\n');

    let mut low = 0;
    let mut high = word_list.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        let word_in_list = &word_list[mid].to_lowercase();

        if user_input_trimmed == word_in_list {
            return true;
        }

        // Search values that are greater than val - to right of current mid_index
        if user_input_trimmed < word_in_list {
            high = mid - 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if user_input_trimmed > word_in_list {
            low = mid + 1;
        }
    }

    return false;
}

fn fill_board(wordle_board: &mut [[String; 5]; 6], user_input: &str, random_word: String, board_counter: usize) {

    let mut user_counter = 0;

    for element in &mut wordle_board[board_counter] {
        let letter = user_input.chars().nth(user_counter).unwrap_or('?');
        let mut new_letter = letter.to_string();
        
        if new_letter.as_str() == random_word.chars().nth(user_counter).unwrap_or('?').to_string().to_lowercase() {
            let mut user_input_string = user_input.to_string();
            let Some(index) = user_input.find(new_letter.as_str()) else { todo!() };
            let garbage = user_input_string.remove(index);
            if random_word.contains(&user_input_string){
                new_letter.push_str("#");
            } else {
                new_letter.push_str("!");
            }

        } else if random_word.contains(new_letter.as_str()){
            new_letter.push_str("?");
        }
        *element = new_letter.clone();
        user_counter += 1;
    }
}




// let word_in_list = &word_list[0];
// let mut second_char = word_in_list.chars().nth(counter).unwrap_or('?');
// println!("{}", second_char);
// counter += 1;
// second_char = word_in_list.chars().nth(counter).unwrap_or('?');
// println!("{}", second_char);
// let list_letter = word_in_list.chars().nth(counter).map(|c| c.to_ascii_lowercase());
// let user_letter = user_input.chars().nth(counter).map(|c| c.to_ascii_lowercase());