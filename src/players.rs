use crate::cards;

// on ne manipule plus de liste chainée de joueurs (contrairement à la version C), mais un vector (tableau) de joueurs
// TODO: process::exit plutôt que std::process::exit
#[derive(Debug)]
pub struct Player {
    number: u32,
    cash: u32,
    pub round_bet: u32,
    total_bet: u32,
    pub state: char, // i : in (en jeu, actif) ; f : fold (passif); a : all-in (passif); o : out (hors-jeu);

    pub deck: cards::Deck,
    raises_history: [usize; 7] // nombre de raises effectués au cours du tour (0 que des check ou call), -1 fold (ajouté en plus de l'historique du tour) => taille max : 5
}

impl Player {
    pub fn new(number: u32, start_cash: u32) -> Player {
        Player {
            number: number,
            cash: start_cash,
            round_bet: 0,
            total_bet: 0,
            state: 'i',
            deck: cards::Deck::new(2),
            raises_history: [0; 7]
        }
    }

    pub fn make_bet(&mut self, mut amount: u32) -> u32 {
        if self.cash <= amount {
            println!("Player {} goes all-in", self.number);
            amount = self.all_in();
        } else {
            self.cash -= amount;
            self.round_bet += amount;
        }
        amount
    }

    pub fn all_in(&mut self) -> u32 {
        let amount = self.cash;
        self.state = 'a';
        self.cash = 0;
        self.round_bet += amount;

        amount
    }
}

pub fn create_players(number_of_players: u32, start_cash: u32) -> Vec<Player> {
    if number_of_players < 2 {
        println!("Nombre de joueurs trop faible");
        std::process::exit(100);
    }

    let mut players: Vec<Player> = vec![];
    for i in 0..number_of_players {
        players.push(Player::new(i, start_cash))
    }
    players
}

pub fn reset_round(players: &mut Vec<Player>) {}
pub fn reset_hand(players: &mut Vec<Player>) {}

pub fn active_players_count(players: &Vec<Player>) -> u32 {
    let mut number = 0;
    for player in players {
        if player.state == 'i' { number += 1 };
    }
    number
}

pub fn next_active_player(players: &Vec<Player>, starting_with: usize) -> usize {
    if starting_with >= players.len(){
        println!("Appel de next_active_player avec un indice de joueur trop grand !\n");
        std::process::exit(100);
    }
    let mut i: usize = starting_with;
    loop { // apparemment il n'est pas possible d'utiliser l'itérateur cycle sur un vector
        i += 1;
        if i == players.len() {i = 0}
        if players[i].state == 'i' || i == starting_with { break; }
    }
    return i
}
