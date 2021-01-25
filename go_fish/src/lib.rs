extern crate cards_lib as lib;



use lib::card::*;
use lib::deck::*;

pub fn play_game() {
    let mut deck = generate_deck();
    deck.shuffle();

    let player_one = Player::new(1, &mut deck, 9);
    let player_two = Player::new(3, &mut deck, 9);

    let mut players = Players(player_one, player_two);

    while players.0.get_score() + players.1.get_score() < 13 {
        players.0.display_hand();
        players.1.display_hand();

        players.1.take_turn(&mut players.0);
        //Deals next card off of deck
        players.1.cards.0.push(deck.deal().unwrap());
        if players.1.four_of_a_kind(players.1.cards.0.last().unwrap().get_number()) {
            players.1.contains(players.1.cards.0.last().unwrap().get_number());
            players.1.update_score();
        }
        println!("Computer Score is {}\n", players.1.get_score()); 
        players.0.display_hand();
        players.1.display_hand();

        players.0.take_turn(&mut players.1);
        //Deals next card off of deck
        players.0.cards.0.push(deck.deal().unwrap());
        if players.0.four_of_a_kind(players.0.cards.0.last().unwrap().get_number()) {
            players.0.contains(players.0.cards.0.last().unwrap().get_number());
            players.0.update_score();
        }
        println!("Player {} Score is {}\n", players.0.player_id, players.0.get_score()); 
        
    }

    if players.0.get_score() > players.1.get_score() {
        println!("Congratulations Player 0 you won with a score of {}", players.0.get_score());
    } else {
        println!("Congratulations Computer you won with a score of {}", players.1.get_score());
    }
}


#[derive(Debug)]
pub struct Player {
    pub player_id : u32,
    pub cards : Deck,
    score : u32,
    asked : Vec<Number>
}
use std::io;
use rand::seq::SliceRandom;
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
    
    pub fn take_turn(&mut self, other : &mut Player) {
        loop {
            
            if self.player_id == 3 {
                //if i have a card the player asked for, ask for that card
                let cards_to_transfer : &Vec<Card> = &Vec::new();
                for player_asked in &mut other.asked {
                    let cards_to_transfer = self.contains(*player_asked);
                    if !cards_to_transfer.is_empty() {

                        break;
                    }
                }


                if !cards_to_transfer.is_empty() {
                    println!("Computer Asked: Do you have any {:?}s?", cards_to_transfer.last().unwrap().get_number());
                    //If i have a card that a players asked for they must have that card then repeat turn
                    println!("Player {}: Yes I do have some {:?}s \nGo Again\n", other.player_id, cards_to_transfer.last().unwrap().get_number());
                    self.cards.0.append(&mut other.contains(cards_to_transfer.last().unwrap().get_number()));
                } else {
                    //else ask for a card I haven't asked for yet in my hand TODO
                    let mut to_ask : Number = self.cards.0.choose(&mut rand::thread_rng()).unwrap().get_number();
                    while self.asked.iter().any(|&i| i==to_ask) {
                        to_ask = self.cards.0.choose(&mut rand::thread_rng()).unwrap().get_number()
                    }
                    let cards_to_transfer = &mut other.contains(to_ask);
                    println!("Computer : Do you have any {:?}s?", to_ask);
                    if cards_to_transfer.is_empty() {
                        println!("Player {}: No I don't have any {:?}s \nMy turn!\n", other.player_id, to_ask);
                        break; 
                    }
                    println!("Player {}: Yes I do have some {:?}s \nGo Again\n", other.player_id, to_ask);
                    self.cards.0.append(cards_to_transfer);
                }
                
                
                //if none then ask for a random card ?Is this possible?
                
            } else {
                println!("Player {} : Got Any? ", self.player_id);
                //Read Players input
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let split: Vec<&str> = input.split(' ').collect();
                //
                //Parse it into a Number enum
                let number : Number = split[0].parse().unwrap();
    
                //Search other players hand for all cards containing number
                let cards_to_transfer = &mut other.contains(number);

                if cards_to_transfer.is_empty() {
                    println!("Player {}: No I don't have any {}s \nMy turn!\n", other.player_id, input.trim());
                    break; 
                }
                println!("Player {}: Yes I do have some {}s \nGo Again\n", other.player_id, input.trim());
                self.cards.0.append(cards_to_transfer);
                
                if self.four_of_a_kind(number) {
                    &mut self.contains(number);
                    self.update_score();
                }
                self.display_hand();
            }
            
            
        }
        
    }


    /*Check for a four of a kind with a given number enum
    */
    pub fn four_of_a_kind(&mut self, number : Number) -> bool {
        let mut count = 0;
        let mut index = 0;

        while index < self.cards.size() {
            if self.cards.0[index].get_number() == number {
                count += 1;
            }
            index += 1;
        }
        count == 4
    }
    
    /*Players deck contains any cards of type Suit and returns
        a deck object containing such cards */
    pub fn contains(&mut self, number : Number) -> Vec::<Card> {
        let mut cards = Vec::<Card>::with_capacity(4);
        let mut index = 0;
        while index < self.cards.size() {
            if number == self.cards.0[index].get_number() {
                //println!("Removing {:?} from hand", self.cards.0.get(index));
                cards.push(self.cards.0.remove(index));
                index = 0;
            } else {
                index += 1;
            }
            
            
        }
        
        cards
    }

}

pub struct Players(pub Player, pub Player);



