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
}
