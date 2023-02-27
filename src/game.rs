use anyhow::*;
use std::io;
use std::io::Write;

use deck::*;

pub struct Game {
    pub deck_count: usize,
    deck: Deck,
    player_hand: Deck,
    dealer_hand: Deck,
    player_standing: bool,
    dealer_standing: bool,
    player_total: u8,
    dealer_total: u8,
}

impl Game {
    const STANDARD_NUMBER_OF_DECKS: usize = 6;
    pub fn new() -> Game {
        Game {
            deck_count: Self::STANDARD_NUMBER_OF_DECKS,
            deck: Deck::new_count(Self::STANDARD_NUMBER_OF_DECKS),
            player_hand: Deck::new_empty(),
            dealer_hand: Deck::new_empty(),
            player_standing: false,
            dealer_standing: false,
            player_total: 0,
            dealer_total: 0,
        }
    }

    fn best_deck_value(deck: &Deck) -> u16 {
        // TODO: Handle multiple Aces in hand
        let soft_value = deck.value(true);
        let hard_value = deck.value(false);
        if soft_value > 21 {
            hard_value
        } else {
            soft_value
        }
    }

    fn show_player_turn_state(&self) {
        let dealer_value = self.dealer_hand.peak(0).unwrap().value(true);
        let player_value = Self::best_deck_value(&self.player_hand);
        println!(
            "
Dealer Hand: {}
{}, ??

Your Hand: {}
{}

        ",
            dealer_value,
            self.dealer_hand.peak(0).unwrap(),
            player_value,
            self.player_hand
        );
    }

    fn show_dealer_turn_state(&self) {
        let dealer_value = Self::best_deck_value(&self.dealer_hand);
        let player_value = Self::best_deck_value(&self.player_hand);
        println!(
            "
Dealer Hand: {}
{}

Your Hand: {}
{}

        ",
            dealer_value, self.dealer_hand, player_value, self.player_hand
        );
    }

    fn deal(&mut self) {
        println!("Dealing...");
        self.deck.shuffle();
        self.player_hand.add(self.deck.draw().unwrap());
        self.player_hand.add(self.deck.draw().unwrap());
        self.dealer_hand.add(self.deck.draw().unwrap());
        self.dealer_hand.add(self.deck.draw().unwrap());
        self.show_player_turn_state();
    }

    fn hit(&mut self) {
        self.player_hand.add(self.deck.draw().unwrap());
        self.show_player_turn_state();
    }

    fn stand(&mut self) {
        self.player_standing = true;

        println!("Dealer flipping... Hit Enter");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        self.show_dealer_turn_state();
    }

    fn win_condition(&mut self) -> Option<String> {
        let player_best_value = Self::best_deck_value(&self.player_hand);
        let dealer_best_value = Self::best_deck_value(&self.dealer_hand);

        // TODO: remove - for debugging only
        // println!(
        //     "Win condition: player_best:{}, dealer_best:{}",
        //     player_best_value, dealer_best_value
        // );

        if !self.player_standing {
            if player_best_value > 21 {
                return Some("Dealer".to_string());
            }
        }

        if !self.dealer_standing {
            if dealer_best_value > 21 {
                return Some("Player".to_string());
            }
        } else {
            if dealer_best_value > 21 {
                return Some("Player".to_string());
            }
            if dealer_best_value == player_best_value {
                return Some("No one".to_string());
            }
            if dealer_best_value > player_best_value {
                return Some("Dealer".to_string());
            } else {
                return Some("Player".to_string());
            }
        }
        None
    }

    fn player_move(&mut self) -> Result<()> {
        let mut buffer = String::new();
        let mut choice = String::new();

        while choice.len() == 0 {
            print!("(h)it or (s)tand: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer)?;
            choice = buffer.trim().to_string();

            if choice.len() != 0 {
                match choice.as_bytes()[0] as char {
                    'h' => {
                        self.hit();
                    }
                    's' => {
                        self.stand();
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn dealer_move(&mut self) {
        let dealer_best_value = Self::best_deck_value(&self.dealer_hand);
        if dealer_best_value >= 17 {
            println!("Dealer standing... Hit Enter");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            self.dealer_standing = true;
        } else {
            println!("Dealer drawing... Hit Enter");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            self.dealer_hand.add(self.deck.draw().unwrap());
            self.show_dealer_turn_state();
        }
    }

    pub fn play(&mut self) -> Result<String> {
        self.deal();

        loop {
            match self.win_condition() {
                Some(winner) => {
                    return Ok(winner);
                }
                None => {
                    if !self.player_standing {
                        self.player_move()?;
                    } else {
                        self.dealer_move();
                    }
                }
            }
        }
    }
}
