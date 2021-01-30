
//! # Player Library
//! 
//! 'player' library is used to contstruct individual player
//! objects for games
extern crate cards_lib as lib;
use lib::card::*;
use lib::deck::*;



/// Logic fields for the player

#[derive(Debug)]
pub struct Player {
    pub player_id : u32,
    pub  cards : Deck,
    score : u32,
}

impl Player {
    
    /// Creates a new player object with 
    /// Player ID, Deck (Vec<Cards>), Number of cards to be inserted into deck 
    /// The default score is set to 0 when making a new player
    pub fn new(player_id : u32, deck : &mut Deck, n_cards : usize) -> Player { 
        Player{player_id, cards : Deck((deck.deal_n(None, Some(n_cards))).to_vec()), score : 0}
    }


    /// Returns players score
    pub fn get_score(&self) -> u32{
        self.score
    }


    /// Updates score by Option Some(u32) or by 1 if None
    pub fn update_score(&mut self, update : Option<u32>) {
        match update {
            Some(i) => self.score += i,
            None => self.score += 1
        }
    }

    
    /// Displays all cards in current players hand
    //TODO: send a formatted reference or copied vec to the caller!
    pub fn display_hand(&self) {
        println!("Player's {} hand", self.player_id);
        println!("{}", self.cards);
    }

    
    /// Checks to see if the Option(Suit) || Option(Number) is in the
    /// players deck.
    /// Calls the inner has_card function on the deck
    pub fn has_card(&self, pair : (Option<Suit>, Option<Number>)) -> bool {
        self.cards.has_card(pair)
    }
}