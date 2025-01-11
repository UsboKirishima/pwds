use crate::crypto::crypto::{Cipher, CryptoManager};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub const DB_PATH: &str = "pwds.enc";

/// Sets the encryption key for the password manager.
pub fn set_enc_key(key: String) {
    let cipher = Cipher::new(&key);
}

/// Checks if the encrypted password database file exists.
pub(crate) fn is_db_file() -> bool {
    Path::new(DB_PATH).exists()
}

/// Encrypts and saves a password to the database file.
/// save_password("user1", "password123", "your-encryption-key");
pub fn save_password(username: &str, password: &str, key: &str) -> io::Result<()> {
    let cipher = Cipher::new(key);

    let encrypted_password = cipher.encrypt(password.to_string());

    let mut passwords = load_passwords(key)?;
    passwords.push((username.to_string(), encrypted_password));

    let file = File::create(DB_PATH)?;
    let mut writer = io::BufWriter::new(file);

    for (user, enc_pwd) in passwords {
        writeln!(writer, "{}:{}", user, enc_pwd)?;
    }

    Ok(())
}

/// Loads the encrypted passwords from the database file and decrypts them.
pub fn load_passwords(key: &str) -> io::Result<Vec<(String, String)>> {
    let cipher = Cipher::new(key);

    if !is_db_file() {
        return Ok(vec![]);
    }

    let mut file = File::open(DB_PATH)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut passwords = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split(':');
        if let (Some(username), Some(encrypted_pwd)) = (parts.next(), parts.next()) {
            let decrypted_password = cipher.decrypt(encrypted_pwd.to_string());
            passwords.push((username.to_string(), decrypted_password));
        }
    }

    Ok(passwords)
}

/// Removes a password from the database file by username.
pub fn remove_password(username: &str, key: &str) -> io::Result<()> {
    let cipher = Cipher::new(key);

    let mut passwords = load_passwords(key)?;

    passwords.retain(|(user, _)| user != username);

    let file = File::create(DB_PATH)?;
    let mut writer = io::BufWriter::new(file);

    for (user, enc_pwd) in passwords {
        writeln!(writer, "{}:{}", user, enc_pwd)?;
    }

    Ok(())
}

/// Modifies an existing password for a given username.
pub fn modify_password(username: &str, new_password: &str, key: &str) -> io::Result<()> {
    let cipher = Cipher::new(key);

    let mut passwords = load_passwords(key)?;

    for (user, enc_pwd) in &mut passwords {
        let decrypted_pwd = cipher.decrypt(enc_pwd.clone());

        if user == username && decrypted_pwd == new_password {
            *enc_pwd = cipher.encrypt(new_password.to_string());
            break;
        }
    }

    let file = File::create(DB_PATH)?;
    let mut writer = io::BufWriter::new(file);

    for (user, enc_pwd) in passwords {
        writeln!(writer, "{}:{}", user, enc_pwd)?;
    }

    Ok(())
}
