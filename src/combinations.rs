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

fn deleted_duplicates(deck: &cards::Deck) -> cards::Deck {
    deck.sort();
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