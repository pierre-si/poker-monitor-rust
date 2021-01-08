mod inout;
mod cards;
mod players;
mod combinations;

pub use crate::inout::*;
pub use crate::cards::*;
pub use crate::players::*;
pub use crate::combinations::*;

pub struct Game {
    players: Vec<players::Player>,
    small_blind: u32,
    blinds_raise_interval: u32,
    N_CARDS_TO_DEAL: [usize; 4],
    // hand variables
    dealer_index: usize,
    small_blind_index: usize,
    big_blind_index: usize,
    hand_number: u32,
    pot: u32,
    table: cards::Hand,
    // round variables
    round_number: usize,
    to_bet: u32,
    raise_value: u32,
}

impl Game {
    pub fn new(players_count: u32, start_cash: u32, small_blind: u32, blinds_raise_interval: u32) -> Game {
        println!("Vous êtes le joueur n°0, le dealer a le numéro : ");
        let dealer_index = inout::ask_player_number(players_count);
        let small_blind_index = if dealer_index + 1 < players_count as usize { dealer_index + 1 } else { 0 };
        let big_blind_index = if small_blind_index + 1 < players_count as usize { small_blind_index + 1 } else { 0 };
        Game {
            players: players::create_players(players_count, start_cash),
            small_blind,
            blinds_raise_interval,
            N_CARDS_TO_DEAL: [0, 3, 1, 1],

            dealer_index: dealer_index, 
            small_blind_index: small_blind_index, 
            big_blind_index: big_blind_index,

            hand_number: 0,
            pot: 0,
            table: cards::Hand::new(5),
            round_number: 0,
            to_bet: 0,
            raise_value: 0,
        }
    }

    pub fn run(&mut self) {
        while self.active_players_count() > 1 { // same game
            println!("*** Main numéro {} Préparation   ***", self.hand_number+1);
            let mut current_player: usize;
            current_player = self.initialize_hand();
            let (mut player_n, mut min_players_count): (u32, u32);
            loop{ // same hand
                println!("\n*** Main numéro {:2}  Tour numéro {} ***", self.hand_number, self.round_number);
                inout::ask_cards(&mut self.table, self.N_CARDS_TO_DEAL[self.round_number-1]);
                
                min_players_count = self.active_players_count();
                player_n = 0;
                loop{ // same round
                    println!("\nPOT  {:5}  REQUIS {:5}  RAISE {:5}", self.pot, self.to_bet, self.raise_value);
                    let action = inout::ask_action(&self.players, current_player, self.to_bet, self.raise_value);
                    self.players[current_player].action(action, &mut self.pot, &mut self.to_bet, &mut self.raise_value);
                    current_player = self.next_active_player(current_player);
                    player_n += 1;
                    // les joueurs actifs au début du tour doivent jouer au moins une fois
                    if self.players[current_player].state != 'i' || (player_n >= min_players_count && self.players[current_player].round_bet == self.to_bet) {
                        break;
                    }
                }
                current_player = self.finalize_round();
                if self.active_players_count() <= 1 || self.round_number >= 5 { break; } 
            }
            self.distribute_pot();
            self.check_players();
            self.rotate_buttons();
        }
    }

    pub fn initialize_hand(&mut self) -> usize {
        self.hand_number += 1;
        if self.hand_number % self.blinds_raise_interval == 0 { self.small_blind *= 2; }
        self.pot = 0;
        self.table.reset_cards();
        self.round_number = 1;

        if self.players[0].state == 'i' {
            println!("Vos cartes :");
            inout::ask_cards(&mut self.players[0].hand, 2);
        }
        // lorsqu'il n'y a plus que 2 joueurs en lice (« Heads-up »), le dealer est small blind
		// il est nécessaire de recalculer psmallBlind au cas où le tour précédent comptait 3 joueurs et s'est achevé par
		// la défaite du big blind.
        if self.active_players_count() == 2 {
            self.small_blind_index = self.next_active_player(self.big_blind_index);
            self.dealer_index = self.small_blind_index;
        }

        if self.players[self.small_blind_index].state != 'o' {
			println!("SMALL BLIND : JOUEUR {}   BET {:5}\n", self.small_blind_index, self.small_blind);
            self.pot += self.players[self.small_blind_index].make_bet(self.small_blind);
        }
        println!("BIG BLIND   : JOUEUR {}   BET {:5}\n", self.big_blind_index, self.small_blind*2);
		self.pot += self.players[self.big_blind_index].make_bet(self.small_blind*2);

        self.to_bet = self.small_blind*2;
        self.raise_value = self.small_blind*2;

        self.next_active_player(self.big_blind_index)
    }
    
