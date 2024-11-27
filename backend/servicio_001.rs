use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions};

#[derive(Deserialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
    pir0: bool,
    pir1: bool,
    pir2: bool,
}

async fn save_sensor_data(data: web::Json<SensorData>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("alfred").collection("sensores");

    let doc = bson::doc! {
        "temperature": data.temperature,
        "humidity": data.humidity,
        "pir0": data.pir0,
        "pir1": data.pir1,
        "pir2": data.pir2,
    };

    collection.insert_one(doc, None).await.unwrap();
    "Data saved"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sensores", web::post().to(save_sensor_data))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
