use std::io;
use crate::players;
use crate::cards;

//TODO mettre dans players.rs
pub fn print_players(players: &Vec<players::Player>){
    for player in players {
        println!("Joueur numéro {} possède {} dollars et a pour état {}", player.number, player.cash, player.state);
    }
}

pub fn print_cards(hand: &cards::Hand) {
    let values_names = ["Un", "Deux", "Trois", "Quatre", "Cinq", "Six", "Sept", "Huit", "Neuf", "Dix", "Valet", "Dame", "Roi", "As"];
    let suits_names = ["trèfle", "carreau", "cœur", "pique"];

    println!("La main comporte {} cartes et {} places vides", hand.known_cards_number, hand.cards_number-hand.known_cards_number);
    for i in 0..hand.known_cards_number {
        println!("Carte {:2} : {} de {}", i+1, values_names[(hand.values[i]-1) as usize], suits_names[(hand.suits[i]-1) as usize]);
    }
}

pub fn print_combination(comb: &[u32]) {
    let combinations_names = ["Carte haute", "Paire", "Deux paires", "Brelan", "Quinte", "Couleur", "Full", "Carré", "Quinte couleur", "Quinte royale"];
    let values_names = ["Un", "Deux", "Trois", "Quatre", "Cinq", "Six", "Sept", "Huit", "Neuf", "Dix", "Valet", "Dame", "Roi", "As"];
    print!("{}", combinations_names[(comb[0]-1) as usize]);
    let mut i = 1;
    while i < 6 && comb[i] != 0 {
        print!("{} ", values_names[(comb[i]-1) as usize]);
        i += 1;
    }
    println!("");
}

#[cfg(test)]
mod test_print {
    use super::*;

    #[test]
    fn print_hand() {
        let dummy_hand = cards::Hand {
            cards_number: 4,
            known_cards_number: 3,

            values: vec![10, 11, 12, 0],
            suits: vec![1, 2, 3, 4],
        };
        print_cards(&dummy_hand);
    }

    #[test]
    fn print_comb() {
        let dummy_combination = [4, 11, 10, 9, 8, 7];
        print_combination(&dummy_combination);
    }
}

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

pub fn ask_action(players: &Vec<players::Player>, player: usize, to_bet: u32, raise_value: u32) -> char {
	if player == 0 {
		println!("========= VOUS ARGENT {:5} =========\n", players[player].cash);
	} else{
		println!("======= JOUEUR {} ARGENT {:5} =======\n", player, players[player].cash);
    }
    println!("MISÉ {:5}  CALL   {:5}  RAISE {:5}\n", players[player].round_bet, to_bet - players[player].round_bet, to_bet - players[player].round_bet + raise_value);

    let mut input = String::new();
    let mut action: char;
	loop{
		println!("ACTION ? ");
	    input.clear();
        io::stdin().read_line(&mut input).expect("failed to read line");
        action = input.chars().next().unwrap();
        match action {
            'j' => print_players(players),
            'c' | 'r' | 'b' | 'f' | 'a' => break,
            _ => continue,
        }
	}
	action
}

// créer une méthode add_card(value, suit) ou set_card(value, suit, index) dans hand qui vérifie que le nombre de carte ne dépasse pas la limite.
pub fn ask_cards(hand: &mut cards::Hand, n_cards: usize) -> bool {
    let mut input = String::new();

    for i in hand.known_cards_number..std::cmp::min(hand.known_cards_number+n_cards, hand.cards_number) {
        loop {
            println!("CARD {} VALUE: ", i+1);
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let ch = input.chars().next().unwrap();
            let digit = ch.to_digit(10);
            match digit {
                Some(0) => continue,
                Some(1) => hand.values[i] = 14,
                Some(d) => hand.values[i] = d,
                None => match ch {
                    't' | 'T' => hand.values[i] = 10,
                    'j' | 'J' => hand.values[i] = 11,
                    'q' | 'Q' => hand.values[i] = 12,
                    'k' | 'K' => hand.values[i] = 13,
                    'a' | 'A' => hand.values[i] = 14,
                    'c' | 'C' => if i == hand.known_cards_number { return false },
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
                'c' | 'C' => hand.suits[i] = 1,
                'd' | 'D' => hand.suits[i] = 2,
                'h' | 'H' => hand.suits[i] = 3,
                's' | 'S' => hand.suits[i] = 4,
                _ => continue,
            }
            break; 
        }
    }
    hand.known_cards_number += n_cards;
    if hand.known_cards_number > hand.cards_number { hand.known_cards_number = hand.cards_number }
    true
}