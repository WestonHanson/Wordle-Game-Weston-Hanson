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

    let mut play_again = true; //Boolean for loop

    println!("\n\nDo you want to play Wordle (yes/no)?");

    while play_again { //Main loop to keep game going

        let mut user_play_again = String::new(); //Var for users responce

        io::stdin().read_line(&mut user_play_again).expect("Failed to read input"); //Reads input from user
    
        if user_play_again.trim().to_lowercase().as_str() == "yes" { //If user puts 'yes' then print rules (trims it because it will have a \n or \r)
            println!("");
            println!("Rules:");
            println!("\tUse 5 letter words, but not every word is in the list.");
            println!("\t{}green {} means the letter is in the correct spot.", color::Bg(color::Green), color::Bg(color::Reset));
            println!("\t{}yellow {} means the letter is in the word but not in the correct spot.", color::Bg(color::Yellow), color::Bg(color::Reset));
            println!("\t{}blue {} mean the letter is in the word more than once.\n", color::Bg(color::Blue), color::Bg(color::Reset));
        } else if user_play_again.trim().to_lowercase().as_str() != "yes" && user_play_again.trim().to_lowercase().as_str() != "no"{ //If user doenst input 'yes' or 'no' then they can try again
            println!("\nPlease try again.\n");
            continue; 
        } else { //If the user inputs 'no' then the game ends
            play_again = false;
            continue;
        }

        let file_name = "words.txt"; //File name
        let contents = fs::read_to_string(file_name).unwrap(); //Grabs the contents of the file and unwraps it
        let mut word_list = Vec::new(); //Creates new vector for words 
        let mut random_word = String::new();

        for line in contents.lines() { //Loop to split each element in contents into its own element in a vector
            let elements: Vec<String> = line.split(',').map(String::from).collect();
            word_list.extend(elements);
        }

        let mut range = rand::thread_rng();
        let random_num = range.gen_range(0..488); //Generates a random number from 0 to 487 becsaue there are 488 words
        let Some(word) = word_list.get(random_num) else { todo!() }; //Takes a word from the vector at the random num index
        random_word = word.clone(); //Clones word so it wont be barrowed somewhere

        let mut game = false; //Boolean to see if the specific game is going on
        let mut word_match = false; //Boolean to see if user guessed the right word
        let mut wordle_board: [[String; 5]; 6] = [
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
            ["_".to_string(), "_".to_string(), "_".to_string(), "_".to_string(), "_".to_string()],
        ];

        let mut wordle_board_counter = 0; //Counter to keep track of which row is updating

        while !game{ //Loop to keep specific game going 

            print_board(&wordle_board);

            println!("Guess a word:");

            let mut user_input = String::new();

            io::stdin().read_line(&mut user_input).expect("Failed to read input"); //Reads users input
            
            user_input = user_input.to_lowercase(); //Makes it lowercase

            println!("");

            if binary_search(&word_list, &user_input) == true { //Searches through the list to see if users guess is in the list, if it is it will fill the board with that guess
                fill_board(&mut wordle_board, &user_input, random_word.clone(), wordle_board_counter); //Fills in board with guess
                wordle_board_counter += 1; //Updetes counter
            } else{ //Else the word is not in the list so it just continues
                println!("Word is not in word list.\n");
                continue; 
            }

            if user_input.trim() == random_word.to_lowercase() || user_input.trim() == random_word { //If the user guessed the right word then the game ends
                game = true; //Breaks loop
                word_match = true; //Prints win option
            }

            if wordle_board_counter > 5 { //If the user has filled all the spots on the board the game ends
                game = true; //Breaks loop
                word_match = false; //Prints lose option 
            }

        }

        print_board(&wordle_board);

        if word_match { //User guessed right word
            println!("Good Job on guessing {}!\n", random_word);
        } else { //User did not guess right word, so it tells user what the word was
            println!("The correct word was {}.\n", random_word);
        }
        
        println!("\nDo you want to play again?"); //Promts to see if user want to play again
    
    }
}


/**
 * print_board: Function to print out wordle board and color letters that need coloring.
 * Elements marked with '!' are in the right spot.
 * Elements marked with '?' are in the word but not the right spot.
 * Elements marked with '#' are muliple places in the word.
 * @param: 5 by 6 array.
 * Void function -> only prints board.
 */
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

/**
 * binary_search: Function to search through sorted vector of words to see if users word in in the list.
 * Uses standard binary search techniques to find a word that matches.
 * @param: vector of stings.
 * @return: bool -> true if word is found, false if not found.
 */
fn binary_search(word_list: &Vec<String>, user_input: &String) -> bool {

    let user_input_trimmed = user_input.trim_end_matches(|c| c == '\r' || c == '\n'); //Trims off \r or \n 

    let mut low = 0;
    let mut high = word_list.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        let word_in_list = &word_list[mid].to_lowercase(); //Takes element at 'middle' reletive to high and low 

        if user_input_trimmed == word_in_list { //If word is found return true
            return true;
        }

        if user_input_trimmed < word_in_list { //If user word is less than word in middle then the high shifts to one less than the middle
            high = mid - 1;
        }

        if user_input_trimmed > word_in_list { //If user word is greater than word in middle then the low shifts to one more than the middle
            low = mid + 1;
        }
    }

    return false; //If word is not found then return false
}

/**
 * fill_board: Function to fill out wordle board with users word and mark each letter that is found in the word with correct symbol.
 * Elements marked with '!' are in the right spot.
 * Elements marked with '?' are in the word but not the right spot.
 * Elements marked with '#' are muliple places in the word.
 * @param: 5 by 6 array.
 * @param: user input.
 * @param: word from list.
 * @param: row counter. 
 * Void function -> only updates board.
 */
fn fill_board(wordle_board: &mut [[String; 5]; 6], user_input: &str, random_word: String, board_counter: usize) {

    let mut user_counter = 0; //To take the correct index from random word and user input

    for element in &mut wordle_board[board_counter] { //Loops through board at specific index
        let letter = user_input.chars().nth(user_counter).unwrap_or('?'); //Grabs elemebt at 'user_counter' index
        let mut new_letter = letter.to_string(); //To minimize borrowing errors, turns letter from char to String
        
        //There might be a bug if user letter is in 2 places but the user letter doesnt line up with the random word letter, then it would just be yellow not blue
        if new_letter.as_str() == random_word.chars().nth(user_counter).unwrap_or('?').to_string().to_lowercase() { //If letter if from user input matches at the same index as the random word
            let mut user_input_string = user_input.to_string(); //To minimize borrowing errors
            let Some(index) = user_input.find(new_letter.as_str()) else { todo!() }; //Finds index of users letter
            let garbage = user_input_string.remove(index); //Removes users letter from word at 'index'
            if random_word.contains(&user_input_string){ //If the word still contains the letter then the letter is marked with '#'
                new_letter.push_str("#");
            } else { //Else it is marked with '!'
                new_letter.push_str("!");
            }

        } else if random_word.contains(new_letter.as_str()){ //If the letter is found somewhere else in the word it is marked with '?'
            new_letter.push_str("?");
        }
        *element = new_letter.clone(); //Updates element with new letter
        user_counter += 1; //Updates counter
    }
}

