mod database;
mod game;

use crate::database::{get_random_word, initialize_db};
use game::play_hangman;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // Initialize the database (create tables, etc.)
    initialize_db(&conn)?;

    let db_word = get_random_word(&conn)?;

    // Start the game
    play_hangman(&db_word)?;

    Ok(())
}
