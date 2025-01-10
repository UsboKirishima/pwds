pub mod pwds {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key, Nonce,
    };
    use std::fs;
    use std::io;
    use std::io::stdin;
    use std::io::Write;

    pub const DB_PATH: &str = "pwds.enc";

    pub(crate) fn is_db_file() -> bool {
        match fs::metadata(DB_PATH) {
            Ok(metadata) => metadata.is_file(),
            Err(_) => false,
        }
    }

    pub(crate) fn _change_enc_key() -> String {
        let mut new_key = String::new();

        print!("[?] Insert new key: ");
        Write::flush(&mut io::stdout()).expect("[-] Error during flush.");

        stdin()
            .read_line(&mut new_key)
            .expect("[-] Error reading encryption key.");

        new_key.trim().to_string()
    }

    pub(crate) fn encrypt(key_str: String, plaintext: String) -> String {
        let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher = Aes256Gcm::new(key);
        let ciphered_data = cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .expect("failed to encrypt");
        // combining nonce and encrypted data together
        // for storage purpose
        let mut encrypted_data: Vec<u8> = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphered_data);
        hex::encode(encrypted_data)
    }

    pub(crate) fn decrypt(key_str: String, encrypted_data: String) -> String {
        let encrypted_data =
            hex::decode(encrypted_data).expect("failed to decode hex string into vec");
        let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
        let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_arr);
        let cipher = Aes256Gcm::new(key);
        let plaintext = cipher
            .decrypt(nonce, ciphered_data)
            .expect("failed to decrypt data");
        String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
    }
}
