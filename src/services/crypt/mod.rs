use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Key, Nonce};
use base64::{engine::general_purpose, Engine as _};
use dotenvy_macro::dotenv;
use crate::{Failure, ResultCode};

const NONCE_SIZE: usize = 12;

#[derive(Clone)]
pub struct CryptService {
    key: Key<Aes256Gcm>,
}

impl CryptService {
    pub fn new() -> Result<Self, Failure> {
        let key_bytes = dotenv!("ENCRYPTION_KEY")
            .as_bytes()
            .to_vec();

        if key_bytes.len() != 32 {
            return Err(Failure {
                message: "Internal App Error".to_string(),
                trace: "Reason: Encryption key must be 32 bytes".to_string(),
                code: ResultCode::EnvError,
            });
        }

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Ok(Self { key: *key })
    }

    pub fn encrypt(&self, data: &str) -> Result<String, Failure> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = rand::random::<[u8; NONCE_SIZE]>();

        let encrypted = cipher.encrypt(Nonce::from_slice(&nonce), data.as_bytes())
            .map_err(|_| Failure {
                message: "Internal App Error".to_string(),
                trace: "Reason: Encryption failed".to_string(),
                code: ResultCode::CryptoError,
            })?;

        Ok(general_purpose::STANDARD.encode([nonce.to_vec(), encrypted].concat()))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String, Failure> {
        let cipher = Aes256Gcm::new(&self.key);
        let decoded = general_purpose::STANDARD.decode(encrypted_data)
            .map_err(|_| Failure {
                message: "Internal App Error".to_string(),
                trace: "Reason: Base64 decode failed".to_string(),
                code: ResultCode::CryptoError,
            })?;

        let (nonce, ciphertext) = decoded.split_at(NONCE_SIZE);
        let decrypted = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|_| Failure {
                message: "Internal App Error".to_string(),
                trace: "Reason: Decryption failed".to_string(),
                code: ResultCode::CryptoError,
            })?;

        String::from_utf8(decrypted).map_err(|_| Failure {
            message: "Internal App Error".to_string(),
            trace: "Reason: Invalid UTF-8".to_string(),
            code: ResultCode::CryptoError,
        })
    }
}
