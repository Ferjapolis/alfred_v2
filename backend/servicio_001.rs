use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions, bson::doc};
use chrono::{NaiveDate, Utc};
use futures::stream::TryStreamExt;

#[derive(Deserialize)]
struct SensorData {
    nodo: String,
    temperature: f32,
    humidity: f32,
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
    timestamp: NaiveDate,
}

#[derive(Serialize)]
struct RelayRecord {
    nodo: String,
    relay: u8,
    state: bool,
    timestamp: NaiveDate,
}

#[derive(Serialize)]
struct PirRecord {
    nodo: String,
    pir: u8,
    state: bool,
    timestamp: NaiveDate,
}

async fn save_sensor_data(data: web::Json<SensorData>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("domotica").collection("sensors");

    let doc = doc! {
        "nodo": &data.nodo,
        "temperature": data.temperature,
        "humidity": data.humidity,
        "timestamp": Utc::now(),
    };

    collection.insert_one(doc, None).await.unwrap();
    "Data saved"
}

async fn save_relay_data(data: web::Json<RelayData>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("domotica").collection("reles");

    let doc = doc! {
        "nodo": &data.nodo,
        "relay": data.relay,
        "state": data.state,
        "timestamp": Utc::now(),
    };

    collection.insert_one(doc, None).await.unwrap();
    "Relay state saved"
}

async fn save_pir_data(data: web::Json<PirData>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("domotica").collection("pir");

    let doc = doc! {
        "nodo": &data.nodo,
        "pir": data.pir,
        "state": data.state,
        "timestamp": Utc::now(),
    };

    collection.insert_one(doc, None).await.unwrap();
    "PIR state saved"
}

async fn get_data(collection: &str, table: web::Path<String>, period: web::Path<String>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("domotica").collection(collection);

    let now = Utc::now();
    let filter = match period.as_str() {
        "day" => doc! {
            "timestamp": {
                "$gte": now.date_naive()
            }
        },
        "week" => doc! {
            "timestamp": {
                "$gte": (now - chrono::Duration::weeks(1)).date_naive()
            }
        },
        "month" => doc! {
            "timestamp": {
                "$gte": (now - chrono::Duration::days(30)).date_naive()
            }
        },
        _ => doc! {},
    };

    let mut cursor = collection.find(filter, None).await.unwrap();
    let mut results = Vec::new();

    while let Some(result) = cursor.try_next().await.unwrap() {
        let record = match collection.name() {
            "sensors" => SensorRecord {
                nodo: result.get_str("nodo").unwrap().to_string(),
                temperature: result.get_f64("temperature").unwrap() as f32,
                humidity: result.get_f64("humidity").unwrap() as f32,
                timestamp: result.get_date("timestamp").unwrap(),
            },
            "reles" => RelayRecord {
                nodo: result.get_str("nodo").unwrap().to_string(),
                relay: result.get_u32("relay").unwrap() as u8,
                state: result.get_bool("state").unwrap(),
                timestamp: result.get_date("timestamp").unwrap(),
            },
            "pir" => PirRecord {
                nodo: result.get_str("nodo").unwrap().to_string(),
                pir: result.get_u32("pir").unwrap() as u8,
                state: result.get_bool("state").unwrap(),
                timestamp: result.get_date("timestamp").unwrap(),
            },
            _ => continue,
        };
        results.push(record);
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
            .route("/{table}/{period}", web::get().to(get_data))
            .route("/{table}/{period}", web::get().to(get_data))
            .route("/{table}/{period}", web::get().to(get_data))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
