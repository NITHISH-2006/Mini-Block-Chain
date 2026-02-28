// ============================================================
// TRANSACTION MODULE  (v2.1 — fixed)
// ------------------------------------------------------------
// FIXES from v2:
//   1. f64 → u64 for amount  (no floating point precision errors)
//   2. Result<> error handling (no unwrap() crashes)
//   3. sign() validates you're using the correct wallet
//
// AMOUNT UNIT — "nits":
//   1 token = 1000 nits (stored as u64 integers)
//   Why? f64: 0.1 + 0.2 = 0.30000000000000004 — WRONG for money
//        u64: 100 + 200 = 300 — always exact
//   Bitcoin calls them "satoshis" (1 BTC = 100,000,000 satoshis)
// ============================================================

use sha2::{Sha256, Digest};
use ed25519_dalek::{VerifyingKey, Signature};
use crate::wallet::{Wallet, verify_signature};

pub const NITS_PER_TOKEN: u64 = 1000;

pub struct Transaction {
    pub from:      String,
    pub to:        String,
    pub amount:    u64,               // stored in nits, NOT tokens
    pub signature: Option<Signature>,
}

impl Transaction {
    /// Create transaction using human-friendly token amount (e.g. 10.5 tokens)
    /// Internally stored as nits: 10.5 → 10500
    pub fn new(from: String, to: String, amount_tokens: f64) -> Self {
        let amount_nits = (amount_tokens * NITS_PER_TOKEN as f64).round() as u64;
        Transaction { from, to, amount: amount_nits, signature: None }
    }

    /// Create directly in nits (used for coinbase/reward transactions)
    pub fn new_nits(from: String, to: String, amount_nits: u64) -> Self {
        Transaction { from, to, amount: amount_nits, signature: None }
    }

    /// Convert internal nits back to human-readable tokens for display
    pub fn amount_as_tokens(&self) -> f64 {
        self.amount as f64 / NITS_PER_TOKEN as f64
    }

    /// The exact bytes that get signed.
    /// We hash (from + to + amount_nits) → 32 fixed bytes.
    /// Hashing first means: change even 1 nit → completely different hash → signature breaks.
    pub fn message_to_sign(&self) -> Vec<u8> {
        let data = format!("{}{}{}", self.from, self.to, self.amount);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    /// FIX: now returns Result<(), String> instead of silently failing.
    /// Also validates: the wallet you're signing with MUST match self.from.
    /// This prevents accidentally authorising someone else's transaction.
    pub fn sign(&mut self, wallet: &Wallet) -> Result<(), String> {
        if self.from != "NETWORK" && wallet.address() != self.from {
            return Err(format!(
                "Wrong wallet — transaction sender is {}... but wallet address is {}...",
                &self.from[..12],
                &wallet.address()[..12]
            ));
        }
        let msg = self.message_to_sign();
        self.signature = Some(wallet.sign(&msg));
        Ok(())
    }

    /// Full validation — returns descriptive error so you know exactly WHY it failed.
    /// Replaces the old is_valid() bool which told you nothing useful on failure.
    pub fn validate(&self) -> Result<(), String> {
        // Rule 1: NETWORK coinbase transactions are exempt from signature rules
        if self.from == "NETWORK" {
            return Ok(());
        }

        // Rule 2: Can't send 0 tokens
        if self.amount == 0 {
            return Err("Transaction amount cannot be zero".to_string());
        }

        // Rule 3: Must have a signature
        let sig = self.signature.as_ref()
            .ok_or_else(|| "Transaction has no signature — call .sign() first".to_string())?;

        // Rule 4: Decode sender's public key from their hex address
        let key_bytes = hex::decode(&self.from)
            .map_err(|_| format!("Cannot decode sender address as hex: {}", &self.from[..12]))?;

        let key_array: [u8; 32] = key_bytes.try_into()
            .map_err(|_| "Sender address has wrong byte length (expected 32)".to_string())?;

        let verifying_key = VerifyingKey::from_bytes(&key_array)
            .map_err(|_| "Sender address is not a valid ed25519 public key".to_string())?;

        // Rule 5: The signature must match this exact transaction data
        let msg = self.message_to_sign();
        if verify_signature(&verifying_key, &msg, sig) {
            Ok(())
        } else {
            Err("Signature is invalid — transaction data may have been tampered with".to_string())
        }
    }

    /// Convenience wrapper — bool for backwards compatibility
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    pub fn display(&self) -> String {
        let from_short = if self.from == "NETWORK" {
            "NETWORK".to_string()
        } else {
            format!("{}...", &self.from[..10])
        };
        let to_short = format!("{}...", &self.to[..10]);
        format!(
            "{} → {} : {} tokens [{}]",
            from_short, to_short,
            self.amount_as_tokens(),
            if self.signature.is_some() { "✅ signed" } else { "❌ unsigned" }
        )
    }
}