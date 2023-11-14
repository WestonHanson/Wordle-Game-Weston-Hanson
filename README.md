# Wordle-Game-Weston-Hanson

## Purpose

    To program a game that pushed me more than a simple tic-tac-toe game.
    This is a Wordle game, so it is a direct copy of the popular game Wordle. I wanted to make a copy but put my own twist on it.

## Discription

    The program runs like a normal Wordle game would. The user has six guesses to guess the correct word. Each time the user guesses a word that is found in the word list the board will update with that word. Then the program prints out the board, coloring each letter that is in the right place, is in the word but not the right place, or (and this is my twist on it) is in multiple places in the word.
    The user can play as many times as they want until they say 'no' to the prompt.

## What makes it harder than a tic-tac-toe game

    This project has a lot of string manipulation that a tic-tac-toe game just does not have. I also implement a binary search to search through a list of words. The way I get that list is by pulling it from a file and then separating it into a vector. The logic on Wordle is also fairly more difficult than a tic-tac-toe game is.

## How to run the project

    You need to move into the src file, then run 'cargo run' in the terminal. Retrieving the file will be done automatically by the program.

## Credit

    Copied from the game Wordle by Josh Wardle (https://www.nytimes.com/games/wordle/index.html). This project is a product of Weston Hanson. Please check the license for more information on how to use this code.
