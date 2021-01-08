use std::io;
use crate::cards;

// on ne manipule plus de liste chainée de joueurs (contrairement à la version C), mais un vector (tableau) de joueurs
// TODO: process::exit plutôt que std::process::exit
#[derive(Debug)]
pub struct Player {
    pub number: u32,
    pub cash: u32,
    pub round_bet: u32,
    pub total_bet: u32,
    pub state: char, // i : in (en jeu, actif) ; f : fold (passif); a : all-in (passif); o : out (hors-jeu);

    pub hand: cards::Hand,
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
            hand: cards::Hand::new(2),
            raises_history: [0; 7]
        }
    }

    pub fn action(&mut self, action: char, pot: &mut u32, to_bet: &mut u32, raise_value: &mut u32) {
        match action {
            // CHECK or CALL
            'c' => if self.round_bet < *to_bet {
                println!("CALL {}", *to_bet - self.round_bet);
                *pot += self.make_bet(*to_bet - self.round_bet);
			}
			else{
				println!("CHECK\n");
            }
            // RAISE (predefined BET)
            'r' => {
                println!("BET {} ; RAISE {}", *to_bet+*raise_value-self.round_bet, *raise_value);
    			*pot += self.make_bet(*to_bet+*raise_value-self.round_bet);
                *to_bet += *raise_value;
            }
            // BET (custom BET)
		    'b' => {
                println!("AMOUNT ? ");
                let mut input = String::new();
			    input.clear();
                io::stdin().read_line(&mut input).expect("failed to read line");
                let bet_value = input.trim().parse().expect("Please type a number!");

                // il doit y avoir un bogue, le montant réellement misé peut être inférieur au bet annoncé si l'utilisateur ne dispose pas assez d'argent
                *raise_value = bet_value - (*to_bet - self.round_bet);
                *to_bet += *raise_value;
                *pot += self.make_bet(bet_value);
				println!("BET {} ; RAISE {}", bet_value, *raise_value);
            }
            // FOLD
            'f' => {
                self.state = 'f';
                println!("FOLD");
            }
            // ALL-IN
		    'a' => {
                if self.cash + self.round_bet > *to_bet {
                    *raise_value = self.cash - (*to_bet - self.round_bet);
                    *to_bet = self.cash + self.round_bet;
                    println!("ALL-IN {} (ALL-IN RAISE {})", self.cash, *raise_value);
                } else {
                    println!("ALL-IN {}", self.cash);
                }
                *pot += self.all_in();
            } 

            _ => panic!("wrong action given to player!")
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

pub fn reset_round(players: &mut Vec<Player>) {
    for player in players {
        player.total_bet += player.round_bet;
        player.round_bet = 0;
    }
}
pub fn reset_hand(players: &mut Vec<Player>) {
    for player in players {
        player.total_bet = 0;
        player.hand.reset_cards();
        if player.cash <= 0 {
            player.cash = 0;
            player.state = 'o';
        } else {
            player.state = 'i';
        }
    }
}

pub fn available_pot_amount(players: &Vec<Player>, initial: usize) -> u32 {
    let mut amount = 0;
    for player in players {
        amount += if player.total_bet > players[initial].total_bet {players[initial].total_bet} else { player.total_bet };
    }
    amount
}