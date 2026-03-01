// Transaction — signed transfer of tokens between two addresses.
// Amounts stored as u64 "nits" (1 token = 1000 nits) to avoid f64 precision errors.
// Signature stored as hex string so it can be serialized to JSON.

use sha2::{Sha256, Digest};
use ed25519_dalek::{VerifyingKey, Signature};
use serde::{Serialize, Deserialize};
use crate::wallet::{Wallet, verify_signature};

pub const NITS_PER_TOKEN: u64 = 1000;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from:          String,
    pub to:            String,
    pub amount:        u64,             // in nits
    pub signature_hex: Option<String>,  // hex string — serializes cleanly to JSON
}

impl Transaction {
    pub fn new(from: String, to: String, amount_tokens: f64) -> Self {
        let amount_nits = (amount_tokens * NITS_PER_TOKEN as f64).round() as u64;
        Transaction { from, to, amount: amount_nits, signature_hex: None }
    }

    pub fn new_nits(from: String, to: String, amount_nits: u64) -> Self {
        Transaction { from, to, amount: amount_nits, signature_hex: None }
    }

    pub fn amount_as_tokens(&self) -> f64 {
        self.amount as f64 / NITS_PER_TOKEN as f64
    }

    /// The exact bytes we sign — hash of (from + to + amount_nits).
    /// Hashing first gives fixed 32 bytes regardless of address length.
    pub fn message_to_sign(&self) -> Vec<u8> {
        let data = format!("{}{}{}", self.from, self.to, self.amount);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    /// Sign with sender's wallet. Validates wallet matches self.from.
    pub fn sign(&mut self, wallet: &Wallet) -> Result<(), String> {
        if self.from != "NETWORK" && wallet.address() != self.from {
            return Err(format!(
                "Wrong wallet — sender is {}... but wallet is {}...",
                &self.from[..12], &wallet.address()[..12]
            ));
        }
        let msg = self.message_to_sign();
        let sig: Signature = wallet.sign(&msg);
        // Store as hex string so JSON serialization works cleanly
        self.signature_hex = Some(hex::encode(sig.to_bytes()));
        Ok(())
    }

    /// Full validation — returns descriptive Err so caller knows exactly why it failed.
    pub fn validate(&self) -> Result<(), String> {
        if self.from == "NETWORK" { return Ok(()); }

        if self.amount == 0 {
            return Err("Amount cannot be zero".to_string());
        }

        let sig_hex = self.signature_hex.as_ref()
            .ok_or("Transaction is unsigned — call sign() first")?;

        // Decode signature from hex back to bytes
        let sig_bytes = hex::decode(sig_hex)
            .map_err(|_| "Signature is not valid hex".to_string())?;
        let sig_array: [u8; 64] = sig_bytes.try_into()
            .map_err(|_| "Signature has wrong byte length".to_string())?;
        let signature = Signature::from_bytes(&sig_array);

        // Decode sender's public key from their address (address IS the public key)
        let key_bytes = hex::decode(&self.from)
            .map_err(|_| "Sender address is not valid hex".to_string())?;
        let key_array: [u8; 32] = key_bytes.try_into()
            .map_err(|_| "Sender address has wrong byte length".to_string())?;
        let verifying_key = VerifyingKey::from_bytes(&key_array)
            .map_err(|_| "Sender address is not a valid ed25519 public key".to_string())?;

        let msg = self.message_to_sign();
        if verify_signature(&verifying_key, &msg, &signature) {
            Ok(())
        } else {
            Err("Signature invalid — transaction may have been tampered".to_string())
        }
    }

    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool { self.validate().is_ok() }

    pub fn display(&self) -> String {
        let from_short = if self.from == "NETWORK" { "NETWORK".to_string() }
                         else { format!("{}...", &self.from[..10]) };
        let to_short = format!("{}...", &self.to[..10]);
        format!("{} → {} : {} tokens [{}]",
            from_short, to_short, self.amount_as_tokens(),
            if self.signature_hex.is_some() { "✅ signed" } else { "❌ unsigned" })
    }
}