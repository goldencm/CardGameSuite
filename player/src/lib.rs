extern crate cards_lib as lib;
use lib::card::*;
use lib::deck::*;

#[derive(Debug)]
pub struct Player {
    pub player_id : u32,
    pub  cards : Deck,
    score : u32,
    pub asked : Vec<Number>
}

impl Player {
    
    /*Creates a new player object with 
        Player ID, Deck (Vec<Cards>), Number of cards to be inserted into deck 
        The default score is set to 0 when making a new player*/
    pub fn new(player_id : u32, deck : &mut Deck, n_cards : usize) -> Player { 
        Player{player_id, cards : Deck((deck.deal_n(n_cards)).to_vec()), score : 0, asked : Vec::new()}
    }

    pub fn get_score(&self) -> u32{
        self.score
    }

    pub fn update_score(&mut self) {
        self.score += 1;
    }

    pub fn display_hand(&self) {
        println!("Player's {} hand", self.player_id);
        println!("{}", self.cards);
    }
}