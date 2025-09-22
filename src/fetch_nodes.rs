use std::str::FromStr;
use std::time::Duration;

use actix_rt;
use chrono::{FixedOffset, Local};
use cron::Schedule;
use reqwest;
use sqlx::SqlitePool;
use serde::{Deserialize};
use serde_json::Value;

use crate::constants::{NODES_API_URL};

#[derive(Debug, Deserialize)]
struct ApiNode {
    #[serde(rename = "publicKey")]
    public_key: String,
    alias: String,
    channels: i64,
    capacity: i64,
    #[serde(rename = "firstSeen")]
    first_seen: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,

    city: Option<Value>,
    country: Option<Value>,
    iso_code: Option<String>,
    subdivision: Option<Value>,
}

// Convert Option<Value> -> Option<String> that's valid JSON or NULL
fn to_json_text(value: Option<Value>) -> Option<String> {
    match value {
        None | Some(Value::Null) => None,
        Some(Value::String(string)) => {
            if let Ok(parsed) = serde_json::from_str::<Value>(&string) {
                Some(parsed.to_string())
            } else {
                Some(Value::String(string).to_string())
            }
        }
        Some(other) => Some(other.to_string()),
    }
}

pub fn spawn_fetch_nodes_job(pool: SqlitePool) {
    actix_rt::spawn(async move {
        // Cron expression for: every minute
        let expression = "0 */1 * * * *";
        let schedule = Schedule::from_str(expression).unwrap();
        let offset = Some(FixedOffset::east_opt(0).unwrap()).unwrap();

        loop {
            let mut upcoming = schedule.upcoming(offset).take(1);
            actix_rt::time::sleep(Duration::from_millis(500)).await;
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {
                    println!("Started importing nodes.");
                    let client = reqwest::Client::builder()
                        .build()
                        .unwrap();

                    let nodes: Vec<ApiNode> = client
                        .get(NODES_API_URL)
                        .send()
                        .await
                        .unwrap()
                        .error_for_status()
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    for n in nodes {
                        let city_json        = to_json_text(n.city);
                        let country_json     = to_json_text(n.country);
                        let subdivision_json = to_json_text(n.subdivision);

                        sqlx::query(
                            r#"
                            INSERT INTO nodes (
                                public_key,
                                alias,
                                channels,
                                capacity,
                                first_seen,
                                updated_at,
                                city,
                                country,
                                iso_code,
                                subdivision
                            )
                            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                            ON CONFLICT(public_key) DO UPDATE SET
                                alias       = excluded.alias,
                                channels    = excluded.channels,
                                capacity    = excluded.capacity,
                                first_seen  = excluded.first_seen,
                                updated_at  = excluded.updated_at,
                                city        = excluded.city,
                                country     = excluded.country,
                                iso_code    = excluded.iso_code,
                                subdivision = excluded.subdivision
                            WHERE excluded.updated_at > nodes.updated_at
                            "#
                        )
                        .bind(n.public_key)
                        .bind(n.alias)
                        .bind(n.channels)
                        .bind(n.capacity)
                        .bind(n.first_seen)
                        .bind(n.updated_at)
                        .bind(city_json)
                        .bind(country_json)
                        .bind(n.iso_code)
                        .bind(subdivision_json)
                        .execute(&pool)
                        .await
                        .unwrap();
                    }

                    println!("Finished importing nodes.");
                }
            }
        }
    });
}
