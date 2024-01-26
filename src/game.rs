use rusqlite::Result;
use std::io;

pub fn play_hangman(db_word: &str) -> Result<()> {
    let mut ans = String::new();

    // TODO: Implement the game logic
    // 1. Select a random word from the database
    // 2. Implement the game loop where the user guesses letters

    let mut i = 1;
    loop {
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");
        if answer_correct(&ans, db_word) {
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

pub fn answer_correct(answer: &str, db_word: &str) -> bool {
    // TODO: Check if the answer is correct
    if answer.trim() == db_word.trim() {
        return true;
    }
    false
}