    pub fn finalize_round(&mut self) -> usize {
        self.round_number += 1;
        for player in &mut self.players {
            player.total_bet += player.round_bet;
            player.round_bet = 0;
        }
        self.to_bet = 0;
        self.raise_value = 2*self.small_blind;
        // le premier joueur actif après le dealer commence le tour suivant
        self.next_active_player(self.dealer_index)
    }

    fn check_players(&mut self) {
        // reset players' hands and check cash
        for player in &mut self.players {
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

    pub fn rotate_buttons(&mut self) {
        self.dealer_index = self.small_blind_index;
        // ne correspond pas à PokerTH: si celui qui était bigblind meurt il ne devient pas small dans PokerTH
        self.small_blind_index = self.big_blind_index;
        self.big_blind_index = self.next_active_player(self.big_blind_index);
    }

    fn distribute_pot(&mut self) {
        let qualified_players = self.qualified_players(); 

        if qualified_players.len() == 1 {
            println!("JOUEUR {} REMPORTE {}", qualified_players[0], self.pot);
            self.players[qualified_players[0]].cash += self.pot;
        } else {
            println!("*** ABATTAGE ***");

            for j in qualified_players.iter() {
                println!("Joueur {} :", j);
                if !inout::ask_cards(&mut self.players[*j].hand, 2) { self.players[*j].state = 'f'; }
                inout::print_cards(&self.players[*j].hand);
            }

            let to_ask = self.table.cards_number - self.table.values.len();
            inout::ask_cards(&mut self.table, to_ask); 
            // tant qu'il y a de l'argent à distribuer on cherche des gagnants
            let mut distributed_amount = 0;
            let mut to_distribute;
            let mut winners = vec![];
            let mut winners_combinations = [0; 6];

            let mut player_combination = [0; 6];
            while distributed_amount < self.pot {
                winners_combinations[0] = 0;
                for j in qualified_players.iter() {
                    let mut player_cards = cards::merge_hands(&self.table, &self.players[*j].hand);
                    inout::print_cards(&player_cards);
                    combinations::combination_type(&mut player_cards, &mut player_combination);
                    println!("Player {}:", *j);
                    inout::print_combination(&player_combination);
                    if combinations::compare_combinations(&player_combination, &winners_combinations) == 1 {
                        winners = vec![*j];
                        // nouvel appel à combination_type probablement inutile…
                        combinations::combination_type(&mut player_cards, &mut winners_combinations);
                    } else if combinations::compare_combinations(&player_combination, &winners_combinations) == 2 {
                        winners.push(*j);
                    }
                }
                winners.sort_by_key(|k| self.players[*k].total_bet);
                let mut i = 0;
                while i < winners.len() {
                    to_distribute = self.available_pot_amount(self.players[winners[i]].number as usize) - distributed_amount;
                    if to_distribute > 0 {
                        for j in i..winners.len() {
                            println!("Joueur {} REMPORTE {}", self.players[winners[j]].number, to_distribute / ((winners.len() - i) as u32));
                            self.players[winners[j]].cash += to_distribute / ((winners.len() - i) as u32);
                        }
                        if to_distribute % ((winners.len() - i) as u32) != 0 {
                            println!("Montant non divisible. Qui remporte le reste : {} ?", to_distribute % ((winners.len() - i) as u32));
                            let num = inout::ask_player_number(self.players.len() as u32); 
                            self.players[num].cash += to_distribute % winners.len() as u32;
                        }
                        distributed_amount += to_distribute;
                    }
                    self.players[winners[i]].state = 'f';
                    i+=1;
                }
            }
        }
    }

    fn available_pot_amount(&self, for_player: usize) -> u32 {
        let mut amount = 0;
        for player in self.players.iter() {
            amount += if player.total_bet > self.players[for_player].total_bet {self.players[for_player].total_bet} else { player.total_bet };
        }
        amount
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
        for player in &self.players {
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
