use std::io;
use crate::players;
use crate::cards;

//TODO mettre dans players.rs
pub fn print_players(players: &[players::Player]){
    for player in players {
        println!("Joueur numéro {} possède {} dollars et a pour état {}", player.number, player.cash, player.state);
    }
}

pub fn print_cards(hand: &cards::Hand) {
    let values_names = ["Un", "Deux", "Trois", "Quatre", "Cinq", "Six", "Sept", "Huit", "Neuf", "Dix", "Valet", "Dame", "Roi", "As"];
    let suits_names = ["trèfle", "carreau", "cœur", "pique"];

    println!("La main comporte {} cartes et {} places vides", hand.values.len(), hand.cards_number-hand.values.len());
    for i in 0..hand.values.len() {
        println!("Carte {:2} : {} de {}", i+1, values_names[(hand.values[i]-1) as usize], suits_names[(hand.suits[i]-1) as usize]);
    }
}

pub fn print_combination(comb: &[u32]) {
    let combinations_names = ["Carte haute", "Paire", "Deux paires", "Brelan", "Quinte", "Couleur", "Full", "Carré", "Quinte couleur", "Quinte royale"];
    let values_names = ["Un", "Deux", "Trois", "Quatre", "Cinq", "Six", "Sept", "Huit", "Neuf", "Dix", "Valet", "Dame", "Roi", "As"];
    print!("{} ", combinations_names[(comb[0]-1) as usize]);
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
// régler le cas ou ce n'est pas un nombre => panic.
pub fn ask_player_number(players_count: u32) -> usize {
    let mut input = String::new();
    let mut number: u32;
    loop{
        input.clear();
        io::stdin().read_line(&mut input).expect("failed to read line");
        number = input.trim().parse().expect("Please type a number!");
        if number < players_count { break; }
        else { println!("Please enter a valid player number")}
    }
    return number as usize;
}

pub fn ask_action(players: &[players::Player], player: usize, to_bet: u32, raise_value: u32) -> char {
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

    for i in 0..std::cmp::min(n_cards, hand.cards_number-hand.values.len()) {
        loop {
            println!("CARD {} VALUE: ", hand.values.len()+1);
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let ch = input.chars().next().unwrap();
            let digit = ch.to_digit(10);
            match digit {
                Some(0) => continue,
                Some(1) => hand.values.push(14),
                Some(d) => hand.values.push(d),
                None => match ch {
                    't' | 'T' => hand.values.push(10),
                    'j' | 'J' => hand.values.push(11),
                    'q' | 'Q' => hand.values.push(12),
                    'k' | 'K' => hand.values.push(13),
                    'a' | 'A' => hand.values.push(14),
                    'c' | 'C' => if hand.values.len() == 0 { return false },
                    _ => continue,
                }
            }
            break;
        }
        loop {
            println!("CARD {} SUIT: ", hand.values.len());
            input.clear();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let ch = input.chars().next().unwrap();
            match ch {
                'c' | 'C' => hand.suits.push(1),
                'd' | 'D' => hand.suits.push(2),
                'h' | 'H' => hand.suits.push(3),
                's' | 'S' => hand.suits.push(4),
                _ => continue,
            }
            break; 
        }
    }
    true
}