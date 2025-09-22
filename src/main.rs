use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use serde::{Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime};

// Location for the sqlite database file
const DB_URL: &str = "sqlite://sqlite.db";

async fn db() -> SqlitePool {
    // Create the database if it doesn't exist
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists")
    }
    
    // Database connection
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    // Perform database migrations
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    println!("migration: {:?}", migration_results);

    db
}

fn sats_to_btc(sats: i64) -> Decimal {
    Decimal::from(sats) / Decimal::from(100_000_000u64)
}

fn timestamp_from_epoch(epoch: i64) -> String {
    DateTime::from_timestamp(epoch, 0)
        .expect("invalid timestamp")
        .to_string()
}

#[derive(Serialize, FromRow)]
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

async fn get_node_list(pool: web::Data<SqlitePool>) -> impl Responder {
    // get nodes from the database
    let rows : Vec<Node> = sqlx::query_as("SELECT * FROM nodes;")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    // transform nodes
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
    let pool = db().await;

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/nodes", web::get().to(get_node_list))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
