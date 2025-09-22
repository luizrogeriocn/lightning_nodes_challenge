use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{FromRow, SqlitePool};
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use chrono::{DateTime};

mod constants;
mod database;
mod fetch_nodes;

fn sats_to_btc(sats: i64) -> Decimal {
    Decimal::from(sats) / Decimal::from(100_000_000u64)
}

fn timestamp_from_epoch(epoch: i64) -> String {
    DateTime::from_timestamp(epoch, 0)
        .expect("invalid timestamp")
        .to_string()
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct Node {
    public_key: String,
    alias: String,
    channels: i64,
    capacity: i64,
    first_seen: i64,
    updated_at: i64,
    city: String,
    country: String,
    iso_code: String,
    subdivision: String,
}

#[derive(Serialize)]
struct NodeResponse {
    public_key: String,
    alias: String,
    capacity: Decimal,
    first_seen: String,
}

async fn get_nodes(pool: web::Data<SqlitePool>) -> impl Responder {
    // Get Nodes from the database
    let rows : Vec<Node> = sqlx::query_as("SELECT * FROM nodes;")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    // Transform Nodes into NodeResponses
    let nodes: Vec<NodeResponse> = rows
        .into_iter()
        .map(|row| NodeResponse {
            public_key: row.public_key,
            alias: row.alias,
            capacity: sats_to_btc(row.capacity),
            first_seen: timestamp_from_epoch(row.first_seen),
        })
        .collect();

    HttpResponse::Ok().json(nodes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Database connection
    let pool = database::setup_and_connect().await;

    // Cron job
    fetch_nodes::spawn_fetch_nodes_job(pool.clone());

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/nodes", web::get().to(get_nodes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
