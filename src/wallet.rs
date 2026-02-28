// ============================================================
// WALLET MODULE
// ------------------------------------------------------------
// A wallet = a cryptographic keypair:
//   SigningKey   = private key (secret — used to SIGN)
//   VerifyingKey = public key  (your address — used to VERIFY)
//
// The public key is mathematically derived from the private key.
// You can go private → public, but NEVER public → private.
// That one-way property is the foundation of blockchain security.
// ============================================================

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    /// Generates a brand new random wallet.
    /// OsRng = OS-level cryptographic randomness (safe for key generation).
    /// Regular rand is NOT safe for cryptographic keys.
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Wallet { signing_key, verifying_key }
    }

    /// Your blockchain "address" — the public key encoded as a hex string.
    /// Like 0x1234abcd... in Ethereum. Shareable with anyone.
    pub fn address(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    /// Signs a byte slice with your private key.
    /// The returned Signature is cryptographic proof you authorized this data.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
}

/// Standalone verify — anyone can call this without owning the wallet.
/// Returns true ONLY if the signature was made by the matching private key.
pub fn verify_signature(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    verifying_key.verify(message, signature).is_ok()
}