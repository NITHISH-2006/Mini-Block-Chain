use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct AppState {
    pub blockchain: Mutex<Blockchain>,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub from:            String,
    pub to:              String,
    pub amount:          f64,
    pub private_key_hex: String,
}

#[derive(Deserialize)]
pub struct MineRequest {
    pub miner_address: String,
}

// every endpoint returns { ok, message, data }
#[derive(Serialize)]
pub struct Res<T: Serialize> {
    pub ok:      bool,
    pub message: String,
    pub data:    Option<T>,
}

fn ok<T: Serialize>(msg: &str, data: T) -> HttpResponse {
    HttpResponse::Ok().json(Res { ok: true, message: msg.into(), data: Some(data) })
}

fn err(msg: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(Res::<()> { ok: false, message: msg.into(), data: None })
}

// GET /wallet/new
pub async fn new_wallet() -> impl Responder {
    let w = Wallet::new();
    ok("wallet created", w.to_info())
}

// POST /transaction
// { from, to, amount, private_key_hex }
pub async fn submit_transaction(
    state: web::Data<AppState>,
    body: web::Json<TransactionRequest>,
) -> impl Responder {
    let wallet = match Wallet::from_private_key_hex(&body.private_key_hex) {
        Ok(w) => w,
        Err(e) => return err(&e),
    };

    if wallet.address() != body.from {
        return err("private key doesn't match the from address");
    }

    let mut txn = Transaction::new(body.from.clone(), body.to.clone(), body.amount);
    if let Err(e) = txn.sign(&wallet) {
        return err(&e);
    }

    let mut bc = state.blockchain.lock().unwrap();
    match bc.add_transaction(txn) {
        Ok(_)  => ok("transaction added to mempool", body.amount),
        Err(e) => err(&e),
    }
}

// POST /mine
// { miner_address }
pub async fn mine_block(
    state: web::Data<AppState>,
    body: web::Json<MineRequest>,
) -> impl Responder {
    let mut bc = state.blockchain.lock().unwrap();
    match bc.mine_pending_transactions(body.miner_address.clone()) {
        Ok(_)  => ok("block mined", bc.chain.len() - 1),
        Err(e) => err(&e),
    }
}

// GET /chain
pub async fn get_chain(state: web::Data<AppState>) -> impl Responder {
    let bc = state.blockchain.lock().unwrap();
    ok("here's the chain", &bc.chain)
}

// GET /balance/:address
pub async fn get_balance(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let address = path.into_inner();
    let bc = state.blockchain.lock().unwrap();
    match bc.get_balance(&address) {
        Ok(bal) => ok("balance fetched", bal),
        Err(e)  => err(&e),
    }
}

// GET /validate
pub async fn validate_chain(state: web::Data<AppState>) -> impl Responder {
    let bc = state.blockchain.lock().unwrap();
    match bc.validate() {
        Ok(_)  => ok("chain is valid", true),
        Err(e) => ok(&e, false),
    }
}