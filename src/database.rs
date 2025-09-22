use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use crate::constants::DB_URL;

pub async fn setup_and_connect() -> SqlitePool {
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
