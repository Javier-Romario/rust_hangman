mod database;

use database::initialize_db;
use rusqlite::{Connection, Result};
use std::io;

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // Initialize the database (create tables, etc.)
    initialize_db(&conn)?;

    // Start the game
    play_hangman(&conn)?;

    Ok(())
}

fn play_hangman(conn: &Connection) -> Result<()> {
    let mut ans = String::new();
    let db_word = get_random_word(conn)?;

    // TODO: Implement the game logic
    // 1. Select a random word from the database
    // 2. Implement the game loop where the user guesses letters

    let mut i = 1;
    loop {
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");
        if answer_correct(&ans, &db_word) {
            println!(
                "Correct! You did it in {} tries, the answer was: {}",
                i, db_word
            );
            break;
        } else {
            println!("Wrong, try again");
            // println!("db_word: {}", db_word);
            // println!("ans: {}", ans);
            ans.clear();
            i += 1;
        }
    }

    // 3. Reveal letters or end the game based on user guesses
    Ok(())
}

fn get_random_word(conn: &Connection) -> Result<String> {
    // TODO: Get a random word from the database
    let mut smt = conn.prepare(
        "
SELECT word FROM words
ORDER BY random()  
LIMIT 1;
",
    )?;

    let mut rows = smt.query([])?;
    let mut words: Vec<String> = Vec::new();
    while let Some(row) = rows.next()? {
        words.push(row.get(0)?);
    }

    // println!("word got: {:?}", words.get(0));
    Ok(words[0].to_string())
}

fn answer_correct(answer: &str, db_word: &str) -> bool {
    // TODO: Check if the answer is correct
    if answer.trim() == db_word.trim() {
        return true;
    }
    false
}
