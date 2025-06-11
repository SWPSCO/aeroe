// keycrypt.rs

use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::collections::HashMap;

use argon2::{Argon2, Params, Algorithm, Version};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use rand::rngs::OsRng;
use rand_core::TryRngCore;
use base64::{engine::general_purpose, Engine as _};

/// Magic header to identify our file format/version.
const HEADER_MAGIC: &[u8] = b"79CLOVER"; // 8 bytes

/// Lengths (in bytes) for salt and nonce.
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;

#[derive(Debug)]
pub struct Keycrypt {
    loaded: bool,
    data: HashMap<String, String>,
    password: String,
    enc: PathBuf,
}

impl Keycrypt {
    /// Creates a new `Keycrypt` instance with:
    /// - `enc_path`: PathBuf to the encrypted file
    /// - `password`: passphrase to use for future encrypt/decrypt operations
    /// The `data` vector starts empty.
    pub fn new(enc: PathBuf) -> Self {
        Keycrypt {
            loaded: false,
            data: HashMap::new(),
            password: String::new(),
            enc,
        }
    }

    /// Returns true if the encrypted file exists.
    pub fn vault_exists(&self) -> bool {
        self.enc.exists() && self.enc.is_file()
    }

    /// Returns true if the encrypted file has been loaded and decrypted.
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn create(&mut self, password: String) -> Result<(), String> {
        self.password = password;
        self.write()?;
        self.loaded = true;
        tracing::debug!("create: {:?}", self);
        Ok(())
    }

    /// Loads the encrypted file and decrypts it using the provided password.
    pub fn load(&mut self, password: String) -> Result<(), String> {
        self.password = password;
        self.decrypt()?;
        self.loaded = true;
        tracing::debug!("load: {:?}", self);
        Ok(())
    }

