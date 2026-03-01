mod wallet;
mod transaction;
mod block;
mod blockchain;
mod api;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(api::AppState {
        blockchain: Mutex::new(blockchain::Blockchain::new("00")),
    });

    // Railway injects PORT as an environment variable
    // locally it falls back to 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("running on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/wallet/new",        web::get().to(api::new_wallet))
            .route("/transaction",       web::post().to(api::submit_transaction))
            .route("/mine",              web::post().to(api::mine_block))
            .route("/chain",             web::get().to(api::get_chain))
            .route("/balance/{address}", web::get().to(api::get_balance))
            .route("/validate",          web::get().to(api::validate_chain))
    })
    .bind(&addr)?
    .run()
    .await
}