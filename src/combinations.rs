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
    for i in 1..5 {
        suits[i-1] = extract_suit(deck, i as u32);
        suits[i-1].sort();
    }
    cards::sort_decks(suits);
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