    /// Returns a list of all wallet names.
    pub fn get_wallets(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
    pub fn get_seedphrase(&self, wallet_name: String) -> Result<String, String> {
        if !self.loaded {
            return Err("Vault not loaded".to_string());
        }
        if !self.data.contains_key(&wallet_name) {
            return Err(format!("Wallet {} not found", wallet_name));
        }
        Ok(self.data.get(&wallet_name).unwrap().clone())
    }

    pub fn add_wallet(&mut self, wallet_name: String, seedphrase: String) -> Result<(), String> {
        if !self.loaded {
            return Err("Vault not loaded".to_string());
        }
        if self.data.contains_key(&wallet_name) {
            return Err(format!("Wallet {} already exists", wallet_name));
        }
        self.data.insert(wallet_name, seedphrase);
        self.write()?;
        tracing::debug!("add_wallet: {:?}", self);
        Ok(())
    }

    /// Encrypts `self.data` (joined by newlines) using `self.password`,
    /// and writes a Base64‐encoded ciphertext file to `self.enc`.
    fn write(&self) -> Result<(), String> {
        // 1) Serialize `self.data` to plaintext bytes
        // We'll use a simple format: key\tvalue per line
        let mut lines = Vec::new();
        for (k, v) in &self.data {
            // Escape tabs and newlines in keys/values for safety
            let k = k.replace('\t', "\\t").replace('\n', "\\n");
            let v = v.replace('\t', "\\t").replace('\n', "\\n");
            lines.push(format!("{}\t{}", k, v));
        }
        let plaintext = lines.join("\n");
        let plaintext_bytes = plaintext.as_bytes();

        // 2) Generate a random salt
        let mut salt = [0u8; SALT_LEN];
        OsRng.try_fill_bytes(&mut salt).map_err(|e| format!("Failed to fill salt: {e}"))?;

        // 3) Derive a 32‐byte key via Argon2id
        //    Params: memory = 1024 * 1024 KiB = 1024 MiB, iterations = 8, parallelism = 1
        let params = Params::new(1024 * 1024, 8, 1, None)
            .map_err(|e| format!("Invalid Argon2 params: {}", e))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut key_bytes = [0u8; 32];
        argon2
            .hash_password_into(self.password.as_bytes(), &salt, &mut key_bytes)
            .map_err(|e| format!("Argon2 key derivation failed: {}", e))?;

        // 4) Initialize XChaCha20‐Poly1305 and generate a random nonce
        let cipher = XChaCha20Poly1305::new(Key::from_slice(&key_bytes));
        let mut nonce_bytes = [0u8; NONCE_LEN];
        OsRng.try_fill_bytes(&mut nonce_bytes).map_err(|e| format!("Failed to fill nonce: {e}"))?;
        let nonce = XNonce::from_slice(&nonce_bytes);

        // 5) Encrypt + authenticate
        let ciphertext = cipher
            .encrypt(nonce, plaintext_bytes)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // 6) Build the payload: HEADER_MAGIC ∥ salt ∥ nonce ∥ ciphertext
        let mut payload = Vec::new();
        payload.extend_from_slice(HEADER_MAGIC);
        payload.extend_from_slice(&salt);
        payload.extend_from_slice(&nonce_bytes);
        payload.extend_from_slice(&ciphertext);

        // 7) Base64‐encode and write to disk
        let b64_payload = general_purpose::STANDARD.encode(&payload);
        fs::write(&self.enc, b64_payload)
            .map_err(|e| format!("Failed to write encrypted file: {}", e))?;

        Ok(())
    }
    /// Reads the Base64‐encoded ciphertext from `self.enc`, decrypts it using `self.password`,
    /// and loads the resulting plaintext (split on `\n`) into `self.data`.
    fn decrypt(&mut self) -> Result<(), String> {
        // 1) Read Base64 string from disk
        let b64_payload = fs::read_to_string(&self.enc)
            .map_err(|e| format!("Failed to read encrypted file: {}", e))?;

        // 2) Decode Base64 → raw payload
        let payload = general_purpose::STANDARD.decode(&b64_payload)
            .map_err(|e| format!("Base64 decoding failed: {}", e))?;

        // Use a Cursor to parse out header, salt, nonce, and ciphertext
        let mut cursor = Cursor::new(&payload);

        // 3a) Validate header magic
        let mut header = [0u8; HEADER_MAGIC.len()];
        cursor
            .read_exact(&mut header)
            .map_err(|_| format!("File too short or corrupted (missing header)"))?;
        if header != HEADER_MAGIC {
            return Err(format!("Invalid file format or wrong version"));
        }

        // 3b) Extract salt
        let mut salt = [0u8; SALT_LEN];
        cursor
            .read_exact(&mut salt)
            .map_err(|_| format!("File too short or corrupted (missing salt)"))?;

        // 3c) Extract nonce
        let mut nonce_bytes = [0u8; NONCE_LEN];
        cursor
            .read_exact(&mut nonce_bytes)
            .map_err(|_| format!("File too short or corrupted (missing nonce)"))?;
        let nonce = XNonce::from_slice(&nonce_bytes);

        // 3d) The remainder is ciphertext (+ 16-byte auth tag)
        let mut ciphertext = Vec::new();
        cursor
            .read_to_end(&mut ciphertext)
            .map_err(|e| format!("Failed to read ciphertext bytes: {}", e))?;

        // 4) Re‐derive the 32‐byte key via Argon2id
        let params = Params::new(1024 * 1024, 8, 1, None)
            .map_err(|e| format!("Invalid Argon2 params: {}", e))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut key_bytes = [0u8; 32];
        argon2
            .hash_password_into(self.password.as_bytes(), &salt, &mut key_bytes)
            .map_err(|e| format!("Argon2 key derivation failed: {}", e))?;

        // 5) Decrypt + verify
        let cipher = XChaCha20Poly1305::new(Key::from_slice(&key_bytes));
        let plaintext_bytes = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| format!("Decryption failed: wrong password or corrupted file"))?;

        // 6) Convert plaintext to UTF‐8 and split lines
        let plaintext_str = String::from_utf8(plaintext_bytes)
            .map_err(|e| format!("Decrypted data is not valid UTF-8: {}", e))?;
        let mut map = HashMap::new();
        for line in plaintext_str.lines() {
            if let Some((k, v)) = line.split_once('\t') {
                // Unescape tabs and newlines
                let k = k.replace("\\t", "\t").replace("\\n", "\n");
                let v = v.replace("\\t", "\t").replace("\\n", "\n");
                map.insert(k, v);
            }
        }
        self.data = map;
        Ok(())
    }
}
