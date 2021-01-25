
pub mod deck {
    use super::card::*;
    /*Deck Struct
        Vector that holds each card object
     */
    #[derive(Debug)]
    pub struct Deck(pub Vec<Card>);


    /* generate_deck
        constructs a deck of 52 cards then returns that deck
        @returns - Deck struct with cards vec
    */
    pub fn generate_deck() -> Deck {
        let suits : [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
        let numbers : [Number; 13] = [Number::Two, Number::Three, Number::Four, Number::Five, Number::Six, Number::Seven, 
                                        Number::Eight, Number::Nine, Number::Ten, Number::Jack, Number::Queen, Number::King, Number::Ace];
        
        let mut deck = Deck(Vec::<Card>::with_capacity(52));
        for suit in &suits {
            for number in &numbers {
                deck.0.push(Card::new(*number, *suit));
            }
        }
        
        return deck;
    }


    use rand::prelude::SliceRandom;
    impl Deck {
        
        pub fn shuffle(&mut self) {
            self.0.shuffle(&mut rand::thread_rng());
        }


        pub fn deal(&mut self) -> Option<Card> {
            if self.size() < 1 {
                panic!("Error: Dealing from empty deck")
            } 
            self.0.pop()
        }

        //TODO: Deal n # of cards into a Vec from a deck
        pub fn deal_n(&mut self, n_cards : usize) -> Vec<Card> {
            let mut hand = Vec::<Card>::with_capacity(n_cards);
            for mut _i  in 0..n_cards {
                let card = self.deal();
                match card {
                    Some(x) => hand.push(x),
                    None => _i -= 1,
                }
            }
            hand
        }

        pub fn size(&mut self) -> usize{
            let mut count : usize = 0;
            for _card in &self.0 {
                count += 1;
            }

            count
        }
    }

    
    impl std::fmt::Display for Deck {
        #[allow(unused_must_use)]
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            for card in &self.0 {
                write!(f, "{:?} {:?}\n", card.get_number(), card.get_suit());
            }
            write!(f, "")
        }
    }
}



pub mod card { 
    

    #[derive(Copy, Clone)]
    /*Card Struct
        Used to hold the suit and number(pip) of each individual card type
     */
    #[derive(Debug)]
    pub struct Card {
        suit : Suit,
        number : Number
    }

    


    /* Suit Enum
        Holds each card suit type
     */
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Suit {
        Club,
        Diamond,
        Heart,
        Spade
    }

    /* Number Enum
        Holds each pip type for all card options
    */
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Number {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace
    }


    impl Card {
        
        /* New
            Generates new card given a number enum and suit enum
        */
        pub fn new(n : Number, s : Suit) -> Card  {
            Card {
                suit : s,
                number : n
            }
        }

        /* get_suit
            Returns the card suit of given card
        */
        pub fn get_suit(self) -> Suit {
            self.suit
        }

        /* get_number
            Returns the number of the given card    
        */
        pub fn get_number(self) -> Number {
            self.number
        }

        
        
    }

    /*Converts a string to suit enum */
    impl std::str::FromStr for Suit {
        type Err = String;
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str().trim() {
                "club" => Ok(Suit::Club),
                "heart" => Ok(Suit::Heart),
                "spade" => Ok(Suit::Spade),
                "diamond" => Ok(Suit::Diamond),
                _ => Err(format!("'{}' is not a valid value for Suit", s)),
            }
        }
    }

    /*Converts a string to number enum */
    impl std::str::FromStr for Number {
        type Err = String;
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str().trim() {
                "two" => Ok(Number::Two),
                "three" => Ok(Number::Three),
                "four" => Ok(Number::Four),
                "five" => Ok(Number::Five),
                "six" => Ok(Number::Six),
                "seven" => Ok(Number::Seven),
                "eight" => Ok(Number::Eight),
                "nine" => Ok(Number::Nine),
                "ten" => Ok(Number::Ten),
                "jack" => Ok(Number::Jack),
                "queen" => Ok(Number::Queen),
                "king" => Ok(Number::King),
                "ace" => Ok(Number::Ace),
                _ => Err(format!("'{}' is not a valid value for Number", s)),
            }
        }
    }
}



