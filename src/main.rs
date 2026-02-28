// ============================================================
// MINI BLOCKCHAIN v2.1 — Nithish Chandrasekaran
// ============================================================

mod wallet;
mod transaction;
mod block;
mod blockchain;

use wallet::Wallet;
use transaction::Transaction;
use blockchain::Blockchain;

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║     Mini Blockchain v2.1 — Rust              ║");
    println!("║     Wallets · Transactions · PoW · Mempool   ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // ── WALLETS ───────────────────────────────────────────────
    println!("👛 Generating wallets...");
    let alice = Wallet::new();
    let bob   = Wallet::new();
    let carol = Wallet::new();
    let miner = Wallet::new();

    println!("  Alice : {}...", &alice.address()[..20]);
    println!("  Bob   : {}...", &bob.address()[..20]);
    println!("  Carol : {}...", &carol.address()[..20]);
    println!("  Miner : {}...", &miner.address()[..20]);

    // ── BLOCKCHAIN ────────────────────────────────────────────
    println!();
    let mut bc = Blockchain::new("00"); // "00" = fast for demo, use "0000" for real

    // ── BLOCK 1 TRANSACTIONS ──────────────────────────────────
    println!("\n📝 Preparing block 1 transactions...");

    // Starter grant: NETWORK gives Alice 100 tokens (no signature needed)
    let starter = Transaction::new("NETWORK".to_string(), alice.address(), 100.0);
    handle(bc.add_transaction(starter), "Network → Alice (100 tokens)");

    // Alice → Bob: must sign with Alice's wallet
    let mut t1 = Transaction::new(alice.address(), bob.address(), 30.0);
    handle(t1.sign(&alice), "Alice signs txn");
    handle(bc.add_transaction(t1), "Alice → Bob (30 tokens)");

    // Bob → Carol
    let mut t2 = Transaction::new(bob.address(), carol.address(), 15.0);
    handle(t2.sign(&bob), "Bob signs txn");
    handle(bc.add_transaction(t2), "Bob → Carol (15 tokens)");

    handle(bc.mine_pending_transactions(miner.address()), "Mine block 1");

    // ── BLOCK 2 TRANSACTIONS ──────────────────────────────────
    println!("📝 Preparing block 2 transactions...");

    let mut t3 = Transaction::new(carol.address(), alice.address(), 5.0);
    handle(t3.sign(&carol), "Carol signs txn");
    handle(bc.add_transaction(t3), "Carol → Alice (5 tokens)");

    let mut t4 = Transaction::new(alice.address(), carol.address(), 10.0);
    handle(t4.sign(&alice), "Alice signs txn");
    handle(bc.add_transaction(t4), "Alice → Carol (10 tokens)");

    handle(bc.mine_pending_transactions(miner.address()), "Mine block 2");

    // ── FULL CHAIN ────────────────────────────────────────────
    bc.print_chain();

    // ── BALANCES ──────────────────────────────────────────────
    println!("💰 BALANCES (replayed from genesis):");
    println!("{}", "─".repeat(48));
    print_balance("Alice", bc.get_balance(&alice.address()));
    print_balance("Bob  ", bc.get_balance(&bob.address()));
    print_balance("Carol", bc.get_balance(&carol.address()));
    print_balance("Miner", bc.get_balance(&miner.address()));

    // ── CHAIN VALIDATION (clean) ──────────────────────────────
    println!("\n🔍 VALIDATION:");
    println!("{}", "─".repeat(48));
    println!("  Clean chain valid : {}", bc.is_valid());

    // ── TAMPER ATTACK DEMO ────────────────────────────────────
    // Attacker modifies a transaction amount directly in memory.
    // Two things catch it:
    //   1. Block hash changes (calculate_hash covers all txn data)
    //   2. Signature fails  (signature was over original amount)
    println!("\n⚠️  TAMPER ATTACK: changing Bob's amount to 9999...");
    bc.chain[1].transactions[0].amount = 9_999_000; // 9999 tokens in nits
    println!("  Chain valid after tamper : {}", bc.is_valid());

    // ── WRONG WALLET DEMO ─────────────────────────────────────
    // Bob tries to sign a transaction from Alice's address — caught immediately.
    println!("\n🚨 WRONG WALLET: Bob tries to sign as Alice...");
    let mut fake = Transaction::new(alice.address(), carol.address(), 500.0);
    match fake.sign(&bob) {
        Ok(_)    => println!("  Signed (this should never print)"),
        Err(msg) => println!("  Rejected at signing: {}", msg),
    }

    // ── UNSIGNED TRANSACTION DEMO ─────────────────────────────
    // What if someone skips signing and submits directly?
    println!("\n🚨 UNSIGNED TX: submitting without signing...");
    let unsigned = Transaction::new(alice.address(), bob.address(), 50.0);
    match bc.add_transaction(unsigned) {
        Ok(_)    => println!("  Accepted (this should never print)"),
        Err(msg) => println!("  Rejected at mempool: {}", msg),
    }

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  All demos complete ✅                        ║");
    println!("╚══════════════════════════════════════════════╝");
}

/// Helper: prints Ok/Err result with a label — avoids repeating match blocks
fn handle<E: std::fmt::Display>(result: Result<(), E>, label: &str) {
    match result {
        Ok(_)    => println!("  ✅ {}", label),
        Err(msg) => println!("  ❌ {} FAILED: {}", label, msg),
    }
}

/// Helper: prints balance result cleanly
fn print_balance(name: &str, result: Result<f64, String>) {
    match result {
        Ok(bal)  => println!("  {} : {:.3} tokens", name, bal),
        Err(msg) => println!("  {} : ERROR — {}", name, msg),
    }
}