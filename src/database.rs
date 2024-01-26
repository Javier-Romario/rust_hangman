use rusqlite::{Connection, Result};

pub fn initialize_db(conn: &Connection) -> Result<()> {
    // TODO: Create tables if they don't exist
    // For example, a table for words: CREATE TABLE words (id INTEGER PRIMARY KEY, word TEXT NOT NULL)
    match conn.execute(
        "CREATE TABLE if not exists words (
            id   INTEGER PRIMARY KEY,
            word TEXT NOT NULL UNIQUE
        )",
        [],
    ) {
        Ok(updated) => println!("{} rows were udpated", updated),
        Err(err) => println!("update failed: {}", err),
    };

    let words = vec!["Hello", "Something", "World", "Mirai", "Helena", "Bla"];

    for word in words {
        conn.execute("INSERT INTO words (word) VALUES (?1)", [word])?;
    }

    Ok(())
}

pub fn get_random_word(conn: &Connection) -> Result<String> {
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
