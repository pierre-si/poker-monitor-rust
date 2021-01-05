use crate::cards;

pub fn value_count(vector: &[u32], value: u32) -> usize {
    vector.iter().filter(|&n| *n == value).count()
}

fn extract_suit(deck: &cards::Deck, suit: u32) -> cards::Deck {
    let mut extracted = cards::Deck::new(value_count(&deck.suits[..deck.known_cards_number], suit));

    for i in 0..deck.known_cards_number {
        if deck.suits[i] == suit {
            extracted.values[extracted.known_cards_number] = deck.values[i];
            extracted.suits[extracted.known_cards_number] = suit;
            extracted.known_cards_number += 1;
        }
    }
    extracted
}

fn split_suit(deck: &cards::Deck, suits: &mut Vec<cards::Deck>) {
    let mut suits: Vec<cards::Deck> = Vec::new();
    for i in 1..5 {
        suits.push(extract_suit(deck, i as u32));
        suits[i-1].sort();
    }
    cards::sort_decks(&mut suits);
}

fn delete_duplicates(deck: &mut cards::Deck) -> cards::Deck {
    let mut size = 0;
    deck.sort();
    let mut i = 0;
    while i < deck.known_cards_number {
        while i+1 < deck.known_cards_number && deck.values[i] == deck.values[i+1] { i += 1; }
        size += 1;
        i += 1;
    }
    let mut unique = cards::Deck::new(size);
    unique.known_cards_number = 1;
    unique.values[0] = deck.values[0];
    unique.suits[0] = deck.suits[0];
    i = 1;
    while unique.known_cards_number < size {
        while deck.values[i] == unique.values[unique.known_cards_number-1] { i += 1; }
        unique.values[unique.known_cards_number] = deck.values[i];
        unique.suits[unique.known_cards_number] = deck.suits[i];
        unique.known_cards_number += 1;
        i += 1;
    }
    unique
}

fn split_straight(deck: &mut cards::Deck, straights: &mut Vec<cards::Deck>) {

    *deck = delete_duplicates(deck);
    if deck.values[0] == 14 {
        let mut temp = cards::Deck::new(1);
        temp.known_cards_number = 1;
        temp.values[0] = 1;
        temp.suits[0] = deck.suits[0];
        *deck = cards::merge_decks(&deck, &temp);
    }
    let mut i = 0;
    let mut number = 0;
    let mut j;
    while i < deck.known_cards_number {
        j = i;
        while i+1 < deck.known_cards_number && deck.values[i] == deck.values[i+1]+1 {
            i += 1;
        }
        i += 1;
        straights[number] = cards::Deck::new(i-j);
        while j < i {
            let index = straights[number].known_cards_number;
            straights[number].values[index] = deck.values[j];
            straights[number].suits[index] = deck.suits[j];
            j += 1;
            straights[number].known_cards_number += 1;
        }
        number += 1;
    }
    cards::sort_decks(straights);
}

fn extract_values(deck: &cards::Deck, value: u32) -> cards::Deck {
    let mut extracted = cards::Deck::new(value_count(&deck.values[..deck.known_cards_number], value));
    let mut i = 0;
    while extracted.known_cards_number < extracted.cards_number {
        if deck.values[i] == value {
            extracted.values[extracted.known_cards_number] = deck.values[i];
            extracted.suits[extracted.known_cards_number] = deck.suits[i];
            extracted.known_cards_number += 1;
        }
        i += 1;
    }
    extracted
}

fn split_same_values(deck: &mut cards::Deck, same_values: &mut Vec<cards::Deck>) {
    let unique = delete_duplicates(deck);
    for i in 0..unique.cards_number {
        same_values[i] = extract_values(deck, unique.values[i]);
    }
    cards::sort_decks(same_values);
}

pub fn combination_type(deck: &mut cards::Deck, types: &mut [u32]) {
    let mut suits = vec![];
    let mut straights = vec![];
    for i in 1..7 { straights.push(cards::Deck::new(1)); }//::new();
    let mut same = vec![];//::new();
    for i in 1..7 { same.push(cards::Deck::new(1)); }
    let mut suits_straights = vec![];//::new();
    for i in 1..7 { suits_straights.push(cards::Deck::new(1)) }

    /*for i in types.iter() {
        *i = 0;
    }*/
    
    split_suit(deck, &mut suits);
    split_straight(deck, &mut straights);
    split_same_values(deck, &mut same);
    split_straight(&mut suits[0], &mut suits_straights);

    if suits_straights[0].known_cards_number >= 5 {
        types[0] = 9;
        types[1] = suits_straights[0].values[0];
    } else if same[0].known_cards_number == 4 {
        types[0] = 8;
        types[1] = same[0].values[0];
        types[2] = if deck.values[0] != types[1] { deck.values[0] } else { deck.values[4] };
    } else if same[0].known_cards_number == 3 && same[1].known_cards_number >= 2 {
        types[0] = 7;
        types[1] = same[0].values[0];
        types[2] = same[1].values[1]; 
    } else if suits[0].known_cards_number >= 5 {
        types[0] = 6;
        for i in 1..6 {
            types[i] = suits[0].values[i-1];
        }
    } else if straights[0].known_cards_number >= 5 {
        types[0] = 5;
        types[1] = straights[0].values[0];
    } else if same[0].known_cards_number == 3 {
        types[0] = 4;
        types[1] = same[0].values[0];
        types[2] = if deck.values[0] != types[1] { deck.values[0] } else { deck.values[3] };
        types[3] = if deck.values[1] != types[1] { deck.values[1] } else { deck.values[4] };
    } else if same[1].known_cards_number == 2 {
        types[0] = 3;
        types[1] = same[0].values[0];
        types[2] = same[1].values[0];
        types[3] = if deck.values[0] != types[1] { deck.values[0] } else {
            if deck.values[2] != types[2] { deck.values[2] } else { deck.values[4] } };
    } else if same[0].known_cards_number == 2 {
        types[0] = 2;
        types[1] = same[0].values[0];
        types[2] = if deck.values[0] != types[1] { deck.values[0] } else { deck.values[2] };
        types[3] = if deck.values[1] != types[1] { deck.values[1] } else { deck.values[3] };
        types[4] = if types[3] == deck.values[1] && deck.values[2] != types[1] { deck.values[2] } else { deck.values[4] };
    } else {
        types[0] = 1;
        for i in 1..6 {
            types[i] = deck.values[i-1];
        }
    }
}

pub fn compare_combinations(first: &[u32], second: &[u32]) -> u32 {
    let mut i = 0;
    while first[i] != 0 {
        if first[i] > second[i] { return 1 }
        if first[i] < second[i] { return 0 }
        i += 1;
    }
    return 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_count() {
        let vector = vec![0, 0, 0, 3, 2, 3];
        let mut value = 3;
        assert_eq!(2, value_count(&vector, value));

    }

    #[test]
    fn slice_count() {
        let vector = vec![0, 0, 0, 3, 2, 3];
        let mut value = 3;
        assert_eq!(1, value_count(&vector[0..4], value));
    }
}