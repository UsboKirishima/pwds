pub mod crypto {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key, Nonce,
    };

    pub trait CryptoManager {
        fn encrypt(&self, plaintext: String) -> String;
        fn decrypt(&self, encrypted_data: String) -> String;
        fn generate_key(base_key: &str) -> String;
    }

    pub struct Cipher {
        key: Key<Aes256Gcm>,
    }

    impl Cipher {
        pub fn new(base_key: &str) -> Self {
            let key = Key::<Aes256Gcm>::from_slice(base_key.as_bytes());
            Self { key: key.clone() }
        }
    }

    impl CryptoManager for Cipher {
        fn encrypt(&self, plaintext: String) -> String {

            //println!("{}", &plaintext);

            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let cipher = Aes256Gcm::new(&self.key);
            let ciphered_data = cipher
                .encrypt(&nonce, plaintext.as_bytes())
                .expect("Failed to encrypt");
            let mut encrypted_data: Vec<u8> = nonce.to_vec();
            encrypted_data.extend_from_slice(&ciphered_data);
            hex::encode(encrypted_data)
        }

        fn decrypt(&self, encrypted_data: String) -> String {
            let encrypted_data =
                hex::decode(encrypted_data).expect("Failed to decode hex string into vec");

            let nonce_len = 12; 
            if encrypted_data.len() < nonce_len {
                panic!(
                    "Data length too short: expected at least {}, got {}",
                    nonce_len,
                    encrypted_data.len()
                );
            }
            let (nonce_arr, ciphered_data) = encrypted_data.split_at(nonce_len);
            let nonce = Nonce::from_slice(nonce_arr);
            let cipher = Aes256Gcm::new(&self.key);
            let plaintext = cipher
                .decrypt(nonce, ciphered_data)
                .expect("Failed to decrypt data");
            String::from_utf8(plaintext).expect("Failed to convert vector of bytes to string")
        }

        fn generate_key(base_key: &str) -> String {
            let mut key = [0u8; 32];
            let base_bytes = base_key.as_bytes();
            key[..base_bytes.len()].copy_from_slice(base_bytes);
            hex::encode(key)
        }
    }
}
