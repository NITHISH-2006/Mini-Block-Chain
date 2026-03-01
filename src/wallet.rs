use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

// WalletInfo is what we send over the API — just the addresses, never the private key
// The real Wallet struct holds the signing key (secret) and never gets serialized
#[derive(Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,         // public key as hex — safe to share
    pub private_key_hex: String, // hex of private key — shown ONCE on creation, never stored
}

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    /// Generates a brand new random wallet.
    /// OsRng = OS-level cryptographic randomness (safe for key generation).
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Wallet { signing_key, verifying_key }
    }

    /// Recreate a wallet from a private key hex string.
    /// Used when the API receives a private key to sign a transaction.
    pub fn from_private_key_hex(hex_str: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex_str)
            .map_err(|_| "Invalid private key hex".to_string())?;
        let key_array: [u8; 32] = bytes.try_into()
            .map_err(|_| "Private key must be 32 bytes".to_string())?;
        let signing_key = SigningKey::from_bytes(&key_array);
        let verifying_key = signing_key.verifying_key();
        Ok(Wallet { signing_key, verifying_key })
    }

    pub fn address(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    pub fn private_key_hex(&self) -> String {
        hex::encode(self.signing_key.as_bytes())
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    /// Returns a WalletInfo struct safe to serialize and send as JSON
    pub fn to_info(&self) -> WalletInfo {
        WalletInfo {
            address: self.address(),
            private_key_hex: self.private_key_hex(),
        }
    }
}

pub fn verify_signature(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    verifying_key.verify(message, signature).is_ok()
}