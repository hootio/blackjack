use anyhow::*;
use std::io;
use std::io::Write;

use deck::*;

pub struct Game {
    pub deck_count: usize,
    cards: Deck,
    player_hand: Deck,
    dealer_hand: Deck,
}

impl Game {
    const STANDARD_DECK_COUNT: usize = 6;
    pub fn new() -> Game {
        Game {
            deck_count: Self::STANDARD_DECK_COUNT,
            cards: Deck::new_count(Self::STANDARD_DECK_COUNT),
            player_hand: Deck::new_empty(),
            dealer_hand: Deck::new_empty(),
        }
    }

    fn show_dealt_state(&self) {
        println!(
            "
Dealer Hand:
{}, ??

Your Hand:
{}, {}

        ",
            self.dealer_hand.peak(0).unwrap(),
            self.player_hand.peak(0).unwrap(),
            self.player_hand.peak(1).unwrap()
        );
    }

    fn deal(&mut self) {
        println!("Dealing...");
        self.cards.shuffle();
        self.player_hand.add(self.cards.draw().unwrap());
        self.player_hand.add(self.cards.draw().unwrap());
        self.dealer_hand.add(self.cards.draw().unwrap());
        self.dealer_hand.add(self.cards.draw().unwrap());
        self.show_dealt_state();
    }

    fn hit(&mut self) {
        !todo!();
    }

    fn stand(&mut self) {
        !todo!();
    }

    fn win_condition(&mut self) -> Option<String> {
        !todo!();
    }

    fn make_move(&mut self) -> Result<()> {
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

    pub fn play(&mut self) -> Result<String> {
        self.deal();

        loop {
            match self.win_condition() {
                Some(winner) => return Ok(winner),
                None => {
                    self.make_move()?;
                }
            }
        }
    }
}
