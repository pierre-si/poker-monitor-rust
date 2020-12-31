use std::collections::LinkedList;
use crate::cards;

#[derive(Debug)]
pub struct Player {
    number: u32,
    cash: u32,
    turn_bet: u32,
    total_bet: u32,
    state: char,

    deck: cards::Deck,
    raises_history: [usize; 7] // nombre de raises effectués au cours du tour (0 que des check ou call), -1 fold (ajouté en plus de l'historique du tour) => taille max : 5
}

fn create_player(number: u32, start_cash: u32) -> Player {
    Player {
        number: number,
        cash: start_cash,
        turn_bet: 0,
        total_bet: 0,
        state: 'i',
        deck: cards::create_deck(2),
        raises_history: [0; 7]
    }
}

pub fn create_players(number_of_players: u32, start_cash: u32) -> Vec<Player> {
    if number_of_players < 2 {
        println!("Nombre de joueurs trop faible");
        std::process::exit(100);
    }

    let mut players: Vec<Player> = vec![];
    for i in 0..number_of_players {
        players.push(create_player(i, start_cash))
    }
    players
}