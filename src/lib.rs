use crate::cards;
use crate::players;
use crate::inout;

pub struct Game {
    pub players: Vec<players::Player>,
    pub small_blind: u32,
    blinds_raise_interval: u32,
    N_CARDS_TO_DEAL: [usize; 4],
    // hand variables
    dealer_index: usize,
    small_blind_index: usize,
    big_blind_index: usize,
    pub hand_number: u32,
    pub pot: u32,
    pub table: cards::Hand,
    // round variables
    pub round_number: usize,
    pub to_bet: u32,
    pub raise_value: u32,
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

    pub fn initialize_hand(&self) {
        self.hand_number += 1;
        if self.hand_number % self.blinds_raise_interval == 0 { self.small_blind *= 2; }
        self.pot = 0;
        self.table.reset_cards();
        self.round_number = 0;

        // reset players hands and check cash
        for player in self.players {
            player.total_bet = 0;
            player.hand.reset_cards();
            if player.cash <= 0 {
                player.cash = 0;
                player.state = 'o';
            } else {
                player.state = 'i';
            }
        }
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
    }
    
    pub fn initialize_round(&self) -> usize {
        self.round_number += 1;
        self.to_bet = 0;
        self.raise_value = 2*self.small_blind;
        for player in self.players {
            player.total_bet += player.round_bet;
            player.round_bet = 0;
        }
        inout::ask_cards(&mut self.table, self.N_CARDS_TO_DEAL[self.round_number-1]);
        // le premier joueur actif après le dealer commence le tour
        // premier tour : la mise des blinds est déjà faite
        if self.round_number == 1 {
            self.next_active_player(self.big_blind_index)
        } else {
            self.next_active_player(self.dealer_index)
        }
    }

    pub fn rotate_buttons(&self) {
        self.dealer_index = self.small_blind_index;
        // ne correspond pas à PokerTH: si celui qui était bigblind meurt il ne devient pas small dans PokerTH
        self.small_blind_index = self.big_blind_index;
        self.big_blind_index = self.next_active_player(self.big_blind_index);
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
