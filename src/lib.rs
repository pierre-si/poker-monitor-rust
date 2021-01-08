use crate::cards;
use crate::players;

pub struct Game {
    pub players: Vec<players::Player>,
    pub small_blind: u32,
    // hand variables
    pub hand_number: u32,
    pub pot: u32,
    pub table: cards::Hand,
    // round variables
    pub round_number: usize,
    pub to_bet: u32,
    pub raise_value: u32,
}

impl Game {
    pub fn new(players_count: u32, start_cash: u32, small_blind: u32) -> Game {
        Game {
            players: players::create_players(players_count, start_cash),
            small_blind,
            hand_number: 0,
            pot: 0,
            table: cards::Hand::new(5),
            round_number: 0,
            to_bet: 0,
            raise_value: 0,
        }
    }

    pub fn initialize_hand(&self) {
        self.hand_number += 1;
        self.pot = 0;
        self.table.reset_cards();
        self.round_number = 1;
    }
    
    pub fn initialize_round(&self) {
        self.round_number += 1;
        self.to_bet = 0;
        self.raise_value = 2*self.small_blind;
        for player in self.players {
            player.total_bet += player.round_bet;
            player.round_bet = 0;
        }
    }

    pub fn next_active_player(&self, starting_with: usize) -> usize {
        if starting_with >= self.players.len(){
            println!("Appel de next_active_player avec un indice de joueur trop grand !\n");
            std::process::exit(100);
        }
        let mut i: usize = starting_with;
        loop { // apparemment il n'est pas possible d'utiliser l'itérateur cycle sur un vector
            i += 1;
            if i == self.players.len() {i = 0}
            if self.players[i].state == 'i' || i == starting_with { break; }
        }
        return i
    }

    pub fn active_players_count(&self) -> u32 {
        let mut number = 0;
        for player in self.players {
            if player.state == 'i' { number += 1 }
        }
        number
    }

    pub fn qualified_players(&self) -> Vec<usize> {
        let mut qualified_players = Vec::new();
        for (i, player) in self.players.iter().enumerate() {
            if player.state == 'i' || player.state == 'a' { qualified_players.push(i) }
        }
        qualified_players 
    }
}
