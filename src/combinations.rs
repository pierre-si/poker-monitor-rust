use crate::cards;

pub fn value_count(vector: &[u32], value: u32) -> usize {
    vector.iter().filter(|&n| *n == value).count()
}

fn extract_suit(hand: &cards::Hand, suit: u32) -> cards::Hand {
    let mut extracted = cards::Hand::new(value_count(&hand.suits[..hand.values.len()], suit));

    for i in 0..hand.values.len() {
        if hand.suits[i] == suit {
            extracted.values.push(hand.values[i]);
            extracted.suits.push(suit);
        }
    }
    extracted
}

fn split_suit(hand: &cards::Hand) -> Vec<cards::Hand> {
    let mut suits: Vec<cards::Hand> = Vec::new();
    for i in 1..5 {
        suits.push(extract_suit(hand, i as u32));
        suits[i-1].sort();
    }
    cards::sort_hands(&mut suits);
    suits
}

fn delete_duplicates(hand: &mut cards::Hand) -> cards::Hand {
    let mut size = 0;
    hand.sort();
    let mut i = 0;
    while i < hand.values.len() {
        while i+1 < hand.values.len() && hand.values[i] == hand.values[i+1] { i += 1; }
        size += 1;
        i += 1;
    }
    let mut unique = cards::Hand::new(size);
    unique.values.push(hand.values[0]);
    unique.suits.push(hand.suits[0]);
    i = 1;
    while unique.values.len() < size {
        while hand.values[i] == unique.values[unique.values.len()-1] { i += 1; }
        unique.values.push(hand.values[i]);
        unique.suits.push(hand.suits[i]);
        i += 1;
    }
    unique
}

fn split_straight(hand: &mut cards::Hand) -> Vec<cards::Hand> {
    let mut hand = delete_duplicates(hand);
    if hand.values[0] == 14 {
        let mut temp = cards::Hand::new(1);
        temp.values.push(1);
        temp.suits.push(hand.suits[0]);
        hand = cards::merge_hands(&hand, &temp);
    }
    let mut straights: Vec<cards::Hand> = Vec::new();
    let mut i = 0;
    let mut j;
    while i < hand.values.len() {
        j = i;
        while i+1 < hand.values.len() && hand.values[i] == hand.values[i+1]+1 {
            i += 1;
        }
        i += 1;
        let mut straight = cards::Hand::new(i-j);
        while j < i {
            straight.values.push(hand.values[j]);
            straight.suits.push(hand.suits[j]);
            j += 1;
        }
        straights.push(straight);
    }
    cards::sort_hands(&mut straights);
    straights
}

fn extract_values(hand: &cards::Hand, value: u32) -> cards::Hand {
    let mut extracted = cards::Hand::new(value_count(&hand.values[..hand.values.len()], value));
    let mut i = 0;
    while extracted.values.len() < extracted.cards_number {
        if hand.values[i] == value {
            extracted.values.push(hand.values[i]);
            extracted.suits.push(hand.suits[i]);
        }
        i += 1;
    }
    extracted
}

fn split_same_values(hand: &mut cards::Hand) -> Vec<cards::Hand> {
    let unique = delete_duplicates(hand);
    let mut same_values: Vec<cards::Hand> = Vec::new();
    for i in 0..unique.cards_number {
        same_values.push(extract_values(hand, unique.values[i]));
    }
    cards::sort_hands(&mut same_values);
    same_values
}

pub fn combination_type(hand: &mut cards::Hand, types: &mut [u32]) {
    /*
    let mut suits: Vec<cards::Deck> = Vec::with_capacity(4);
    unsafe { suits.set_len(4); }
    let mut straights: Vec<cards::Deck> = Vec::with_capacity(7);
    unsafe { straights.set_len(7); }
    let mut same: Vec<cards::Deck> = Vec::with_capacity(7);
    unsafe { same.set_len(7); }
    let mut suits_straights: Vec<cards::Deck> = Vec::with_capacity(7);
    unsafe { suits_straights.set_len(7); }
    */

    for i in 1..types.len() {
        types[i] = 0;
    }
    
    let mut suits = split_suit(hand);
    let straights = split_straight(hand);
    let same = split_same_values(hand);
    let suits_straights = split_straight(&mut suits[0]);

    if suits_straights[0].values.len() >= 5 {
        types[0] = 9;
        types[1] = suits_straights[0].values[0];
    } else if same[0].values.len() == 4 {
        types[0] = 8;
        types[1] = same[0].values[0];
        types[2] = if hand.values[0] != types[1] { hand.values[0] } else { hand.values[4] };
    } else if same[0].values.len() == 3 && same[1].values.len() >= 2 {
        types[0] = 7;
        types[1] = same[0].values[0];
        types[2] = same[1].values[1]; 
    } else if suits[0].values.len() >= 5 {
        types[0] = 6;
        for i in 1..6 {
            types[i] = suits[0].values[i-1];
        }
    } else if straights[0].values.len() >= 5 {
        types[0] = 5;
        types[1] = straights[0].values[0];
    } else if same[0].values.len() == 3 {
        types[0] = 4;
        types[1] = same[0].values[0];
        types[2] = if hand.values[0] != types[1] { hand.values[0] } else { hand.values[3] };
        types[3] = if hand.values[1] != types[1] { hand.values[1] } else { hand.values[4] };
    } else if same[1].values.len() == 2 {
        types[0] = 3;
        types[1] = same[0].values[0];
        types[2] = same[1].values[0];
        types[3] = if hand.values[0] != types[1] { hand.values[0] } else {
            if hand.values[2] != types[2] { hand.values[2] } else { hand.values[4] } };
    } else if same[0].values.len() == 2 {
        types[0] = 2;
        types[1] = same[0].values[0];
        types[2] = if hand.values[0] != types[1] { hand.values[0] } else { hand.values[2] };
        types[3] = if hand.values[1] != types[1] { hand.values[1] } else { hand.values[3] };
        types[4] = if types[3] == hand.values[1] && hand.values[2] != types[1] { hand.values[2] } else { hand.values[4] };
    } else {
        types[0] = 1;
        for i in 1..6 {
            types[i] = hand.values[i-1];
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