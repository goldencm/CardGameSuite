//! # Cards Library
//! 
//! 'cards-lib' is a collection of two modules that make up general
//! functionality for handling of any game that involves a regular deck
//! of 52 cards


/// Contains a struct that holds a vector of Card Structs
/// and various functional implementations for handling the deck struct
pub mod deck {

    use super::card::*;
    
    /// Struct containing a Vec of Card structs
    #[derive(Debug)]
    pub struct Deck(pub Vec<Card>);


    /// Generate a new dack of the standard 52 Cards
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
        ///Uses rand::SliceRandom to shuffle the vector
        pub fn shuffle(&mut self) {
            self.0.shuffle(&mut rand::thread_rng());
        }


        ///Deals a single card from the Deck with the Option of a specified card
        /// 
        /// # Errors
        /// Returns a None type if the specified Card provided is not within the deck
        /// and prints the error
        /// 
        /// # Panics
        /// Deck size is less than 1
        pub fn deal(&mut self, card : Option<Card>) -> Option<Card> {
            match card {
                Some(c) => {
                    let index = self.0.iter().position(|card| card.eq(c));
                    match index {
                        Some(i) => Some(self.0.remove(i)),
                        None => {
                            eprintln!("Card {:?} is not in the deck", c);
                            None
                        }
                    }      
                }
                None => {
                    if self.size() < 1 {
                        panic!("Error: Dealing from empty deck")
                    } 
                    self.0.pop()
                }
            }
            
        }

        /// Deals N number of cards from the deck with the Option of a specified set of cards.
        /// You have the option of providing either a vector of cards or the number of cards you would
        /// like to pop off the deck vector.
        /// 
        /// # Panics
        /// Deck does not contain the provided card
        /// cards & n_cards are both None type    
        pub fn deal_n(&mut self, cards : Option<Vec<Card>>, n_cards : Option<usize>) -> Vec<Card> {
            #![allow(unused)]
            let mut hand = Vec::<Card>::new();
            match cards {
                Some(vec) => {
                    hand = Vec::<Card>::with_capacity(vec.len());
                    for c in vec {
                        let card = self.deal(Some(c));
                        match card {
                            Some(x) => hand.push(x),
                            None => panic!("Deck does not contain card : {:?}", c)
                        }
                    }
                },
                None => {
                    match n_cards{
                        Some(n) => {
                                hand = Vec::<Card>::with_capacity(n_cards.unwrap());
                                for mut _i  in 0..n_cards.unwrap() {
                                    let card = self.deal(None);
                                    match card {
                                        Some(x) => hand.push(x),
                                        None => _i -= 1,
                                    }
                                }
                            },
                        None => panic!("No card size provided")
                    }
                    
                    
                }   
            }
            hand
                
        }

        ///Returns size of current deck counting all Some types
        pub fn size(&mut self) -> usize{
            let mut count : usize = 0;
            for _card in &self.0 {
                count += 1;
            }

            count
        }

        ///Checks wether or not a deck has a card based on a Suit, Number, or both
        /// 
        ///# Panics
        /// No suit or number was provided for the function call
        pub fn has_card(&self, pair : (Option<Suit>, Option<Number>)) -> bool {
    
            match pair {
                //Match self with both suit and number
                (Some(suit), Some(number)) => {
                    for card in &self.0 {
                        if card.eq(Card::new(number, suit)) {
                            return true; }
                    }
                    return false;
                },
    
                //Match self with just suit
                (Some(suit), None) => {
                    for card in &self.0 {
                        if card.get_suit() == suit {
                            return true; }
                    }
                    return false;
                },
    
                //Match self with just number
                (None, Some(number)) => {
                    for card in &self.0 {
                        if card.get_number() == number {
                            return true; }
                    }
                    return false;
                },
    
                //Return error
                (None, None) => panic!("No suit or number provided for has_card function"),
            }
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


/// Implementation of a playing Card containing a Suit and Number

pub mod card { 
    
    /// Card contains a Suit and Number Enum
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct Card {
        suit : Suit,
        number : Number
    }

    


    /// Suit types
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Suit {
        Club,
        Diamond,
        Heart,
        Spade
    }

    /// Pip types
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
        
        /// Creates a new card struct from a number and suit enum
        pub fn new(n : Number, s : Suit) -> Card  {
            Card {
                suit : s,
                number : n
            }
        }

        /// Returns the suit enum from self
        pub fn get_suit(self) -> Suit {
            self.suit
        }

        /// Returns the number enum from self
        pub fn get_number(self) -> Number {
            self.number
        }

        /// Compare two equality of two cards
        pub fn eq(self, other : Card) -> bool {
            self.get_number() == other.get_number() && self.get_suit() == other.get_suit()
        } 
        
        
    }

    /// Converts a string to suit enum
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

    /// Converts a string to number enum
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


