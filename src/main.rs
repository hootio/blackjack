use anyhow::Result;
use std::io;
use std::io::Write;

mod game;

fn intro(deck_count: usize) -> Result<String> {
    println!(
        "
Welcome to Blackjack, by hootio!

Rules:
-   Deck count: {}
-   Dealer hits on soft 17
",
        deck_count
    );

    print!("Type your name and hit enter to start the game: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let name = buffer.trim().to_string();
    if name.len() == 0 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Name cannot be empty",
        ))?
    }

    println!("\nGood luck, {}\n", name);

    Ok(name)
}

fn outro(name: String) {
    println!(
        "
Great playing with you, {}!
Come back again :)
",
        name
    );
}

fn main() -> Result<()> {
    let mut game = game::Game::new();

    let name = intro(game.deck_count)?;

    println!("The winner is {}!", game.play()?);

    // TODO: Add keep playing on same deck logic here
    outro(name);

    Ok(())
}
