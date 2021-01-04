use std::io;
use crate::players;
use crate::cards;

//TODO: utiliser get(number) suivi d'un match sur le Optional renvoyé.
pub fn ask_player_number(players: &Vec<players::Player>) -> usize {
    let mut input = String::new();
    let mut number: usize;
    loop{
        input.clear();
        io::stdin().read_line(&mut input).expect("failed to read line");
        number = input.trim().parse().expect("Please type a number!");
        if number < players.len() { break; }
        else { println!("Please enter a valid player number")}
    }
    return number;
}

pub fn ask_cards(deck: &mut cards::Deck, n_cards: usize) {
    let mut input = String::new();

    for i in deck.known_cards_number..deck.known_cards_number+n_cards {
        loop {
            println!("CARD {} VALUE: ", i+1);
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let ch = input.chars().next().unwrap();
            let digit = ch.to_digit(10);
            match digit {
                Some(1) => deck.values[i] = 14,
                Some(d) => deck.values[i] = d,
                None => match ch {
                    't' | 'T' => deck.values[i] = 10,
                    'j' | 'J' => deck.values[i] = 11,
                    'q' | 'Q' => deck.values[i] = 12,
                    'k' | 'K' => deck.values[i] = 13,
                    'a' | 'A' => deck.values[i] = 14,
                    'c' | 'C' => if i == deck.known_cards_number { return },
                    _ => continue,
                }
            }
            break;
        }
        loop {
            println!("CARD {} SUIT: ", i+1);
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let ch = input.chars().next().unwrap();
            match ch {
                'c' | 'C' => deck.suits[i] = 1,
                'd' | 'D' => deck.suits[i] = 2,
                'h' | 'H' => deck.suits[i] = 3,
                's' | 'S' => deck.suits[i] = 4,
                _ => continue,
            }
            break; 
        }
    }
    deck.known_cards_number += n_cards;
    if deck.known_cards_number > deck.cards_number { deck.known_cards_number = deck.cards_number }
}