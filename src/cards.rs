// usize / isize when the variable is related to memory size (size of an object, index of a vector)
// i32 / u64 / … when the variable is a number

// je pensais devoir implémenter Clone sur Hand pour régler les problèmes de combinations (le clonage permettant de les enregistrer dans le vecteur de hands), mais finalement pas besoin, il est possible de créer les vecteurs de hands dans les fonctions appelées par combination_type et de renvoyer l'ownership de ces derniers à combination_type
// je pensais aussi essayer de créer un vec vide dans type_combination, de passer sa ref mut aux sous fonctions et de push à l'intérieur des sous fonctions.
#[derive(Debug)]
pub struct Hand {
    pub cards_number: usize,

    pub values: Vec<u32>, // 0 indéterminé, 1 as, 2 deux, …, 10 dix, 11 valet, 12 dame, 13 roi, (14 as accessoirement)
    pub suits: Vec<u32>, // 0 indéterminé, 1 trèfle, 2 carreau, 3 cœur, 4 pique.
}

impl Hand {
    pub fn new(cards_number: usize) -> Hand {
        Hand {
            cards_number: cards_number,

            values: vec![],
            suits: vec![],
        }
    }

    pub fn reset_cards(&mut self) {
        for value in &mut self.values { *value = 0; }
        for suit in &mut self.suits { *suit = 0; }
    }

    pub fn sort(&mut self) {
        // insertion sort
        let mut value: u32;
        let mut suit: u32;
        let mut j: usize;
        for i in 0..self.values.len() {
            value = self.values[i];
            suit = self.suits[i];
            j = i;
            while j > 0 && self.values[j-1] < value {
                self.values[j] = self.values[j-1];
                self.suits[j] = self.suits[j-1];
                j -= 1;
            }
            self.values[j] = value;
            self.suits[j] = suit;
        }
    }

    pub fn compare_with(&self, other: &Hand) -> std::cmp::Ordering {
        if self.values.len() > other.values.len() {
            return std::cmp::Ordering::Greater
        } else if self.values.len() < other.values.len() {
            return std::cmp::Ordering::Less
        } else {
            for i in 0..self.values.len() {
                if self.values[i] > other.values[i] { return std::cmp::Ordering::Greater }
                if self.values[i] < other.values[i] { return std::cmp::Ordering::Less }
            }
        }

        std::cmp::Ordering::Equal
    }
}

pub fn merge_hands(first_hand: &Hand, second_hand: &Hand) -> Hand {
    let mut merge = Hand::new(first_hand.cards_number + second_hand.cards_number);

    for i in 0..first_hand.values.len() {
        merge.values.push(first_hand.values[i]);
        merge.suits.push(first_hand.suits[i]);
    }
    for i in 0..second_hand.values.len() {
        merge.values.push(second_hand.values[i]);
        merge.suits.push(second_hand.suits[i]);
    }
    merge
}

pub fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| { a.compare_with(b).reverse() })
    // pb : then() est semble être exécuté même si la première comparaison n'est pas equal.
    //hands.sort_by(|a, b| { a.known_cards_number.cmp(&b.known_cards_number).reverse().then(a.compare_with(b).reverse())});
/*
    let mut j: usize;
    for i in 0..hands.len() {
        let current = hands[i];
        j = i;
        while j > 0 && (hands[j-1].known_cards_number < current.known_cards_number 
                        || (hands[j-1].known_cards_number == current.known_cards_number
                            && current.higher_value_than(&hands[j-1]))) {
            hands[j] = hands[j-1];
            j -= 1;
        }
        hands[j] = current;
    }
*/
}

#[cfg(test)]
mod test_print {
    use super::*;

    #[test]
    fn two_hands_sort() {
        let mut hands = vec![
            Hand{cards_number: 4, values: vec![11, 4], suits: vec![2, 2]},
            Hand{cards_number: 4, values: vec![11, 4, 6], suits: vec![2, 1, 3]},
        ];

        sort_hands(&mut hands);
        assert_eq!(hands[0].values.len(), 3);
    }
}
