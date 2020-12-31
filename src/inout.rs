use std::io;
use crate::players;

pub fn ask_player_number(players: &Vec<players::Player>) -> usize {
    let mut input = String::new();
    let mut number: usize;
    loop{
        io::stdin().read_line(&mut input).expect("failed to read line");
        number = input.trim().parse().expect("Please type a number!");
        if number < players.len() { break; }
        else { println!("Please enter a valid player number")}
    }
    return number;
}