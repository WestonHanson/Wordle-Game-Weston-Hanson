# Wordle-Game-Weston-Hanson

## Purpose

    To program a game that pushed me more than a simple tic-tac-toe game.
    This is a Wordle game, so it is a direct copy of the popular game Wordle. I wanted to make a copy but put my own twist on it.

## Discription

    The program runs like a normal Wordle game would. The user has six guesses to guess the correct word. Each time the user guesses a word that is found in the word list the board will update with that word. Then the program prints out the board, coloring each letter that is in the right place, is in the word but not the right place, or (and this is my twist on it) is in multiple places in the word.
    The user can play as many times as they want until they say 'no' to the prompt.

## Screenshots of the game:
![Alt text](https://github.com/WestonHanson/Wordle-Game-Weston-Hanson/assets/121981035/ffb3af8f-483a-4512-be62-a4cdd0f5e764)
    ![image](https://github.com/WestonHanson/Wordle-Game-Weston-Hanson/assets/121981035/00f83542-7ec0-4607-9102-fddf65c65c72)
    ![image](https://github.com/WestonHanson/Wordle-Game-Weston-Hanson/assets/121981035/edc95303-3117-48d1-9807-9f6a7dbd1296)


## What makes it harder than a tic-tac-toe game

    This project has a lot of string manipulation that a tic-tac-toe game just does not have. I also implement a binary search to search through a list of words. The way I get that list is by pulling it from a file and then separating it into a vector. The logic on Wordle is also fairly more difficult than a tic-tac-toe game is.

#Challenges
    The biggest challenge was working around the language constraints. Rust has very strict type-checking and only certain functions work on each type. For example, I originally had an array of &str for my Wordle board, but when I got to the fill_board() function I ran into many different problems like borrowing errors and different type errors. So, I had to switch my board to String, and that made me have to update my print_board() function. That did not solve the problem altogether but it made it easier. The project itself had some difficult logic issues, but for the most part, it was straight forward, but it was the language constraints that prevented me from moving on a lot of the time. 

## How to run the project

    You need to move into the src file, then run `cargo run` in the terminal. Retrieving the file will be done automatically by the program.

## Credit

    Copied from the game Wordle by Josh Wardle (https://www.nytimes.com/games/wordle/index.html). This project is a product of Weston Hanson. Please check the license for more information on how to use this code.
