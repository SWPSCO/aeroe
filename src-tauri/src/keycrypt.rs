// keycrypt.rs

use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;

use argon2::{Argon2, Params, Algorithm, Version};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use rand::rngs::OsRng;
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

/// Magic header to identify our file format/version.
const HEADER_MAGIC: &[u8] = b"79CLOVER"; // 8 bytes

/// Lengths (in bytes) for salt and nonce.
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;

/// `Keycrypt` stores:
/// - `data`: decrypted contents as an array of strings (each line)
/// - `password`: the passphrase used for encryption/decryption
/// - `enc`: path to the encrypted file on disk
pub struct Keycrypt {
    loaded: bool,
    data: Vec<String>,
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
            data: Vec::new(),
            password: String::new(),
            enc,
        }
    }

    pub fn load(&mut self, password: String) -> Result<(), String> {
        self.password = password;
        self.decrypt()?;
        self.loaded = true;
        Ok(())
    }

    /// Encrypts `self.data` (joined by newlines) using `self.password`,
    /// and writes a Base64‐encoded ciphertext file to `self.enc`.
    pub fn write(&self) -> Result<(), String> {
        // 1) Serialize `self.data` to plaintext bytes
        let plaintext = self.data.join("\n");
        let plaintext_bytes = plaintext.as_bytes();

        // 2) Generate a random salt
        let mut salt = [0u8; SALT_LEN];
        let mut rng = OsRng;
        rng.fill_bytes(&mut salt);

        // 3) Derive a 32‐byte key via Argon2id
        //    Params: memory = 19 * 1024 KiB = 19 MiB, iterations = 2, parallelism = 1
        let params = Params::new(19 * 1024, 2, 1, None)
            .map_err(|e| format!("Invalid Argon2 params: {}", e))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut key_bytes = [0u8; 32];
        argon2
            .hash_password_into(self.password.as_bytes(), &salt, &mut key_bytes)
            .map_err(|e| format!("Argon2 key derivation failed: {}", e))?;

        // 4) Initialize XChaCha20‐Poly1305 and generate a random nonce
        let cipher = XChaCha20Poly1305::new(Key::from_slice(&key_bytes));
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rng.fill_bytes(&mut nonce_bytes);
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

    /// Wipes (clears) the in‐memory plaintext data.
    pub fn wipe(&mut self) {
        self.data.clear();
    }

    /// Reads the Base64‐encoded ciphertext from `self.enc`, decrypts it using `self.password`,
    /// and loads the resulting plaintext (split on `\n`) into `self.data`.
    pub fn decrypt(&mut self) -> Result<(), String> {
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
        let params = Params::new(19 * 1024, 2, 1, None)
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
        self.data = plaintext_str
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(())
    }
}
