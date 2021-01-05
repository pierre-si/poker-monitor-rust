// usize / isize when the variable is related to memory size (size of an object, index of a vector)
// i32 / u64 / … when the variable is a number

#[derive(Debug)]
pub struct Deck {
    pub cards_number: usize,
    pub known_cards_number: usize,

    pub values: Vec<u32>, // 0 indéterminé, 1 as, 2 deux, …, 10 dix, 11 valet, 12 dame, 13 roi, (14 as accessoirement)
    pub suits: Vec<u32>, // 0 indéterminé, 1 trèfle, 2 carreau, 3 cœur, 4 pique.
}

impl Deck {
    pub fn new(cards_number: usize) -> Deck {
        Deck {
            cards_number: cards_number,
            known_cards_number: 0,

            values: vec![0; cards_number],
            suits: vec![0; cards_number]
        }
    }

    pub fn reset_cards(&mut self) {
        self.known_cards_number = 0;
        for value in &mut self.values { *value = 0; }
        for suit in &mut self.suits { *suit = 0; }
    }

    pub fn sort_deck(&mut self) {
        // insertion sort
        let mut value: u32;
        let mut suit: u32;
        let mut j: usize;
        for i in 0..self.known_cards_number {
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

    pub fn compare_with(&self, other: &Deck) -> std::cmp::Ordering {
        for i in 0..self.known_cards_number {
            if self.values[i] > other.values[i] { return std::cmp::Ordering::Greater }
            if self.values[i] < other.values[i] { return std::cmp::Ordering::Less }
        }

        std::cmp::Ordering::Equal
    }
}

pub fn merge_decks(first_deck: &Deck, second_deck: &Deck) -> Deck {
    let mut merge = Deck::new(first_deck.cards_number + second_deck.cards_number);
    merge.known_cards_number = first_deck.known_cards_number + second_deck.known_cards_number;

    for i in 0..first_deck.known_cards_number {
        merge.values[i] = first_deck.values[i];
        merge.suits[i] = first_deck.suits[i];
    }
    for i in first_deck.known_cards_number..first_deck.known_cards_number+second_deck.known_cards_number {
        merge.values[i] = second_deck.values[i - first_deck.known_cards_number];
        merge.suits[i] = second_deck.suits[i - first_deck.known_cards_number];
    }
    merge
}

pub fn sort_decks(decks: &mut Vec<Deck>) {
    decks.sort_by(|a, b| { a.known_cards_number.cmp(&b.known_cards_number).reverse().then(a.compare_with(b).reverse())
    });
/*
    let mut j: usize;
    for i in 0..decks.len() {
        let current = decks[i];
        j = i;
        while j > 0 && (decks[j-1].known_cards_number < current.known_cards_number 
                        || (decks[j-1].known_cards_number == current.known_cards_number
                            && current.higher_value_than(&decks[j-1]))) {
            decks[j] = decks[j-1];
            j -= 1;
        }
        decks[j] = current;
    }
*/
}