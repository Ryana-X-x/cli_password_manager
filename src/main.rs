use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead};
use rand::Rng;
use serde::{Serialize, Deserialize};
use rpassword::read_password;
use base64::{engine::general_purpose::STANDARD, Engine};
use blake3;

const FILE_PATH: &str = "passwords.json";

#[derive(Serialize, Deserialize)]
struct PasswordStore {
    passwords: HashMap<String, String>,
}

fn get_encryption_key(master_password: &str) -> Key<Aes256Gcm> {
    let mut key_bytes = [0u8; 32];
    let hash = blake3::hash(master_password.as_bytes());
    key_bytes.copy_from_slice(hash.as_bytes());
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}

fn encrypt_password(password: &str, key: &Key<Aes256Gcm>) -> String {
    let cipher = Aes256Gcm::new(key);
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), password.as_bytes())
        .expect("Encryption failed");
    format!("{}:{}", STANDARD.encode(nonce), STANDARD.encode(ciphertext))
}

fn decrypt_password(encrypted: &str, key: &Key<Aes256Gcm>) -> Option<String> {
    let cipher = Aes256Gcm::new(key);
    let parts: Vec<&str> = encrypted.split(':').collect();
    if parts.len() != 2 { return None; }
    let nonce = STANDARD.decode(parts[0]).ok()?;
    let ciphertext = STANDARD.decode(parts[1]).ok()?;
    cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref()).ok()
        .map(|plaintext| String::from_utf8(plaintext).ok()).flatten()
}

fn load_passwords() -> PasswordStore {
    if let Ok(content) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&content).unwrap_or(PasswordStore { passwords: HashMap::new() })
    } else {
        PasswordStore { passwords: HashMap::new() }
    }
}

fn save_passwords(store: &PasswordStore) {
    let content = serde_json::to_string_pretty(store).expect("Serialization failed");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_PATH)
        .expect("Failed to open file");
    file.write_all(content.as_bytes()).expect("Failed to write file");
}

fn main() {
    println!("Enter master password:");
    let master_password = read_password().expect("Failed to read password");
    let key = get_encryption_key(&master_password);
    let mut store = load_passwords();
    
    loop {
        println!("\nOptions:\n1. Add Password\n2. Retrieve Password\n3. Delete Password\n4. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        match choice.trim() {
            "1" => {
                println!("Enter site/service:");
                let mut site = String::new();
                io::stdin().read_line(&mut site).expect("Failed to read input");
                
                println!("Enter password:");
                let password = read_password().expect("Failed to read password");
                store.passwords.insert(site.trim().to_string(), encrypt_password(&password, &key));
                save_passwords(&store);
                println!("Password saved!");
            },
            "2" => {
                println!("Enter site/service:");
                let mut site = String::new();
                io::stdin().read_line(&mut site).expect("Failed to read input");
                if let Some(encrypted) = store.passwords.get(site.trim()) {
                    if let Some(password) = decrypt_password(encrypted, &key) {
                        println!("Password: {}", password);
                    } else {
                        println!("Decryption failed. Incorrect master password?");
                    }
                } else {
                    println!("No password found.");
                }
            },
            "3" => {
                println!("Enter site/service:");
                let mut site = String::new();
                io::stdin().read_line(&mut site).expect("Failed to read input");
                if store.passwords.remove(site.trim()).is_some() {
                    save_passwords(&store);
                    println!("Password deleted.");
                } else {
                    println!("No password found.");
                }
            },
            "4" => break,
            _ => println!("Invalid choice, try again."),
        }
    }
}
