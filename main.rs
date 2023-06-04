use rusqlite::{Connection, Result};
use rusqlite::OptionalExtension;

struct PasswordManager {
    conn: Connection,
}

impl PasswordManager {
    fn new() -> Result<Self> {
        let conn = Connection::open("passwords.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (id INTEGER PRIMARY KEY AUTOINCREMENT, website TEXT, username TEXT, password TEXT)",
            [],
        )?;
        Ok(PasswordManager { conn })
    }

    fn add_password(&mut self, website: &str, username: &str, password: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO passwords (website, username, password) VALUES (?1, ?2, ?3)",
            &[website, username, password],
        )?;
        Ok(())
    }

    fn get_password(&self, website: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT password FROM passwords WHERE website = ?1")?;
        let password = stmt.query_row(&[website], |row| row.get(0)).optional()?;
        Ok(password)
    }
}

fn main() -> Result<()> {
    let mut manager = PasswordManager::new()?;

    // Add a password
    manager.add_password("example.com", "john_doe", "password123")?;

    // Get a password
    if let Some(password) = manager.get_password("example.com")? {
        println!("Password: {}", password);
    } else {
        println!("No password found for the given website.");
    }

    Ok(())
}
