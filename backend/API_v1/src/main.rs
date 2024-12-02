use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use influxdb::Client;
use chrono::{Utc, DateTime};
use futures::stream::TryStreamExt;

#[derive(Deserialize)]
struct SensorData {
    nodo: String,
    temperature: f32,
    humidity: f32,
    pir0: bool,
    pir1: bool,
    pir2: bool,
}

#[derive(Deserialize)]
struct RelayData {
    nodo: String,
    relay: u8,
    state: bool,
}

#[derive(Deserialize)]
struct PirData {
    nodo: String,
    pir: u8,
    state: bool,
}

#[derive(Serialize)]
struct SensorRecord {
    nodo: String,
    temperature: f32,
    humidity: f32,
    pir0: bool,
    pir1: bool,
    pir2: bool,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
struct RelayRecord {
    nodo: String,
    relay: u8,
    state: bool,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
struct PirRecord {
    nodo: String,
    pir: u8,
    state: bool,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
struct NodeMetadata {
    nodo: String,
    description: String,
    location: String,
}

async fn save_sensor_data(data: web::Json<SensorData>) -> impl Responder {
    let client = Client::new("http://localhost:8086", "domotica");
    let mut query = client.query("sensors");
    query.add_tag("nodo", &data.nodo);
    query.add_field("temperature", data.temperature as f64);
    query.add_field("humidity", data.humidity as f64);
    query.add_field("pir0", data.pir0 as i64);
    query.add_field("pir1", data.pir1 as i64);
    query.add_field("pir2", data.pir2 as i64);
    query.write().await.unwrap();
    "Data saved"
}

async fn save_relay_data(data: web::Json<RelayData>) -> impl Responder {
    let client = Client::new("http://localhost:8086", "domotica");
    let mut query = client.query("reles");
    query.add_tag("nodo", &data.nodo);
    query.add_field("relay", data.relay as i64);
    query.add_field("state", data.state as i64);
    query.write().await.unwrap();
    "Relay state saved"
}

async fn save_pir_data(data: web::Json<PirData>) -> impl Responder {
    let client = Client::new("http://localhost:8086", "domotica");
    let mut query = client.query("pir");
    query.add_tag("nodo", &data.nodo);
    query.add_field("pir", data.pir as i64);
    query.add_field("state", data.state as i64);
    query.write().await.unwrap();
    "PIR state saved"
}

async fn get_data(measurement: &str, nodo: web::Path<String>, period: web::Path<String>) -> impl Responder {
    let client = Client::new("http://localhost:8086", "domotica");
    let query = match period.as_str() {
        "day" => format!("SELECT * FROM {} WHERE nodo = '{}' AND time >= now() - 1d", measurement, nodo),
        "week" => format!("SELECT * FROM {} WHERE nodo = '{}' AND time >= now() - 7d", measurement, nodo),
        "month" => format!("SELECT * FROM {} WHERE nodo = '{}' AND time >= now() - 30d", measurement, nodo),
        _ => format!("SELECT * FROM {} WHERE nodo = '{}'", measurement, nodo),
    };

    let mut results = Vec::new();
    let mut query_result = client.query(query).await.unwrap();

    while let Some(result) = query_result.next().await {
        let record = match measurement {
            "sensors" => SensorRecord {
                nodo: result.get_tag("nodo").unwrap().to_string(),
                temperature: result.get_field("temperature").unwrap() as f32,
                humidity: result.get_field("humidity").unwrap() as f32,
                pir0: result.get_field("pir0").unwrap() != 0,
                pir1: result.get_field("pir1").unwrap() != 0,
                pir2: result.get_field("pir2").unwrap() != 0,
                timestamp: result.get_timestamp().unwrap(),
            },
            "reles" => RelayRecord {
                nodo: result.get_tag("nodo").unwrap().to_string(),
                relay: result.get_field("relay").unwrap() as u8,
                state: result.get_field("state").unwrap() != 0,
                timestamp: result.get_timestamp().unwrap(),
            },
            "pir" => PirRecord {
                nodo: result.get_tag("nodo").unwrap().to_string(),
                pir: result.get_field("pir").unwrap() as u8,
                state: result.get_field("state").unwrap() != 0,
                timestamp: result.get_timestamp().unwrap(),
            },
            _ => continue,
        };
        results.push(record);
    }

    HttpResponse::Ok().json(results)
}

async fn get_node_metadata() -> impl Responder {
    let client = Client::new("http://localhost:8086", "domotica");
    let query = "SELECT * FROM nodos";
    let mut results = Vec::new();
    let mut query_result = client.query(query).await.unwrap();

    while let Some(result) = query_result.next().await {
        let metadata = NodeMetadata {
            nodo: result.get_tag("nodo").unwrap().to_string(),
            description: result.get_field("description").unwrap().to_string(),
            location: result.get_field("location").unwrap().to_string(),
        };
        results.push(metadata);
    }

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sensors", web::post().to(save_sensor_data))
            .route("/reles", web::post().to(save_relay_data))
            .route("/pir", web::post().to(save_pir_data))
            .route("/sensors/{nodo}/{period}", web::get().to(get_data))
            .route("/reles/{nodo}/{period}", web::get().to(get_data))
            .route("/pir/{nodo}/{period}", web::get().to(get_data))
            .route("/nodos", web::get().to(get_node_metadata))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
